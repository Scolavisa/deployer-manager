use std::path::Path;

use uuid::Uuid;

use crate::error::AppError;
use crate::models::{Project, ProjectConfig};
use crate::services::config::ConfigManager;
use crate::state::AppState;

/// Validate that a path is a valid project directory containing `.deployments/deploy.php`
pub fn validate_project_path(path: &Path) -> Result<(), AppError> {
    if !path.exists() {
        return Err(AppError::InvalidProjectPath(format!(
            "Path does not exist: {}",
            path.display()
        )));
    }

    let deploy_file = path.join(".deployments").join("deploy.php");
    if !deploy_file.exists() {
        return Err(AppError::InvalidProjectPath(format!(
            "Missing .deployments/deploy.php in: {}",
            path.display()
        )));
    }

    Ok(())
}

/// Register a new project: validate path, generate ID, persist to config
pub fn register_project(
    path: &Path,
    state: &AppState,
    config_manager: &ConfigManager,
) -> Result<Project, AppError> {
    validate_project_path(path)?;

    let id = Uuid::new_v4().to_string();
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let project_config = ProjectConfig {
        id: id.clone(),
        name: name.clone(),
        path: path.to_string_lossy().to_string(),
    };

    {
        let mut config = state.config.lock().unwrap();
        config.projects.push(project_config);
        config_manager.save(&config)?;
    }

    Ok(Project {
        id,
        name,
        path: path.to_path_buf(),
        available: true,
    })
}

/// Remove a project by ID from config and persist
pub fn remove_project(
    project_id: &str,
    state: &AppState,
    config_manager: &ConfigManager,
) -> Result<(), AppError> {
    let mut config = state.config.lock().unwrap();
    let original_len = config.projects.len();
    config.projects.retain(|p| p.id != project_id);

    if config.projects.len() == original_len {
        return Err(AppError::ProjectNotFound(project_id.to_string()));
    }

    config_manager.save(&config)?;
    Ok(())
}

/// List all projects with `available` field reflecting whether path exists on disk
pub fn list_projects(state: &AppState) -> Vec<Project> {
    let config = state.config.lock().unwrap();
    config
        .projects
        .iter()
        .map(|pc| {
            let path = Path::new(&pc.path);
            Project {
                id: pc.id.clone(),
                name: pc.name.clone(),
                path: path.to_path_buf(),
                available: path.exists(),
            }
        })
        .collect()
}

