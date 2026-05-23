pub mod commands;
pub mod error;
pub mod models;
pub mod services;
pub mod state;

use services::config::ConfigManager;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config_manager = ConfigManager::new().expect("Failed to initialize config manager");
    let config = config_manager.load();
    let app_state = AppState::new(config);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .manage(app_state)
        .manage(config_manager)
        .invoke_handler(tauri::generate_handler![
            commands::projects::register_project,
            commands::projects::remove_project,
            commands::projects::list_projects,
            commands::projects::get_project,
            commands::environments::get_environments,
            commands::deployments::start_deployment,
            commands::deployments::get_deployment_status,
            commands::releases::get_releases,
            commands::git::get_tags,
            commands::git::get_branches,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
