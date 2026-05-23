use std::fs;
use std::path::PathBuf;

use crate::error::AppError;
use crate::models::AppConfig;

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    /// Create a new ConfigManager with the default platform config path
    pub fn new() -> Result<Self, AppError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| AppError::ConfigError("Could not determine config directory".into()))?;
        let config_path = config_dir.join("deployment-manager").join("config.json");
        Ok(Self { config_path })
    }

    /// Create a ConfigManager with a custom path (useful for testing)
    pub fn with_path(config_path: PathBuf) -> Self {
        Self { config_path }
    }

    /// Get the config file path
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }

    /// Load configuration from disk. Returns empty config if file is missing or corrupted.
    pub fn load(&self) -> AppConfig {
        match fs::read_to_string(&self.config_path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => AppConfig::default(),
        }
    }

    /// Save configuration to disk. Creates directory structure if needed.
    pub fn save(&self, config: &AppConfig) -> Result<(), AppError> {
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                AppError::ConfigError(format!("Failed to create config directory: {}", e))
            })?;
        }
        let json = serde_json::to_string_pretty(config)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize config: {}", e)))?;
        fs::write(&self.config_path, json)
            .map_err(|e| AppError::ConfigError(format!("Failed to write config file: {}", e)))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ProjectConfig;
    use tempfile::TempDir;

    #[test]
    fn test_load_missing_file_returns_default() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("nonexistent").join("config.json");
        let manager = ConfigManager::with_path(path);
        let config = manager.load();
        assert_eq!(config, AppConfig::default());
    }

    #[test]
    fn test_load_corrupted_file_returns_default() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("config.json");
        fs::write(&path, "not valid json {{{").unwrap();
        let manager = ConfigManager::with_path(path);
        let config = manager.load();
        assert_eq!(config, AppConfig::default());
    }

    #[test]
    fn test_save_creates_directory_and_file() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("subdir").join("config.json");
        let manager = ConfigManager::with_path(path.clone());

        let config = AppConfig {
            projects: vec![ProjectConfig {
                id: "test-id".into(),
                name: "TestProject".into(),
                path: "/home/user/project".into(),
            }],
        };

        manager.save(&config).unwrap();
        assert!(path.exists());

        let content = fs::read_to_string(&path).unwrap();
        let loaded: AppConfig = serde_json::from_str(&content).unwrap();
        assert_eq!(loaded, config);
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("config.json");
        let manager = ConfigManager::with_path(path);

        let config = AppConfig {
            projects: vec![
                ProjectConfig {
                    id: "id-1".into(),
                    name: "Project1".into(),
                    path: "/path/one".into(),
                },
                ProjectConfig {
                    id: "id-2".into(),
                    name: "Project2".into(),
                    path: "/path/two".into(),
                },
            ],
        };

        manager.save(&config).unwrap();
        let loaded = manager.load();
        assert_eq!(loaded, config);
    }

    #[test]
    fn test_config_path_returns_correct_path() {
        let path = PathBuf::from("/some/path/config.json");
        let manager = ConfigManager::with_path(path.clone());
        assert_eq!(manager.config_path(), &path);
    }
}
