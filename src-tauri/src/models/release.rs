use serde::{Deserialize, Serialize};

/// A past release retrieved from `dep releases`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub name: String,
    pub date: Option<String>,
    pub is_current: bool,
    pub target: Option<String>,
}
