use log::{info, error};
use tauri::State;

use crate::error::AppError;
use crate::models::Environment;
use crate::services::{hosts, project};
use crate::state::AppState;

#[tauri::command]
pub async fn get_environments(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Environment>, AppError> {
    let proj = project::get_project(&project_id, &state)?;
    let hosts_path = proj.path.join(".deployments").join("hosts.yaml");
    info!("Loading environments from: {:?}", hosts_path);
    match hosts::parse_hosts_file(&hosts_path) {
        Ok(envs) => {
            info!("Found {} environments: {:?}", envs.len(), envs.iter().map(|e| &e.name).collect::<Vec<_>>());
            Ok(envs)
        }
        Err(e) => {
            error!("Failed to parse hosts file {:?}: {}", hosts_path, e);
            Err(e)
        }
    }
}
