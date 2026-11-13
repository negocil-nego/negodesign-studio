use crate::{
    model::{
        codegen::{
            CodegenPath, FullstackCodegenRequest, FullstackCodegenResult,
            OpenApiFrontendCodegenRequest, OpenApiFrontendCodegenResult, SpringBootProjectRequest,
            SpringBootProjectResult,
        },
        migration::{DatabaseConnection, DatabaseTable, OpenApiEntity, SavedDatabaseConnection},
        project::{
            ComponentCopyRequest, ComponentExport, ComponentFile, ComponentInstallResult, Project,
        },
    },
    repository::{
        aspnet_project_creator, codegen_paths_repository, codegen_repository,
        component_copy_repository, database_repository, laravel_project_creator,
        nestjs_project_creator, openapi_repository, project_repository,
        spring_boot_project_creator,
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
    app: AppHandle,
    request: FullstackCodegenRequest,
) -> Result<FullstackCodegenResult, String> {
    let result = codegen_repository::generate_fullstack_crud(request.clone())?;
    let _ = codegen_paths_repository::save_path(
        &app,
        "frontend",
        &request.frontend_path,
        Some(format!("{:?}", request.frontend_framework).as_str()),
    );
    if let (true, Some(backend_path), Some(backend_framework)) = (
        matches!(request.architecture_mode, crate::model::codegen::ArchitectureMode::Split),
        request.backend_path.as_deref(),
        request.backend_framework.as_ref(),
    ) {
        let _ = codegen_paths_repository::save_path(
            &app,
            "backend",
            backend_path,
            Some(format!("{backend_framework:?}").as_str()),
        );
    }
    Ok(result)
}

#[tauri::command]
pub fn generate_openapi_frontend(
    request: OpenApiFrontendCodegenRequest,
) -> Result<OpenApiFrontendCodegenResult, String> {
    codegen_repository::generate_openapi_frontend(request)
}

#[tauri::command]
pub fn list_codegen_paths(
    app: AppHandle,
    target: String,
) -> Result<Vec<CodegenPath>, String> {
    codegen_paths_repository::list_paths(&app, &target)
}

#[tauri::command]
pub fn save_codegen_path(
    app: AppHandle,
    target: String,
    path: String,
    label: Option<String>,
) -> Result<CodegenPath, String> {
    codegen_paths_repository::save_path(&app, &target, &path, label.as_deref())
}

#[tauri::command]
pub fn delete_codegen_path(app: AppHandle, id: i64) -> Result<(), String> {
    codegen_paths_repository::delete_path(&app, id)
}

#[tauri::command]
pub fn create_spring_boot_project(
    request: SpringBootProjectRequest,
) -> Result<SpringBootProjectResult, String> {
    spring_boot_project_creator::create_spring_boot_project(&request)
}

#[tauri::command]
pub fn create_nestjs_project(
    request: crate::model::codegen::NestjsProjectRequest,
) -> Result<crate::model::codegen::BackendProjectResult, String> {
    nestjs_project_creator::create_nestjs_project(&request)
}

#[tauri::command]
pub fn create_laravel_project(
    request: crate::model::codegen::LaravelProjectRequest,
) -> Result<crate::model::codegen::BackendProjectResult, String> {
    laravel_project_creator::create_laravel_project(&request)
}

#[tauri::command]
pub fn create_aspnet_project(
    request: crate::model::codegen::AspnetProjectRequest,
) -> Result<crate::model::codegen::BackendProjectResult, String> {
    aspnet_project_creator::create_aspnet_project(&request)
}
