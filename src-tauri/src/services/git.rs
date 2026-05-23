use std::path::Path;
use std::process::Command;

use crate::error::AppError;

/// Get all git tags from a repository directory
pub fn get_tags(project_path: &Path) -> Result<Vec<String>, AppError> {
    let output = Command::new("git")
        .args(["tag", "-l"])
        .current_dir(project_path)
        .output()
        .map_err(|e| AppError::GitError(format!("Failed to execute git tag: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::GitError(format!("git tag failed: {}", stderr)));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let tags = parse_git_output(&stdout);
    Ok(tags)
}

/// Get all git branches (local and remote) from a repository directory
pub fn get_branches(project_path: &Path) -> Result<Vec<String>, AppError> {
    let output = Command::new("git")
        .args(["branch", "-a", "--format=%(refname:short)"])
        .current_dir(project_path)
        .output()
        .map_err(|e| AppError::GitError(format!("Failed to execute git branch: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::GitError(format!("git branch failed: {}", stderr)));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let branches = parse_git_output(&stdout);
    Ok(branches)
}

/// Parse git command output into a list of trimmed, non-empty lines
pub fn parse_git_output(output: &str) -> Vec<String> {
    output
        .lines()
        .map(|line| line.trim().trim_start_matches("* ").to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_git_output_multiple_lines() {
        let output = "v1.0.0\nv1.1.0\nv2.0.0\n";
        let result = parse_git_output(output);
        assert_eq!(result, vec!["v1.0.0", "v1.1.0", "v2.0.0"]);
    }

    #[test]
    fn test_parse_git_output_empty() {
        let output = "";
        let result = parse_git_output(output);
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_git_output_with_whitespace() {
        let output = "  main  \n  develop  \n  feature/test  \n";
        let result = parse_git_output(output);
        assert_eq!(result, vec!["main", "develop", "feature/test"]);
    }

    #[test]
    fn test_parse_git_output_with_star_prefix() {
        let output = "* main\n  develop\n  feature/test\n";
        let result = parse_git_output(output);
        assert_eq!(result, vec!["main", "develop", "feature/test"]);
    }

    #[test]
    fn test_parse_git_output_with_empty_lines() {
        let output = "tag1\n\ntag2\n\n";
        let result = parse_git_output(output);
        assert_eq!(result, vec!["tag1", "tag2"]);
    }
}
