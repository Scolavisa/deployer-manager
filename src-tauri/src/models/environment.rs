use serde::{Deserialize, Serialize};

/// A deployment environment parsed from hosts.yaml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub name: String,
    pub hostname: String,
    pub remote_user: String,
    pub deploy_path: String,
    pub branch: Option<String>,
    pub stage: Option<String>,
    pub keep_releases: Option<u32>,
}
