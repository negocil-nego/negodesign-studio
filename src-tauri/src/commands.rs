use crate::{
    model::project::{ComponentExport, ComponentFile, ComponentInstallResult, Project},
    repository::project_repository,
};
use tauri::AppHandle;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn list_projects(app: AppHandle) -> Result<Vec<Project>, String> {
    project_repository::list_projects(&app)
}

#[tauri::command]
pub fn save_project(app: AppHandle, path: String) -> Result<Project, String> {
    project_repository::save_project(&app, &path)
}

#[tauri::command]
pub fn set_active_project(app: AppHandle, id: i64) -> Result<Project, String> {
    project_repository::set_active_project(&app, id)
}

#[tauri::command]
pub fn install_component(
    app: AppHandle,
    files: Vec<ComponentFile>,
    components: Vec<ComponentExport>,
) -> Result<ComponentInstallResult, String> {
    project_repository::install_component(&app, files, components)
}
