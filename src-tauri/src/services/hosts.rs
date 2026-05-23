use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

use crate::error::AppError;
use crate::models::Environment;

/// Raw host entry as deserialized from YAML
#[derive(Debug, Deserialize)]
struct HostEntry {
    hostname: Option<String>,
    remote_user: Option<String>,
    deploy_path: Option<String>,
    branch: Option<String>,
    stage: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    keep_releases: Option<u32>,
}

/// Parse a hosts.yaml file and extract all environments
pub fn parse_hosts_file(path: &Path) -> Result<Vec<Environment>, AppError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| AppError::HostsParseError(format!("Failed to read hosts file: {}", e)))?;
    parse_hosts_yaml(&content)
}

/// Wrapper struct for hosts.yaml files that have a top-level "hosts:" key
#[derive(Debug, Deserialize)]
struct HostsWrapper {
    hosts: HashMap<String, HostEntry>,
}

/// Parse hosts YAML content string into environments.
/// Supports two formats:
/// 1. Top-level "hosts:" key wrapping environments (PHP Deployer standard)
/// 2. Environments directly at the top level
pub fn parse_hosts_yaml(content: &str) -> Result<Vec<Environment>, AppError> {
    // First try: parse with a top-level "hosts:" key (PHP Deployer standard format)
    let hosts = if let Ok(wrapper) = serde_yaml::from_str::<HostsWrapper>(content) {
        wrapper.hosts
    } else {
        // Fallback: parse as a flat map of environment_name -> host_entry
        serde_yaml::from_str::<HashMap<String, HostEntry>>(content)
            .map_err(|e| AppError::HostsParseError(format!("Invalid YAML: {}", e)))?
    };

    if hosts.is_empty() {
        return Err(AppError::HostsParseError(
            "No environments found in hosts config".into(),
        ));
    }

    let mut environments: Vec<Environment> = hosts
        .into_iter()
        .map(|(name, entry)| Environment {
            name,
            hostname: entry.hostname.unwrap_or_default(),
            remote_user: entry.remote_user.unwrap_or_default(),
            deploy_path: entry.deploy_path.unwrap_or_default(),
            branch: entry.branch,
            stage: entry.stage,
            keep_releases: entry.keep_releases,
        })
        .collect();

    // Sort alphabetically by name for consistent ordering
    environments.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(environments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wrapped_hosts_yaml() {
        let yaml = r#"
hosts:
  prod:
    hostname: 57.129.13.26
    remote_user: micksp
    deploy_path: /var/www/deployments/prod/ClassE
    branch: master
    keep_releases: 3
    stage: prod
  staging:
    hostname: 57.129.13.26
    remote_user: micksp
    deploy_path: /var/www/deployments/staging/ClassE
    branch: master
    keep_releases: 3
    stage: staging
"#;

        let envs = parse_hosts_yaml(yaml).unwrap();
        assert_eq!(envs.len(), 2);

        let prod = envs.iter().find(|e| e.name == "prod").unwrap();
        assert_eq!(prod.hostname, "57.129.13.26");
        assert_eq!(prod.remote_user, "micksp");
        assert_eq!(prod.deploy_path, "/var/www/deployments/prod/ClassE");
        assert_eq!(prod.branch, Some("master".to_string()));
        assert_eq!(prod.stage, Some("prod".to_string()));

        let staging = envs.iter().find(|e| e.name == "staging").unwrap();
        assert_eq!(staging.hostname, "57.129.13.26");
        assert_eq!(staging.stage, Some("staging".to_string()));
    }

    #[test]
    fn test_parse_valid_hosts_yaml() {
        let yaml = r#"
production:
  hostname: prod.example.com
  remote_user: deploy
  deploy_path: /var/www/app
  branch: main
  stage: prod

staging:
  hostname: staging.example.com
  remote_user: deploy
  deploy_path: /var/www/staging
  stage: staging
"#;

        let envs = parse_hosts_yaml(yaml).unwrap();
        assert_eq!(envs.len(), 2);

        let prod = envs.iter().find(|e| e.name == "production").unwrap();
        assert_eq!(prod.hostname, "prod.example.com");
        assert_eq!(prod.remote_user, "deploy");
        assert_eq!(prod.deploy_path, "/var/www/app");
        assert_eq!(prod.branch, Some("main".to_string()));
        assert_eq!(prod.stage, Some("prod".to_string()));

        let staging = envs.iter().find(|e| e.name == "staging").unwrap();
        assert_eq!(staging.hostname, "staging.example.com");
        assert_eq!(staging.remote_user, "deploy");
        assert_eq!(staging.deploy_path, "/var/www/staging");
        assert_eq!(staging.branch, None);
        assert_eq!(staging.stage, Some("staging".to_string()));
    }

    #[test]
    fn test_parse_missing_optional_fields() {
        let yaml = r#"
dev:
  hostname: dev.example.com
  remote_user: devuser
  deploy_path: /var/www/dev
"#;

        let envs = parse_hosts_yaml(yaml).unwrap();
        assert_eq!(envs.len(), 1);

        let dev = &envs[0];
        assert_eq!(dev.name, "dev");
        assert_eq!(dev.hostname, "dev.example.com");
        assert_eq!(dev.branch, None);
        assert_eq!(dev.stage, None);
    }

    #[test]
    fn test_parse_invalid_yaml() {
        let yaml = "not: valid: yaml: [[[";
        let result = parse_hosts_yaml(yaml);
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::HostsParseError(msg) => assert!(msg.contains("Invalid YAML")),
            _ => panic!("Expected HostsParseError"),
        }
    }

    #[test]
    fn test_parse_empty_yaml() {
        let yaml = "{}";
        let result = parse_hosts_yaml(yaml);
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::HostsParseError(msg) => {
                assert!(msg.contains("No environments found"))
            }
            _ => panic!("Expected HostsParseError"),
        }
    }

    #[test]
    fn test_parse_hosts_file_not_found() {
        let path = Path::new("/nonexistent/path/hosts.yaml");
        let result = parse_hosts_file(path);
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::HostsParseError(msg) => {
                assert!(msg.contains("Failed to read hosts file"))
            }
            _ => panic!("Expected HostsParseError"),
        }
    }

    use proptest::prelude::*;

    // Strategy for valid environment names (alphanumeric, no special chars)
    #[allow(dead_code)]
    fn arb_env_name() -> impl Strategy<Value = String> {
        "[a-z][a-z0-9]{2,10}".prop_map(|s| s)
    }

    // Strategy for valid hostname
    #[allow(dead_code)]
    fn arb_hostname() -> impl Strategy<Value = String> {
        "[a-z]{3,8}\\.[a-z]{2,5}\\.[a-z]{2,3}".prop_map(|s| s)
    }

    proptest! {
        // Feature: deployment-manager, Property 5: Hosts parsing extracts all environments
        // **Validates: Requirements 3.1**
        #[test]
        fn hosts_parsing_extracts_all_environments(
            env_count in 1usize..5,
        ) {
            // Generate unique environment names
            let env_names: Vec<String> = (0..env_count)
                .map(|i| format!("env{}", i))
                .collect();

            // Build YAML string
            let mut yaml = String::new();
            for name in &env_names {
                yaml.push_str(&format!(
                    "{}:\n  hostname: {}.example.com\n  remote_user: deploy\n  deploy_path: /var/www/{}\n\n",
                    name, name, name
                ));
            }

            let result = parse_hosts_yaml(&yaml).unwrap();
            prop_assert_eq!(result.len(), env_count);

            // Verify all environment names are present
            let result_names: Vec<&str> = result.iter().map(|e| e.name.as_str()).collect();
            for name in &env_names {
                prop_assert!(result_names.contains(&name.as_str()));
            }
        }

        // Feature: deployment-manager, Property 6: Invalid YAML produces parse error
        // **Validates: Requirements 3.3**
        #[test]
        fn invalid_yaml_produces_parse_error(
            input in "[^a-zA-Z0-9:\\s]{5,50}"
        ) {
            let result = parse_hosts_yaml(&input);
            prop_assert!(result.is_err());
        }
    }
}
