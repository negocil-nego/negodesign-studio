use crate::model::project::{ComponentExport, ComponentFile, ComponentInstallResult, Project};
use rusqlite::{Connection, OptionalExtension, params};
use std::{
    fs,
    path::{Component, Path, PathBuf},
};
use tauri::{AppHandle, Manager};

fn open_database(app: &AppHandle) -> Result<Connection, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(&app_data_dir).map_err(|error| error.to_string())?;

    let connection =
        Connection::open(app_data_dir.join("negodesign.db")).map_err(|error| error.to_string())?;
    connection
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS TB_PROJECTS (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                path TEXT NOT NULL UNIQUE,
                language TEXT NOT NULL DEFAULT 'Unknown',
                active INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );",
        )
        .map_err(|error| error.to_string())?;

    connection
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS TB_PROJECT_COMPONENTS (
                project_id INTEGER NOT NULL,
                component_id TEXT NOT NULL,
                name TEXT NOT NULL,
                entry TEXT NOT NULL,
                language TEXT NOT NULL,
                PRIMARY KEY (project_id, component_id)
            );",
        )
        .map_err(|error| error.to_string())?;

    ensure_column(&connection, "language", "TEXT NOT NULL DEFAULT 'Unknown'")?;
    ensure_column(&connection, "active", "INTEGER NOT NULL DEFAULT 0")?;
    connection
        .execute(
            "UPDATE TB_PROJECTS SET active = 1
             WHERE id = (SELECT id FROM TB_PROJECTS ORDER BY id DESC LIMIT 1)
               AND NOT EXISTS (SELECT 1 FROM TB_PROJECTS WHERE active = 1)",
            [],
        )
        .map_err(|error| error.to_string())?;
    Ok(connection)
}

