use std::collections::HashMap;
use std::sync::Mutex;

use crate::models::{AppConfig, DeploymentStatus};

/// Shared application state managed by Tauri
pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub active_deployments: Mutex<HashMap<String, DeploymentStatus>>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config: Mutex::new(config),
            active_deployments: Mutex::new(HashMap::new()),
        }
    }
}