/// Get a single project by ID
pub fn get_project(project_id: &str, state: &AppState) -> Result<Project, AppError> {
    let config = state.config.lock().unwrap();
    let pc = config
        .projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| AppError::ProjectNotFound(project_id.to_string()))?;

    let path = Path::new(&pc.path);
    Ok(Project {
        id: pc.id.clone(),
        name: pc.name.clone(),
        path: path.to_path_buf(),
        available: path.exists(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::AppConfig;
    use proptest::prelude::*;
    use tempfile::TempDir;

    #[test]
    fn test_validate_project_path_nonexistent() {
        let path = Path::new("/nonexistent/path/project");
        let result = validate_project_path(path);
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::InvalidProjectPath(msg) => assert!(msg.contains("does not exist")),
            _ => panic!("Expected InvalidProjectPath"),
        }
    }

    #[test]
    fn test_validate_project_path_missing_deploy_php() {
        let tmp = TempDir::new().unwrap();
        let result = validate_project_path(tmp.path());
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::InvalidProjectPath(msg) => {
                assert!(msg.contains("Missing .deployments/deploy.php"))
            }
            _ => panic!("Expected InvalidProjectPath"),
        }
    }

    #[test]
    fn test_validate_project_path_valid() {
        let tmp = TempDir::new().unwrap();
        let deploy_dir = tmp.path().join(".deployments");
        std::fs::create_dir_all(&deploy_dir).unwrap();
        std::fs::write(deploy_dir.join("deploy.php"), "<?php").unwrap();

        let result = validate_project_path(tmp.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_project_success() {
        let tmp = TempDir::new().unwrap();
        let deploy_dir = tmp.path().join(".deployments");
        std::fs::create_dir_all(&deploy_dir).unwrap();
        std::fs::write(deploy_dir.join("deploy.php"), "<?php").unwrap();

        let config_tmp = TempDir::new().unwrap();
        let config_manager =
            ConfigManager::with_path(config_tmp.path().join("config.json"));
        let state = AppState::new(AppConfig::default());

        let project = register_project(tmp.path(), &state, &config_manager).unwrap();
        assert!(!project.id.is_empty());
        assert_eq!(
            project.name,
            tmp.path().file_name().unwrap().to_str().unwrap()
        );
        assert_eq!(project.path, tmp.path());
        assert!(project.available);

        // Verify persisted
        let loaded = config_manager.load();
        assert_eq!(loaded.projects.len(), 1);
        assert_eq!(loaded.projects[0].id, project.id);
    }

    #[test]
    fn test_register_project_invalid_path() {
        let config_tmp = TempDir::new().unwrap();
        let config_manager =
            ConfigManager::with_path(config_tmp.path().join("config.json"));
        let state = AppState::new(AppConfig::default());

        let result =
            register_project(Path::new("/nonexistent"), &state, &config_manager);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_project_success() {
        let config_tmp = TempDir::new().unwrap();
        let config_manager =
            ConfigManager::with_path(config_tmp.path().join("config.json"));
        let config = AppConfig {
            projects: vec![ProjectConfig {
                id: "test-id".into(),
                name: "TestProject".into(),
                path: "/some/path".into(),
            }],
        };
        let state = AppState::new(config);

        let result = remove_project("test-id", &state, &config_manager);
        assert!(result.is_ok());

        let loaded = config_manager.load();
        assert!(loaded.projects.is_empty());
    }

    #[test]
    fn test_remove_project_not_found() {
        let config_tmp = TempDir::new().unwrap();
        let config_manager =
            ConfigManager::with_path(config_tmp.path().join("config.json"));
        let state = AppState::new(AppConfig::default());

        let result = remove_project("nonexistent", &state, &config_manager);
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::ProjectNotFound(id) => assert_eq!(id, "nonexistent"),
            _ => panic!("Expected ProjectNotFound"),
        }
    }

    #[test]
    fn test_list_projects_with_availability() {
        let tmp = TempDir::new().unwrap();
        let config = AppConfig {
            projects: vec![
                ProjectConfig {
                    id: "exists".into(),
                    name: "Exists".into(),
                    path: tmp.path().to_string_lossy().to_string(),
                },
                ProjectConfig {
                    id: "missing".into(),
                    name: "Missing".into(),
                    path: "/nonexistent/path".into(),
                },
            ],
        };
        let state = AppState::new(config);

        let projects = list_projects(&state);
        assert_eq!(projects.len(), 2);

        let existing = projects.iter().find(|p| p.id == "exists").unwrap();
        assert!(existing.available);

        let missing = projects.iter().find(|p| p.id == "missing").unwrap();
        assert!(!missing.available);
    }

    #[test]
    fn test_get_project_found() {
        let config = AppConfig {
            projects: vec![ProjectConfig {
                id: "my-id".into(),
                name: "MyProject".into(),
                path: "/some/path".into(),
            }],
        };
        let state = AppState::new(config);

        let project = get_project("my-id", &state).unwrap();
        assert_eq!(project.id, "my-id");
        assert_eq!(project.name, "MyProject");
    }

    #[test]
    fn test_get_project_not_found() {
        let state = AppState::new(AppConfig::default());
        let result = get_project("nonexistent", &state);
        assert!(result.is_err());
    }

    proptest! {
        // Feature: deployment-manager, Property 1: Project validation determines registration outcome
        // **Validates: Requirements 1.1, 1.2**
        #[test]
        fn project_validation_matches_file_existence(has_deploy_php in proptest::bool::ANY) {
            let tmp = TempDir::new().unwrap();
            if has_deploy_php {
                let deploy_dir = tmp.path().join(".deployments");
                std::fs::create_dir_all(&deploy_dir).unwrap();
                std::fs::write(deploy_dir.join("deploy.php"), "<?php").unwrap();
            }
            let result = validate_project_path(tmp.path());
            prop_assert_eq!(result.is_ok(), has_deploy_php);
        }

        // Feature: deployment-manager, Property 2: Project removal decreases list by one
        // **Validates: Requirements 1.3**
        #[test]
        fn project_removal_decreases_list_by_one(
            count in 1usize..6,
            remove_idx in 0usize..5,
        ) {
            let remove_idx = remove_idx % count;
            let projects: Vec<ProjectConfig> = (0..count)
                .map(|i| ProjectConfig {
                    id: format!("id-{}", i),
                    name: format!("Project{}", i),
                    path: format!("/path/{}", i),
                })
                .collect();

            let target_id = projects[remove_idx].id.clone();
            let config_tmp = TempDir::new().unwrap();
            let config_manager = ConfigManager::with_path(config_tmp.path().join("config.json"));
            let state = AppState::new(AppConfig { projects });

            remove_project(&target_id, &state, &config_manager).unwrap();

            let remaining = list_projects(&state);
            prop_assert_eq!(remaining.len(), count - 1);
            prop_assert!(remaining.iter().all(|p| p.id != target_id));
        }

        // Feature: deployment-manager, Property 4: Path availability reflects file system state
        // **Validates: Requirements 1.5**
        #[test]
        fn path_availability_reflects_filesystem(
            existing_count in 1usize..4,
            missing_count in 1usize..4,
        ) {
            let tmp = TempDir::new().unwrap();
            let mut projects = Vec::new();

            // Create existing paths
            for i in 0..existing_count {
                let dir = tmp.path().join(format!("exists_{}", i));
                std::fs::create_dir_all(&dir).unwrap();
                projects.push(ProjectConfig {
                    id: format!("exists-{}", i),
                    name: format!("Exists{}", i),
                    path: dir.to_string_lossy().to_string(),
                });
            }

            // Add non-existing paths
            for i in 0..missing_count {
                projects.push(ProjectConfig {
                    id: format!("missing-{}", i),
                    name: format!("Missing{}", i),
                    path: format!("/nonexistent/path/{}", i),
                });
            }

            let state = AppState::new(AppConfig { projects });
            let listed = list_projects(&state);

            for p in &listed {
                if p.id.starts_with("exists") {
                    prop_assert!(p.available, "Expected available=true for {}", p.id);
                } else {
                    prop_assert!(!p.available, "Expected available=false for {}", p.id);
                }
            }
        }
    }
}