fn ensure_column(connection: &Connection, column: &str, definition: &str) -> Result<(), String> {
    let mut statement = connection
        .prepare("PRAGMA table_info(TB_PROJECTS)")
        .map_err(|error| error.to_string())?;
    let columns = statement
        .query_map([], |row| row.get::<_, String>("name"))
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    drop(statement);

    if !columns.iter().any(|existing| existing == column) {
        connection
            .execute_batch(&format!(
                "ALTER TABLE TB_PROJECTS ADD COLUMN {column} {definition};"
            ))
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn detect_language(project_path: &str) -> String {
    let root = Path::new(project_path);
    if root.join("pubspec.yaml").is_file() {
        return "Flutter".to_string();
    }

    let package_json = match fs::read_to_string(root.join("package.json")) {
        Ok(content) => content,
        Err(_) => return "Unknown".to_string(),
    };
    let manifest: serde_json::Value = match serde_json::from_str(&package_json) {
        Ok(manifest) => manifest,
        Err(_) => return "Unknown".to_string(),
    };

    let has_dependency = |packages: &[&str]| {
        ["dependencies", "devDependencies", "peerDependencies"]
            .iter()
            .filter_map(|section| {
                manifest
                    .get(*section)
                    .and_then(serde_json::Value::as_object)
            })
            .any(|dependencies| {
                packages
                    .iter()
                    .any(|package| dependencies.contains_key(*package))
            })
    };

    if ["svelte.config.js", "svelte.config.mjs", "svelte.config.ts"]
        .iter()
        .any(|config| root.join(*config).is_file())
        || has_dependency(&["svelte", "@sveltejs/kit"])
    {
        return "Svelte".to_string();
    }
    if [
        "nuxt.config.js",
        "nuxt.config.mjs",
        "nuxt.config.ts",
        "nuxt.config.mts",
        "nuxt.config.cjs",
    ]
    .iter()
    .any(|config| root.join(*config).is_file())
        || has_dependency(&["nuxt"])
    {
        return "Nuxt".to_string();
    }
    if [
        "next.config.js",
        "next.config.mjs",
        "next.config.ts",
        "next.config.mts",
        "next.config.cjs",
    ]
        .iter()
        .any(|config| root.join(*config).is_file())
        || has_dependency(&["next"])
    {
        return "Nextjs".to_string();
    }
    if ["vue.config.js", "vue.config.ts"]
        .iter()
        .any(|config| root.join(*config).is_file())
        || has_dependency(&["vue", "@vue/cli-service"])
    {
        return "Vue".to_string();
    }
    if has_dependency(&["react", "react-dom"]) {
        return "React".to_string();
    }
    "Unknown".to_string()
}

fn map_project(row: &rusqlite::Row<'_>) -> rusqlite::Result<Project> {
    Ok(Project {
        id: row.get("id")?,
        name: row.get("name")?,
        path: row.get("path")?,
        language: row.get("language")?,
        active: row.get::<_, i64>("active")? != 0,
    })
}

pub fn list_projects(app: &AppHandle) -> Result<Vec<Project>, String> {
    let connection = open_database(app)?;
    let mut statement = connection
        .prepare("SELECT id, name, path, language, active FROM TB_PROJECTS ORDER BY active DESC, id DESC")
        .map_err(|error| error.to_string())?;
    let projects = statement
        .query_map([], map_project)
        .map_err(|error| error.to_string())?;

    let mut projects = projects
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;

    for project in &mut projects {
        if project.language == "Unknown" {
            let language = detect_language(&project.path);
            if language != "Unknown" {
                connection
                    .execute(
                        "UPDATE TB_PROJECTS SET language = ?1 WHERE id = ?2",
                        params![language, project.id],
                    )
                    .map_err(|error| error.to_string())?;
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
        .map_err(|error| error.to_string())?;
    let Some(project_id) = project_id else {
        return Ok(Vec::new());
    };

    let mut statement = connection
        .prepare(
            "SELECT component_id, name, entry FROM TB_PROJECT_COMPONENTS
             WHERE project_id = ?1 ORDER BY entry, name",
        )
        .map_err(|error| error.to_string())?;
    let components = statement
        .query_map([project_id], |row| {
            Ok(ComponentExport {
                id: row.get(0)?,
                name: row.get(1)?,
                entry: row.get(2)?,
            })
        })
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    Ok(components)
}
pub fn save_project(app: &AppHandle, path: &str) -> Result<Project, String> {
    let name = Path::new(path)
        .file_name()
        .and_then(|folder| folder.to_str())
        .filter(|folder| !folder.is_empty())
        .unwrap_or(path)
        .to_string();
    let language = detect_language(path);
    let connection = open_database(app)?;

    connection
        .execute("UPDATE TB_PROJECTS SET active = 0", [])
        .map_err(|error| error.to_string())?;
    connection
        .execute(
            "INSERT INTO TB_PROJECTS (name, path, language, active) VALUES (?1, ?2, ?3, 1)
             ON CONFLICT(path) DO UPDATE SET
               name = excluded.name,
               language = excluded.language,
               active = 1",
            params![name, path, language],
        )
        .map_err(|error| error.to_string())?;

    connection
        .query_row(
            "SELECT id, name, path, language, active FROM TB_PROJECTS WHERE path = ?1",
            [path],
            map_project,
        )
        .map_err(|error| error.to_string())
}

pub fn set_active_project(app: &AppHandle, id: i64) -> Result<Project, String> {
    let connection = open_database(app)?;
    connection
        .execute("UPDATE TB_PROJECTS SET active = 0", [])
        .map_err(|error| error.to_string())?;
    let changed = connection
        .execute("UPDATE TB_PROJECTS SET active = 1 WHERE id = ?1", [id])
        .map_err(|error| error.to_string())?;

    if changed == 0 {
        return Err("O projecto seleccionado já não existe.".to_string());
    }

    connection
        .query_row(
            "SELECT id, name, path, language, active FROM TB_PROJECTS WHERE id = ?1",
            [id],
            map_project,
        )
        .map_err(|error| error.to_string())
}

fn validate_relative_path(path: &str) -> Result<PathBuf, String> {
    let relative_path = Path::new(path);
    if relative_path.as_os_str().is_empty()
        || relative_path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err("O caminho do ficheiro de componente é inválido.".to_string());
    }
    Ok(relative_path.to_path_buf())
}

fn validate_export_name(name: &str) -> Result<(), String> {
    let mut characters = name.chars();
    let starts_valid = characters.next().is_some_and(|character| {
        character.is_ascii_alphabetic() || character == '_' || character == '$'
    });
    if !starts_valid
        || !characters.all(|character| {
            character.is_ascii_alphanumeric() || character == '_' || character == '$'
        })
    {
        return Err(format!("O nome de exportação '{name}' não é válido."));
    }
    Ok(())
}

fn build_index(language: &str, components: &mut [ComponentExport]) -> Result<String, String> {
    components.sort_by(|left, right| left.name.cmp(&right.name));
    let mut index = String::from("// Gerado pelo NegoDesign Studio.\n");
    for component in components {
        validate_export_name(&component.name)?;
        let entry = validate_relative_path(&component.entry)?;
        let entry = entry.to_string_lossy().replace('\\', "/");

        if language == "Flutter" {
            index.push_str(&format!("export '{entry}';\n"));
        } else {
            let entry = if language == "React" || language == "Nextjs" {
                entry.strip_suffix(".tsx").unwrap_or(&entry)
            } else {
                &entry
            };
            index.push_str(&format!(
                "export {{ default as {} }} from \"./{}\";\n",
                component.name, entry
            ));
        }
    }
    if language != "Flutter" {
        index.push_str(
            "\nif (typeof document !== \"undefined\" && !document.querySelector('link[data-negodesign-hugeicons]')) {\n  const hugeiconsStylesheet = document.createElement(\"link\");\n  hugeiconsStylesheet.rel = \"stylesheet\";\n  hugeiconsStylesheet.href = \"/hugeicons/css/hugeicons.css\";\n  hugeiconsStylesheet.dataset.negodesignHugeicons = \"true\";\n  document.head.append(hugeiconsStylesheet);\n}\n",
        );
    }
    Ok(index)
}

fn decode_base64(input: &str) -> Result<Vec<u8>, String> {
    let mut decoded = Vec::with_capacity(input.len() * 3 / 4);
    let mut buffer = 0_u32;
    let mut bits = 0_u8;

    for byte in input.bytes() {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => break,
            _ => return Err("O conteúdo binário em base64 é inválido.".to_string()),
        };

        buffer = (buffer << 6) | u32::from(value);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            decoded.push((buffer >> bits) as u8);
        }
    }

    Ok(decoded)
}

fn ensure_flutter_icons_registered(project_root: &Path) -> Result<bool, String> {
    let pubspec_path = project_root.join("pubspec.yaml");
    let source = fs::read_to_string(&pubspec_path).map_err(|error| error.to_string())?;
    if source.contains("assets/icons/") {
        return Ok(false);
    }

    let mut lines: Vec<String> = source.lines().map(str::to_owned).collect();
    let flutter_index = lines.iter().position(|line| line.trim() == "flutter:");
    if let Some(flutter_index) = flutter_index {
        let section_end = lines
            .iter()
            .enumerate()
            .skip(flutter_index + 1)
            .find(|(_, line)| !line.trim().is_empty() && !line.starts_with(char::is_whitespace))
            .map(|(index, _)| index)
            .unwrap_or(lines.len());
        let assets_index = lines
            .iter()
            .enumerate()
            .take(section_end)
            .skip(flutter_index + 1)
            .find(|(_, line)| line.trim() == "assets:")
            .map(|(index, _)| index);

        if let Some(assets_index) = assets_index {
            lines.insert(assets_index + 1, "    - assets/icons/".to_string());
        } else {
            lines.insert(section_end, "  assets:".to_string());
            lines.insert(section_end + 1, "    - assets/icons/".to_string());
        }
    } else {
        if !lines.last().is_some_and(String::is_empty) {
            lines.push(String::new());
        }
        lines.extend([
            "flutter:".to_string(),
            "  assets:".to_string(),
            "    - assets/icons/".to_string(),
        ]);
    }

    fs::write(pubspec_path, format!("{}\n", lines.join("\n")))
        .map_err(|error| error.to_string())?;
    Ok(true)
}

pub fn install_component(
    app: &AppHandle,
    files: Vec<ComponentFile>,
    components: Vec<ComponentExport>,
) -> Result<ComponentInstallResult, String> {
    if files.is_empty() || components.is_empty() {
        return Err("Não foram encontrados ficheiros ou componentes para instalar.".to_string());
    }
    let connection = open_database(app)?;
    let (project_id, project_path, language): (i64, String, String) = connection
        .query_row(
            "SELECT id, path, language FROM TB_PROJECTS WHERE active = 1 LIMIT 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .map_err(|_| "Seleccione primeiro um projecto activo.".to_string())?;
    if !["Svelte", "React", "Vue", "Nextjs", "Nuxt", "Flutter"]
        .contains(&language.as_str())
    {
        return Err("A linguagem do projecto não foi identificada.".to_string());
    }

    let project_root = PathBuf::from(project_path);
    let package_root = project_root.join("negodesign");
    let public_root = match language.as_str() {
        "Svelte" => project_root.join("static"),
        "Flutter" => project_root.join("assets"),
        _ => project_root.join("public"),
    };
    let mut copied = 0;
    let mut needs_flutter_icons = false;

    for file in files {
        let relative_path = validate_relative_path(&file.path)?;
        if language == "Flutter" && relative_path.starts_with("static/hugeicons/svg") {
            needs_flutter_icons = true;
        }
        let destination = if let Ok(asset_path) = relative_path.strip_prefix("static") {
            if language == "Flutter" && relative_path.starts_with("static/hugeicons/svg/") {
                let file_name = asset_path
                    .file_name()
                    .ok_or_else(|| "O asset SVG não tem um nome de ficheiro.".to_string())?;
                public_root.join("icons").join(file_name)
            } else {
                public_root.join(asset_path)
            }
        } else if relative_path.starts_with("project") || relative_path.starts_with("hooks") {
            package_root.join(&relative_path)
        } else {
            return Err(format!(
                "O ficheiro '{}' não está numa localização permitida.",
                file.path
            ));
        };

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        let content = match file.content.strip_prefix("base64:") {
            Some(encoded) => decode_base64(encoded)?,
            None => file.content.into_bytes(),
        };
        fs::write(destination, content).map_err(|error| error.to_string())?;
        copied += 1;
    }

    if needs_flutter_icons && ensure_flutter_icons_registered(&project_root)? {
        copied += 1;
    }

    for component in &components {
        validate_export_name(&component.name)?;
        validate_relative_path(&component.entry)?;
        connection
            .execute(
                "INSERT INTO TB_PROJECT_COMPONENTS (project_id, component_id, name, entry, language)
                 VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(project_id, component_id) DO UPDATE SET
                   name = excluded.name,
                   entry = excluded.entry,
                   language = excluded.language",
                params![project_id, component.id, component.name, component.entry, language],
            )
            .map_err(|error| error.to_string())?;
    }

    let mut statement = connection
        .prepare(
            "SELECT component_id, name, entry FROM TB_PROJECT_COMPONENTS
             WHERE project_id = ?1 AND language = ?2 ORDER BY name",
        )
        .map_err(|error| error.to_string())?;
    let mut installed = statement
        .query_map(params![project_id, language], |row| {
            Ok(ComponentExport {
                id: row.get(0)?,
                name: row.get(1)?,
                entry: row.get(2)?,
            })
        })
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    drop(statement);

    let index = build_index(&language, &mut installed)?;
    let index_name = if language == "Flutter" {
        "index.dart"
    } else {
        "index.ts"
    };
    fs::create_dir_all(&package_root).map_err(|error| error.to_string())?;
    fs::write(package_root.join(index_name), index).map_err(|error| error.to_string())?;

    Ok(ComponentInstallResult {
        copied_files: copied,
        exported_components: installed.len(),
    })
}
