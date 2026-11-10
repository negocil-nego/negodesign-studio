use crate::{
    model::project::{ComponentExport, ComponentFile, ComponentInstallResult, Project},
    repository::project::{
        db::{map_project, open_database},
        detect::detect_language,
        installer::{
            build_index, validate_export_name, validate_relative_path, write_project_files,
        },
    },
};
use rusqlite::{OptionalExtension, params};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tauri::AppHandle;

pub fn list_projects(app: &AppHandle) -> Result<Vec<Project>, String> {
    let connection = open_database(app)?;
    let mut statement = connection
        .prepare("SELECT id, name, path, language, active FROM TB_PROJECTS ORDER BY active DESC, id DESC")
        .map_err(|e| e.to_string())?;
    let mut projects = statement
        .query_map([], map_project)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    drop(statement);

    for project in &mut projects {
        if project.language == "Unknown" {
            let language = detect_language(&project.path);
            if language != "Unknown" {
                connection
                    .execute(
                        "UPDATE TB_PROJECTS SET language = ?1 WHERE id = ?2",
                        params![language, project.id],
                    )
                    .map_err(|e| e.to_string())?;
                project.language = language;
            }
        }
    }

    Ok(projects)
}

pub fn list_active_project_components(app: &AppHandle) -> Result<Vec<ComponentExport>, String> {
    let connection = open_database(app)?;
    let project_id = connection
        .query_row(
            "SELECT id FROM TB_PROJECTS WHERE active = 1 LIMIT 1",
            [],
            |row| row.get::<_, i64>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let Some(project_id) = project_id else {
        return Ok(Vec::new());
    };

    let mut statement = connection
        .prepare(
            "SELECT component_id, name, entry FROM TB_PROJECT_COMPONENTS
             WHERE project_id = ?1 ORDER BY entry, name",
        )
        .map_err(|e| e.to_string())?;
    statement
        .query_map([project_id], |row| {
            Ok(ComponentExport {
                id: row.get(0)?,
                name: row.get(1)?,
                entry: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

pub fn save_project(app: &AppHandle, path: &str) -> Result<Project, String> {
    let name = Path::new(path)
        .file_name()
        .and_then(|f| f.to_str())
        .filter(|f| !f.is_empty())
        .unwrap_or(path)
        .to_string();
    let language = detect_language(path);
    let mut connection = open_database(app)?;
    let tx = connection.transaction().map_err(|e| e.to_string())?;

    tx.execute("UPDATE TB_PROJECTS SET active = 0", [])
        .map_err(|e| e.to_string())?;
    tx.execute(
        "INSERT INTO TB_PROJECTS (name, path, language, active) VALUES (?1, ?2, ?3, 1)
         ON CONFLICT(path) DO UPDATE SET
           name = excluded.name,
           language = excluded.language,
           active = 1",
        params![name, path, language],
    )
    .map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())?;

    connection
        .query_row(
            "SELECT id, name, path, language, active FROM TB_PROJECTS WHERE path = ?1",
            [path],
            map_project,
        )
        .map_err(|e| e.to_string())
}

pub fn set_active_project(app: &AppHandle, id: i64) -> Result<Project, String> {
    let mut connection = open_database(app)?;
    let tx = connection.transaction().map_err(|e| e.to_string())?;
    tx.execute("UPDATE TB_PROJECTS SET active = 0", [])
        .map_err(|e| e.to_string())?;
    let changed = tx
        .execute("UPDATE TB_PROJECTS SET active = 1 WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    if changed == 0 {
        return Err("O projecto seleccionado já não existe.".to_string());
    }
    tx.commit().map_err(|e| e.to_string())?;

    connection
        .query_row(
            "SELECT id, name, path, language, active FROM TB_PROJECTS WHERE id = ?1",
            [id],
            map_project,
        )
        .map_err(|e| e.to_string())
}

pub fn install_component(
    app: &AppHandle,
    files: Vec<ComponentFile>,
    components: Vec<ComponentExport>,
) -> Result<ComponentInstallResult, String> {
    if files.is_empty() || components.is_empty() {
        return Err("Não foram encontrados ficheiros ou componentes para instalar.".to_string());
    }
    let mut connection = open_database(app)?;
    let (project_id, project_path, language): (i64, String, String) = connection
        .query_row(
            "SELECT id, path, language FROM TB_PROJECTS WHERE active = 1 LIMIT 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .map_err(|_| "Seleccione primeiro um projecto activo.".to_string())?;
    if !["Svelte", "React", "Vue", "Nextjs", "Nuxt", "Flutter"].contains(&language.as_str()) {
        return Err("A linguagem do projecto não foi identificada.".to_string());
    }

    let project_root = PathBuf::from(project_path);
    let copied = write_project_files(&project_root, &language, files)?;

    let tx = connection.transaction().map_err(|e| e.to_string())?;
    {
        let mut insert_stmt = tx
            .prepare(
                "INSERT INTO TB_PROJECT_COMPONENTS (project_id, component_id, name, entry, language)
                 VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(project_id, component_id) DO UPDATE SET
                   name = excluded.name,
                   entry = excluded.entry,
                   language = excluded.language",
            )
            .map_err(|e| e.to_string())?;
        for component in &components {
            validate_export_name(&component.name)?;
            validate_relative_path(&component.entry)?;
            insert_stmt
                .execute(params![
                    project_id,
                    component.id,
                    component.name,
                    component.entry,
                    language
                ])
                .map_err(|e| e.to_string())?;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;

    let mut statement = connection
        .prepare(
            "SELECT component_id, name, entry FROM TB_PROJECT_COMPONENTS
             WHERE project_id = ?1 AND language = ?2 ORDER BY name",
        )
        .map_err(|e| e.to_string())?;
    let mut installed = statement
        .query_map(params![project_id, language], |row| {
            Ok(ComponentExport {
                id: row.get(0)?,
                name: row.get(1)?,
                entry: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    drop(statement);

    let package_root = project_root.join("negodesign");
    let index = build_index(&language, &mut installed)?;
    let index_name = if language == "Flutter" {
        "index.dart"
    } else {
        "index.ts"
    };
    fs::create_dir_all(&package_root).map_err(|e| e.to_string())?;
    fs::write(package_root.join(index_name), index).map_err(|e| e.to_string())?;

    Ok(ComponentInstallResult {
        copied_files: copied,
        exported_components: installed.len(),
    })
}
