use crate::{model::project::Project, repository::project_repository};
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
