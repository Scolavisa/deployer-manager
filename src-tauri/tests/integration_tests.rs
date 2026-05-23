use deployment_manager_lib::error::AppError;
use deployment_manager_lib::models::{AppConfig, ProjectConfig};
use deployment_manager_lib::services::config::ConfigManager;
use deployment_manager_lib::services::process;
use deployment_manager_lib::services::project;
use deployment_manager_lib::state::AppState;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

// =============================================================================
// Task 14.1: Config persistence integration tests
// =============================================================================

#[test]
fn integration_config_write_and_reload() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("subdir").join("config.json");
    let manager = ConfigManager::with_path(config_path.clone());

    let config = AppConfig {
        projects: vec![
            ProjectConfig {
                id: "proj-1".into(),
                name: "Project One".into(),
                path: "/home/user/project1".into(),
            },
            ProjectConfig {
                id: "proj-2".into(),
                name: "Project Two".into(),
                path: "/home/user/project2".into(),
            },
        ],
    };

    manager.save(&config).unwrap();
    assert!(config_path.exists());

    // Create a new manager pointing to the same path (simulates app restart)
    let manager2 = ConfigManager::with_path(config_path);
    let loaded = manager2.load();
    assert_eq!(loaded, config);
}

#[test]
fn integration_config_fresh_from_nonexistent_path() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("does_not_exist").join("config.json");
    let manager = ConfigManager::with_path(config_path);

    // Loading from a non-existent path should return default config
    let config = manager.load();
    assert_eq!(config, AppConfig::default());
    assert!(config.projects.is_empty());
}

#[test]
fn integration_config_corrupted_graceful_recovery() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("config.json");
    fs::write(&config_path, "this is not json!!!").unwrap();

    let manager = ConfigManager::with_path(config_path);
    let config = manager.load();
    assert_eq!(config, AppConfig::default());
}

// =============================================================================
// Task 14.2: Process execution integration tests
// =============================================================================

#[test]
fn integration_deploy_args_basic() {
    let path = Path::new("/home/user/project/.deployments/deploy.php");

    let args = process::build_deploy_args(path, "staging", None, None);
    assert_eq!(
        args,
        vec!["deploy", "-f", "/home/user/project/.deployments/deploy.php", "staging"]
    );
}

#[test]
fn integration_deploy_args_with_tag() {
    let path = Path::new("/home/user/project/.deployments/deploy.php");

    let args = process::build_deploy_args(path, "prod", Some("v1.11.0"), None);
    assert_eq!(
        args,
        vec![
            "deploy",
            "-f",
            "/home/user/project/.deployments/deploy.php",
            "prod",
            "--tag=v1.11.0"
        ]
    );
}

#[test]
fn integration_deploy_args_with_branch() {
    let path = Path::new("/home/user/project/.deployments/deploy.php");

    let args = process::build_deploy_args(path, "staging", None, Some("hotfix/packages-and-icons"));
    assert_eq!(
        args,
        vec![
            "deploy",
            "-f",
            "/home/user/project/.deployments/deploy.php",
            "staging",
            "--branch=hotfix/packages-and-icons"
        ]
    );
}

#[test]
fn integration_validate_deploy_options_both_none() {
    assert!(process::validate_deploy_options(&None, &None).is_ok());
}

#[test]
fn integration_validate_deploy_options_only_tag() {
    assert!(process::validate_deploy_options(&Some("v1.0".into()), &None).is_ok());
}

#[test]
fn integration_validate_deploy_options_only_branch() {
    assert!(process::validate_deploy_options(&None, &Some("main".into())).is_ok());
}

#[test]
fn integration_validate_deploy_options_both_set_errors() {
    let result = process::validate_deploy_options(&Some("v1.0".into()), &Some("main".into()));
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::TagBranchConflict => {}
        other => panic!("Expected TagBranchConflict, got: {:?}", other),
    }
}

#[test]
fn integration_releases_args_construction() {
    let path = Path::new("/project/.deployments/deploy.php");
    let args = process::build_releases_args(path, "staging");
    assert_eq!(
        args,
        vec!["-f", "/project/.deployments/deploy.php", "releases", "staging"]
    );
}

// =============================================================================
// Task 14.3: Project registration flow integration tests
// =============================================================================

#[test]
fn integration_project_registration_full_flow() {
    // Setup: create a valid project directory
    let project_tmp = TempDir::new().unwrap();
    let deploy_dir = project_tmp.path().join(".deployments");
    fs::create_dir_all(&deploy_dir).unwrap();
    fs::write(deploy_dir.join("deploy.php"), "<?php // deploy config").unwrap();

    // Setup: config manager with temp path
    let config_tmp = TempDir::new().unwrap();
    let config_manager = ConfigManager::with_path(config_tmp.path().join("config.json"));
    let state = AppState::new(AppConfig::default());

    // Register the project
    let registered = project::register_project(project_tmp.path(), &state, &config_manager).unwrap();
    assert!(!registered.id.is_empty());
    assert_eq!(registered.path, project_tmp.path().to_path_buf());
    assert!(registered.available);

    // Verify it appears in the list
    let projects = project::list_projects(&state);
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].id, registered.id);

    // Verify config persisted to disk
    let loaded_config = config_manager.load();
    assert_eq!(loaded_config.projects.len(), 1);
    assert_eq!(loaded_config.projects[0].id, registered.id);

    // Remove the project
    project::remove_project(&registered.id, &state, &config_manager).unwrap();

    // Verify it's gone from the list
    let projects = project::list_projects(&state);
    assert!(projects.is_empty());

    // Verify config on disk is updated
    let loaded_config = config_manager.load();
    assert!(loaded_config.projects.is_empty());
}

#[test]
fn integration_project_registration_invalid_path() {
    let config_tmp = TempDir::new().unwrap();
    let config_manager = ConfigManager::with_path(config_tmp.path().join("config.json"));
    let state = AppState::new(AppConfig::default());

    // Try to register a non-existent path
    let result = project::register_project(Path::new("/nonexistent/path"), &state, &config_manager);
    assert!(result.is_err());

    // Try to register a path without deploy.php
    let empty_tmp = TempDir::new().unwrap();
    let result = project::register_project(empty_tmp.path(), &state, &config_manager);
    assert!(result.is_err());

    // Verify nothing was added
    let projects = project::list_projects(&state);
    assert!(projects.is_empty());
}
