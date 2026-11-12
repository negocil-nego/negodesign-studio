mod commands;
mod model;
mod repository;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::list_projects,
            commands::list_active_project_components,
            commands::save_project,
            commands::set_active_project,
            commands::install_component,
            commands::copy_component_svelte,
            commands::copy_component_react,
            commands::copy_component_nextjs,
            commands::copy_component_vue,
            commands::copy_component_nuxt,
            commands::copy_component_flutter,
            commands::connect_database,
            commands::list_saved_database_connections,
            commands::connect_saved_database,
            commands::fetch_openapi,
            commands::import_openapi_file,
            commands::generate_fullstack_crud,
            commands::generate_openapi_frontend
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
