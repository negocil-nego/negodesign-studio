use crate::{
    model::project::{
        ComponentCopyRequest, ComponentExport, ComponentFile, ComponentInstallResult,
    },
    repository::{
        component_copy::{
            assets::relocate_static_assets, collector::CopyCollector, manifest::ManifestCatalog,
            paths::workspace_root, theme::attach_theme_and_styles,
        },
        project_repository,
    },
};
use tauri::AppHandle;

pub fn copy_svelte(
    app: &AppHandle,
    request: ComponentCopyRequest,
) -> Result<ComponentInstallResult, String> {
    copy_component_with_language(app, "Svelte", request)
}

pub fn copy_react(
    app: &AppHandle,
    request: ComponentCopyRequest,
) -> Result<ComponentInstallResult, String> {
    copy_component_with_language(app, "React", request)
}

pub fn copy_nextjs(
    app: &AppHandle,
    request: ComponentCopyRequest,
) -> Result<ComponentInstallResult, String> {
    copy_component_with_language(app, "React", request)
}

pub fn copy_vue(
    app: &AppHandle,
    request: ComponentCopyRequest,
) -> Result<ComponentInstallResult, String> {
    copy_component_with_language(app, "Vue", request)
}

pub fn copy_nuxt(
    app: &AppHandle,
    request: ComponentCopyRequest,
) -> Result<ComponentInstallResult, String> {
    copy_component_with_language(app, "Vue", request)
}

pub fn copy_flutter(
    app: &AppHandle,
    request: ComponentCopyRequest,
) -> Result<ComponentInstallResult, String> {
    copy_component_with_language(app, "Flutter", request)
}

pub fn copy_component_with_language(
    app: &AppHandle,
    manifest_lang: &str,
    request: ComponentCopyRequest,
) -> Result<ComponentInstallResult, String> {
    let root = workspace_root()?;
    let catalog = ManifestCatalog::load(&root.join("src"))?;

    let locales = request
        .locales
        .unwrap_or_else(|| vec!["en".to_string(), "pt".to_string()]);
    let selected_theme = request.theme.unwrap_or_else(|| "default".to_string());

    let mut collector = CopyCollector::new(&root, &catalog, manifest_lang, &locales);
    collector.collect(&request.component_id, request.manifest_path.as_deref())?;

    attach_theme_and_styles(
        &root,
        &mut collector.files_by_path,
        manifest_lang,
        &selected_theme,
    )?;

    let final_map = if let Some(static_dir) = request.static_directory.as_deref() {
        relocate_static_assets(collector.files_by_path, static_dir)
    } else {
        collector.files_by_path
    };

    let files: Vec<ComponentFile> = final_map
        .into_iter()
        .map(|(path, content)| ComponentFile { path, content })
        .collect();
    let components: Vec<ComponentExport> = collector.exported_by_id.into_values().collect();

    project_repository::install_component(app, files, components)
}
