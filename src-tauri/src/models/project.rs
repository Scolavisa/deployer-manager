use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A registered project in the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub available: bool,
}
