use serde::{Deserialize, Serialize};

/// Status of an active or completed deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DeploymentStatus {
    Running {
        deployment_id: String,
        started_at: String,
    },
    Completed {
        deployment_id: String,
        success: bool,
        exit_code: i32,
        duration_secs: f64,
    },
}

/// A single line of deployment output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentOutput {
    pub deployment_id: String,
    pub line: String,
    pub stream: OutputStream,
}

/// Which output stream a line came from
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputStream {
    Stdout,
    Stderr,
}
