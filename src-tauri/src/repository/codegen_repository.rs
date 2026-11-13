use crate::{
    model::{
        codegen::{
            ArchitectureMode, FrontendFramework, FullstackCodegenRequest, FullstackCodegenResult,
            OpenApiFrontendCodegenRequest, OpenApiFrontendCodegenResult,
        },
        project::{ComponentExport, ComponentFile},
    },
    repository::{
        codegen::{
            backend::generate_backend_files, frontend::generate_frontend_files,
            openapi::generate_openapi_frontend_files,
        },
        component_copy::{
            assets::relocate_static_assets, collector::CopyCollector, manifest::ManifestCatalog,
            paths::workspace_root, theme::attach_theme_and_styles,
        },
        project::installer::{build_index, validate_relative_path, write_project_files},
    },
};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

pub fn generate_fullstack_crud(
    request: FullstackCodegenRequest,
) -> Result<FullstackCodegenResult, String> {
    if request.tables.is_empty() {
        return Err("Seleccione pelo menos uma tabela para gerar o CRUD.".to_string());
    }
    let frontend_path = request.frontend_path.trim();
    if frontend_path.is_empty() {
        return Err("Indique o caminho da aplicação frontend.".to_string());
    }
    let frontend_root = PathBuf::from(frontend_path);

    let mut summary = Vec::new();
    let copied_ui_files = install_admin_crud_ui_by_framework(
        &frontend_root,
        &request.frontend_framework,
        request.theme.as_deref(),
    )?;
    summary.push(format!(
        "Copiados {copied_ui_files} ficheiros do componente AdminCrud e dependências para {frontend_path}/negodesign"
    ));

    let frontend_files = generate_frontend_files(&request, &request.tables)?;
    let generated_frontend_files = write_file_map(&frontend_root, &frontend_files)?;
    summary.push(format!(
        "Gerados {generated_frontend_files} ficheiros de páginas e integração no projecto frontend ({:?})",
        request.frontend_framework
    ));

    let mut generated_backend_files = 0;
    if request.architecture_mode == ArchitectureMode::Split {
        let backend_path = request
            .backend_path
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| "Indique o caminho da aplicação backend.".to_string())?;
        let backend_root = PathBuf::from(backend_path);
        let backend_files = generate_backend_files(&request, &request.tables)?;

        // Se for projecto Spring Boot existente com pom.xml, adiciona apenas dependências em falta sem sobrescrever
        if matches!(
            request.backend_framework,
            Some(crate::model::codegen::BackendFramework::SpringBoot)
        ) {
            let pom_path = backend_root.join("pom.xml");
            if pom_path.exists() {
                if let Ok(true) = crate::repository::codegen::backend::spring_boot::pom_merger::ensure_pom_dependencies(
                    &pom_path,
                    &request.database_engine,
                ) {
                    summary.push("Adicionadas dependências em falta ao pom.xml existente sem sobrescrever a configuração do projecto".to_string());
                }
            }
        }

        generated_backend_files = write_file_map(&backend_root, &backend_files)?;
        let lang_note = match (&request.backend_framework, &request.spring_language) {
            (Some(crate::model::codegen::BackendFramework::SpringBoot), Some(lang)) => {
                format!(", linguagem: {lang:?}")
            }
            _ => String::new(),
        };
        summary.push(format!(
            "Gerados {generated_backend_files} ficheiros no projecto backend ({:?}, ORM: {}{})",
            request.backend_framework,
            request.orm_strategy.as_deref().unwrap_or("default"),
            lang_note
        ));
    } else {
        summary.push(format!(
            "Geradas rotas de API fullstack com ligação direta a {:?} na aplicação frontend",
            request.database_engine
        ));
    }

    Ok(FullstackCodegenResult {
        generated_backend_files,
        generated_frontend_files,
        copied_ui_files,
        tables_processed: request.tables.len(),
        summary,
    })
}

