use serde::{Deserialize, Serialize};

/// Configuration for a single project (persisted to disk)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectConfig {
    pub id: String,
    pub name: String,
    pub path: String,
}

/// Application configuration persisted to disk
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AppConfig {
    pub projects: Vec<ProjectConfig>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Strategy to generate arbitrary ProjectConfig
    fn arb_project_config() -> impl Strategy<Value = ProjectConfig> {
        (
            "[a-z0-9]{8}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{12}",
            "[a-zA-Z][a-zA-Z0-9_-]{0,30}",
            "/[a-zA-Z0-9/_-]{1,50}",
        )
            .prop_map(|(id, name, path)| ProjectConfig { id, name, path })
    }

    // Strategy to generate arbitrary AppConfig
    fn arb_app_config() -> impl Strategy<Value = AppConfig> {
        proptest::collection::vec(arb_project_config(), 0..10)
            .prop_map(|projects| AppConfig { projects })
    }

    proptest! {
        // Feature: deployment-manager, Property 3: Configuration serialization round-trip
        // **Validates: Requirements 1.4, 7.2**
        #[test]
        fn config_serialization_roundtrip(config in arb_app_config()) {
            let json = serde_json::to_string(&config).unwrap();
            let deserialized: AppConfig = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(config, deserialized);
        }
    }
}
