use crate::{
    model::{
        codegen::{
            FullstackCodegenRequest, FullstackCodegenResult, OpenApiFrontendCodegenRequest,
            OpenApiFrontendCodegenResult,
        },
        migration::{DatabaseConnection, DatabaseTable, OpenApiEntity, SavedDatabaseConnection},
        project::{
            ComponentCopyRequest, ComponentExport, ComponentFile, ComponentInstallResult, Project,
        },
    },
    repository::{
        codegen_repository, component_copy_repository, database_repository, openapi_repository,
        project_repository,
    },
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
pub fn list_active_project_components(app: AppHandle) -> Result<Vec<ComponentExport>, String> {
    project_repository::list_active_project_components(&app)
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

#[tauri::command]
pub fn copy_component_svelte(
    app: AppHandle,
    request: ComponentCopyRequest,
) -> Result<ComponentInstallResult, String> {
    component_copy_repository::copy_svelte(&app, request)
}

#[tauri::command]
pub fn copy_component_react(
    app: AppHandle,
    request: ComponentCopyRequest,
) -> Result<ComponentInstallResult, String> {
    component_copy_repository::copy_react(&app, request)
}

#[tauri::command]
pub fn copy_component_nextjs(
    app: AppHandle,
    request: ComponentCopyRequest,
) -> Result<ComponentInstallResult, String> {
    component_copy_repository::copy_nextjs(&app, request)
}

#[tauri::command]
pub fn copy_component_vue(
    app: AppHandle,
    request: ComponentCopyRequest,
) -> Result<ComponentInstallResult, String> {
    component_copy_repository::copy_vue(&app, request)
}

#[tauri::command]
pub fn copy_component_nuxt(
    app: AppHandle,
    request: ComponentCopyRequest,
) -> Result<ComponentInstallResult, String> {
    component_copy_repository::copy_nuxt(&app, request)
}

#[tauri::command]
pub fn copy_component_flutter(
    app: AppHandle,
    request: ComponentCopyRequest,
) -> Result<ComponentInstallResult, String> {
    component_copy_repository::copy_flutter(&app, request)
}

#[tauri::command]
pub fn connect_database(
    app: AppHandle,
    connection: DatabaseConnection,
) -> Result<Vec<DatabaseTable>, String> {
    database_repository::inspect_and_save(&app, &connection)
}

#[tauri::command]
pub fn list_saved_database_connections(
    app: AppHandle,
) -> Result<Vec<SavedDatabaseConnection>, String> {
    database_repository::list_saved_connections(&app)
}

#[tauri::command]
pub fn connect_saved_database(app: AppHandle, id: i64) -> Result<Vec<DatabaseTable>, String> {
    database_repository::inspect_saved_connection(&app, id)
}

#[tauri::command]
pub fn fetch_openapi(url: String) -> Result<Vec<OpenApiEntity>, String> {
    openapi_repository::extract(&url)
}

#[tauri::command]
pub fn import_openapi_file(path: String) -> Result<Vec<OpenApiEntity>, String> {
    openapi_repository::extract_file(&path)
}

#[tauri::command]
pub fn generate_fullstack_crud(
    request: FullstackCodegenRequest,
) -> Result<FullstackCodegenResult, String> {
    codegen_repository::generate_fullstack_crud(request)
}

#[tauri::command]
pub fn generate_openapi_frontend(
    request: OpenApiFrontendCodegenRequest,
) -> Result<OpenApiFrontendCodegenResult, String> {
    codegen_repository::generate_openapi_frontend(request)
}
