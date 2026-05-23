use tauri::State;

use crate::error::AppError;
use crate::services::{git as git_service, project};
use crate::state::AppState;

#[tauri::command]
pub async fn get_tags(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<String>, AppError> {
    let proj = project::get_project(&project_id, &state)?;
    git_service::get_tags(&proj.path)
}

#[tauri::command]
pub async fn get_branches(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<String>, AppError> {
    let proj = project::get_project(&project_id, &state)?;
    git_service::get_branches(&proj.path)
}
