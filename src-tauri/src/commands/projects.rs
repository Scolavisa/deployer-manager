use tauri::State;

use crate::error::AppError;
use crate::models::Project;
use crate::services::config::ConfigManager;
use crate::services::project;
use crate::state::AppState;

#[tauri::command]
pub async fn register_project(
    path: String,
    state: State<'_, AppState>,
    config_manager: State<'_, ConfigManager>,
) -> Result<Project, AppError> {
    let path = std::path::Path::new(&path);
    project::register_project(path, &state, &config_manager)
}

#[tauri::command]
pub async fn remove_project(
    project_id: String,
    state: State<'_, AppState>,
    config_manager: State<'_, ConfigManager>,
) -> Result<(), AppError> {
    project::remove_project(&project_id, &state, &config_manager)
}

#[tauri::command]
pub async fn list_projects(state: State<'_, AppState>) -> Result<Vec<Project>, AppError> {
    Ok(project::list_projects(&state))
}

#[tauri::command]
pub async fn get_project(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Project, AppError> {
    project::get_project(&project_id, &state)
}
