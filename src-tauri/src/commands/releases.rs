use std::process::Command;

use log::{info, error, debug};
use tauri::State;

use crate::error::AppError;
use crate::models::Release;
use crate::services::{process, project};
use crate::state::AppState;

#[tauri::command]
pub async fn get_releases(
    project_id: String,
    environment: String,
    state: State<'_, AppState>,
) -> Result<Vec<Release>, AppError> {
    let proj = project::get_project(&project_id, &state)?;
    let deploy_config = proj.path.join(".deployments").join("deploy.php");

    info!("Fetching releases for project '{}', env '{}', config: {:?}", proj.name, environment, deploy_config);
    let args = process::build_releases_args(&deploy_config, &environment);
    debug!("Running: dep {}", args.join(" "));

    let output = Command::new("dep")
        .args(&args)
        .current_dir(&proj.path)
        .output()
        .map_err(|e| {
            error!("Failed to execute dep releases: {}", e);
            AppError::ProcessError(format!("Failed to execute dep releases: {}", e))
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("dep releases failed (exit {}): {}", output.status, stderr);
        return Err(AppError::ProcessError(format!(
            "dep releases failed: {}",
            stderr
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let releases = parse_releases_output(&stdout);
    Ok(releases)
}

/// Parse the output of `dep releases` into Release structs.
/// Handles the table format output from PHP Deployer:
/// | Date (UTC)          | Release      | Author  | Target  | Commit |
/// | 2026-05-22 11:09:44 | 17 (current) | Michael | branch  | hash   |
pub fn parse_releases_output(output: &str) -> Vec<Release> {
    let mut releases: Vec<Release> = Vec::new();
    let mut is_header_row = true;

    for line in output.lines() {
        let trimmed = line.trim();

        // Skip empty lines and border lines (+---+)
        if trimmed.is_empty() || trimmed.starts_with('+') {
            continue;
        }

        // Skip lines that don't start with | (non-table output like "task releases")
        if !trimmed.starts_with('|') {
            continue;
        }

        // Split by | and collect columns
        let columns: Vec<&str> = trimmed
            .split('|')
            .map(|col| col.trim())
            .filter(|col| !col.is_empty())
            .collect();

        if columns.is_empty() {
            continue;
        }

        // Skip the header row (first row with | that contains "Date" or "Release")
        if is_header_row {
            is_header_row = false;
            continue;
        }

        // Parse data rows: Date | Release | Author | Target | Commit
        if columns.len() >= 2 {
            let date_str = columns[0].to_string();
            let release_col = columns[1].to_string();

            let is_current = release_col.contains("current");
            let name = release_col
                .replace("(current)", "")
                .trim()
                .to_string();

            if name.is_empty() {
                continue;
            }

            let target = if columns.len() >= 4 {
                Some(columns[3].to_string())
            } else {
                None
            };

            releases.push(Release {
                name,
                date: if date_str.is_empty() { None } else { Some(date_str) },
                is_current,
                target,
            });
        }
    }

    // Releases from dep are already in chronological order, reverse for most recent first
    releases.reverse();
    releases
}

/// Try to extract a date from a release name (typically a timestamp like 20240101120000)
#[allow(dead_code)]
fn extract_date_from_release_name(name: &str) -> Option<String> {
    // If the name looks like a timestamp (14+ digits), format it
    let digits: String = name.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() >= 14 {
        Some(format!(
            "{}-{}-{} {}:{}:{}",
            &digits[0..4],
            &digits[4..6],
            &digits[6..8],
            &digits[8..10],
            &digits[10..12],
            &digits[12..14]
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_parse_table_format() {
        let output = r#"task releases
+---------------------+--------------+---------+------ staging ------+------------------------------------------+
| Date (UTC)          | Release      | Author  | Target              | Commit                                   |
+---------------------+--------------+---------+---------------------+------------------------------------------+
| 2026-05-21 17:45:25 | 1            | Michael | master              | unknown                                  |
| 2026-05-22 08:07:42 | 3            | Michael | master              | unknown                                  |
| 2026-05-22 11:09:44 | 17 (current) | Michael | hotfix/packages     | 13c5dd4a46deb6d0d52418a36372f4f9c46b27b1 |
+---------------------+--------------+---------+---------------------+------------------------------------------+
"#;
        let releases = parse_releases_output(output);
        assert_eq!(releases.len(), 3);

        // Reversed: most recent first
        assert_eq!(releases[0].name, "17");
        assert!(releases[0].is_current);
        assert_eq!(releases[0].date, Some("2026-05-22 11:09:44".to_string()));
        assert_eq!(releases[0].target, Some("hotfix/packages".to_string()));

        assert_eq!(releases[1].name, "3");
        assert!(!releases[1].is_current);

        assert_eq!(releases[2].name, "1");
        assert_eq!(releases[2].date, Some("2026-05-21 17:45:25".to_string()));
    }

    #[test]
    fn test_parse_releases_output_empty() {
        let output = "";
        let releases = parse_releases_output(output);
        assert!(releases.is_empty());
    }

    #[test]
    fn test_parse_releases_only_borders() {
        let output = "+---+\n+---+\n";
        let releases = parse_releases_output(output);
        assert!(releases.is_empty());
    }

    #[test]
    fn test_parse_releases_current_marker() {
        let output = r#"+---+
| Date (UTC)          | Release      | Author  | Target | Commit |
+---+
| 2026-05-22 09:27:25 | 15 (current) | Michael | main   | abc123 |
| 2026-05-22 09:26:49 | 14           | Michael | main   | def456 |
+---+
"#;
        let releases = parse_releases_output(output);
        assert_eq!(releases.len(), 2);
        // Reversed
        assert_eq!(releases[0].name, "14");
        assert!(!releases[0].is_current);
        assert_eq!(releases[1].name, "15");
        assert!(releases[1].is_current);
    }

    proptest! {
        // Feature: deployment-manager, Property 10: Releases are sorted reverse chronologically
        // **Validates: Requirements 5.2**
        #[test]
        fn releases_reversed_from_input_order(count in 2usize..10) {
            // Build a table with releases in chronological order (as dep outputs)
            let mut table = String::from("+---+\n| Date | Release | Author | Target | Commit |\n+---+\n");
            for i in 1..=count {
                table.push_str(&format!(
                    "| 2026-05-{:02} 10:00:00 | {}           | Dev    | main   | abc    |\n",
                    i, i
                ));
            }
            table.push_str("+---+\n");

            let releases = parse_releases_output(&table);
            prop_assert_eq!(releases.len(), count);

            // Should be reversed (most recent first)
            for i in 0..releases.len() - 1 {
                let current: u32 = releases[i].name.trim().parse().unwrap();
                let next: u32 = releases[i + 1].name.trim().parse().unwrap();
                prop_assert!(current > next, "Expected {} > {}", current, next);
            }
        }
    }
}
