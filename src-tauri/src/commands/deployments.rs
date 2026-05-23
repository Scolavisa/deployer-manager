use std::time::{Instant, SystemTime, UNIX_EPOCH};

use tauri::{AppHandle, Emitter, State};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::DeploymentStatus;
use crate::services::{process, project};
use crate::state::AppState;

#[tauri::command]
pub async fn start_deployment(
    app_handle: AppHandle,
    project_id: String,
    environment: String,
    tag: Option<String>,
    branch: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    // Validate tag/branch mutual exclusion
    process::validate_deploy_options(&tag, &branch)?;

    // Get project
    let proj = project::get_project(&project_id, &state)?;
    let deploy_config = proj.path.join(".deployments").join("deploy.php");

    // Generate deployment ID
    let deployment_id = Uuid::new_v4().to_string();

    // Record as running
    let started_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    {
        let mut deployments = state.active_deployments.lock().unwrap();
        deployments.insert(
            deployment_id.clone(),
            DeploymentStatus::Running {
                deployment_id: deployment_id.clone(),
                started_at,
            },
        );
    }

    let start = Instant::now();

    // Spawn the deployment process
    let exit_code = process::spawn_deployment(
        &app_handle,
        &deployment_id,
        &proj.path,
        &deploy_config,
        &environment,
        tag.as_deref(),
        branch.as_deref(),
    )
    .await?;

    let duration_secs = start.elapsed().as_secs_f64();
    let success = exit_code == 0;

    // Update status to completed
    {
        let mut deployments = state.active_deployments.lock().unwrap();
        deployments.insert(
            deployment_id.clone(),
            DeploymentStatus::Completed {
                deployment_id: deployment_id.clone(),
                success,
                exit_code,
                duration_secs,
            },
        );
    }

    // Emit completion event
    let _ = app_handle.emit(
        "deploy_complete",
        serde_json::json!({
            "deployment_id": deployment_id,
            "success": success,
            "exit_code": exit_code,
        }),
    );

    Ok(deployment_id)
}

#[tauri::command]
pub async fn get_deployment_status(
    deployment_id: String,
    state: State<'_, AppState>,
) -> Result<DeploymentStatus, AppError> {
    let deployments = state.active_deployments.lock().unwrap();
    deployments
        .get(&deployment_id)
        .cloned()
        .ok_or_else(|| AppError::ProjectNotFound(format!("Deployment not found: {}", deployment_id)))
}