pub fn generate_openapi_frontend(
    request: OpenApiFrontendCodegenRequest,
) -> Result<OpenApiFrontendCodegenResult, String> {
    if request.entities.is_empty() {
        return Err("Nenhuma entidade OpenAPI fornecida para geração.".to_string());
    }
    let frontend_path = request.frontend_path.trim();
    if frontend_path.is_empty() {
        return Err("Indique o caminho da aplicação frontend.".to_string());
    }
    let frontend_root = PathBuf::from(frontend_path);

    let mut summary = Vec::new();
    let copied_ui_files = install_admin_crud_ui_by_framework(
        &frontend_root,
        &request.frontend_framework,
        request.theme.as_deref(),
    )?;
    summary.push(format!(
        "Copiados {copied_ui_files} ficheiros do componente AdminCrud e dependências para {frontend_path}/negodesign"
    ));

    let frontend_files = generate_openapi_frontend_files(&request)?;
    let generated_frontend_files = write_file_map(&frontend_root, &frontend_files)?;
    let endpoints_count = if request.selected_endpoints.is_empty() {
        request.entities.len()
    } else {
        request.selected_endpoints.len()
    };
    summary.push(format!(
        "Gerados {generated_frontend_files} ficheiros de DTOs, API Client, TanStack Queries e páginas AdminCrud ({:?})",
        request.frontend_framework
    ));

    Ok(OpenApiFrontendCodegenResult {
        generated_frontend_files,
        copied_ui_files,
        endpoints_processed: endpoints_count,
        summary,
    })
}

fn install_admin_crud_ui_by_framework(
    frontend_root: &Path,
    framework: &FrontendFramework,
    theme: Option<&str>,
) -> Result<usize, String> {
    let root = workspace_root()?;
    let catalog = ManifestCatalog::global(&root.join("src"))?;

    let (manifest_lang, project_lang, static_dir) = match framework {
        FrontendFramework::Sveltekit => ("Svelte", "Svelte", Some("static/nego")),
        FrontendFramework::Nuxt => ("Vue", "Nuxt", Some("public")),
        FrontendFramework::Nextjs => ("React", "Nextjs", Some("public")),
    };

    let locales = vec!["en".to_string(), "pt".to_string()];
    let selected_theme = theme.unwrap_or("default");

    let mut collector = CopyCollector::new(&root, catalog, manifest_lang, &locales);
    collector.collect(
        "admin-crud",
        Some("src/project/ui/pages/admin/dependence.json"),
    )?;

    attach_theme_and_styles(
        &root,
        &mut collector.files_by_path,
        manifest_lang,
        selected_theme,
    )?;

    let final_map = if let Some(sd) = static_dir {
        relocate_static_assets(collector.files_by_path, sd)
    } else {
        collector.files_by_path
    };

    let files: Vec<ComponentFile> = final_map
        .into_iter()
        .map(|(path, content)| ComponentFile { path, content })
        .collect();
    let mut components: Vec<ComponentExport> = collector.exported_by_id.into_values().collect();

    let copied = write_project_files(frontend_root, project_lang, files)?;
    let package_root = frontend_root.join("negodesign");
    let index = build_index(project_lang, &mut components)?;
    fs::create_dir_all(&package_root).map_err(|e| e.to_string())?;
    fs::write(package_root.join("index.ts"), index).map_err(|e| e.to_string())?;

    Ok(copied)
}

fn write_file_map(base_dir: &Path, files: &BTreeMap<String, String>) -> Result<usize, String> {
    let mut count = 0;
    for (rel_path, content) in files {
        let valid_rel = validate_relative_path(rel_path)?;
        let full_path = base_dir.join(valid_rel);
        if rel_path == "pom.xml" && full_path.exists() {
            continue;
        }
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(full_path, content).map_err(|e| e.to_string())?;
        count += 1;
    }
    Ok(count)
}

