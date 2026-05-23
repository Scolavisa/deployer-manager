use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("Invalid project path: {0}")]
    InvalidProjectPath(String),

    #[error("Project not found: {0}")]
    ProjectNotFound(String),

    #[error("Failed to parse hosts config: {0}")]
    HostsParseError(String),

    #[error("Deployment already in progress for this environment")]
    DeploymentInProgress,

    #[error("Cannot specify both tag and branch")]
    TagBranchConflict,

    #[error("Process execution failed: {0}")]
    ProcessError(String),

    #[error("Git operation failed: {0}")]
    GitError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}
