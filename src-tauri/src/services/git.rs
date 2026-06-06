use std::path::Path;
use std::process::Command;

use semver::Version;

use crate::error::AppError;
use crate::services::process;

/// Get all git tags from a repository directory
pub fn get_tags(project_path: &Path) -> Result<Vec<String>, AppError> {
    let git_path = process::resolve_git_path();
    let output = Command::new(&git_path)
        .args(["tag", "-l"])
        .current_dir(project_path)
        .output()
        .map_err(|e| AppError::GitError(format!("Failed to execute git tag: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::GitError(format!("git tag failed: {}", stderr)));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut tags = parse_git_output(&stdout);
    sort_descending(&mut tags);
    Ok(tags)
}

fn sort_descending(values: &mut [String]) {
    values.sort_by(|a, b| compare_tags_desc(a, b));
}

fn compare_tags_desc(a: &str, b: &str) -> std::cmp::Ordering {
    match (parse_tag_version(a), parse_tag_version(b)) {
        (Some(a_version), Some(b_version)) => b_version.cmp(&a_version),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => b.cmp(a),
    }
}

fn parse_tag_version(tag: &str) -> Option<Version> {
    // Support both plain semver tags (1.2.3) and common prefixed tags (v1.2.3).
    Version::parse(tag.trim_start_matches('v')).ok()
}

/// Get remote git branches from a repository directory.
/// Only remote branches are shown because PHP Deployer checks out branches on
/// the remote server, so local-only branches cannot be deployed.
pub fn get_branches(project_path: &Path) -> Result<Vec<String>, AppError> {
    let git_path = process::resolve_git_path();
    let output = Command::new(&git_path)
        .args(["branch", "-r", "--format=%(refname:short)"])
        .current_dir(project_path)
        .output()
        .map_err(|e| AppError::GitError(format!("Failed to execute git branch: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::GitError(format!("git branch failed: {}", stderr)));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut branches = parse_remote_branches(&stdout);

    branches.sort();
    branches.dedup();

    Ok(branches)
}

/// Parse `git branch -r --format=%(refname:short)` output into branch names.
/// Each line is `<remote>/<branch>` (e.g. `origin/master`). We split on the
/// first `/` to strip the remote name. Entries without `/` (e.g. bare `origin`
/// when git shortens `origin/HEAD`) and `HEAD` refs are excluded.
fn parse_remote_branches(output: &str) -> Vec<String> {
    output
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            // Split "origin/branch-name" → ("origin", "branch-name")
            let (_, branch) = trimmed.split_once('/')?;
            if branch == "HEAD" {
                return None;
            }
            Some(branch.to_string())
        })
        .collect()
}

/// Parse git command output into a list of trimmed, non-empty lines
pub fn parse_git_output(output: &str) -> Vec<String> {
    output
        .lines()
        .map(|line| {
            line.trim()
                .trim_start_matches("* ")
                .trim_start_matches("origin/")
                .to_string()
        })
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
    fn test_parse_git_output_with_origin_prefix() {
        let output = "origin/main\norigin/develop\norigin/feature/test\n";
        let result = parse_git_output(output);
        assert_eq!(result, vec!["main", "develop", "feature/test"]);
    }

    #[test]
    fn test_parse_git_output_with_origin_and_star_prefix() {
        let output = "* origin/main\n  origin/develop\n  origin/feature/test\n";
        let result = parse_git_output(output);
        assert_eq!(result, vec!["main", "develop", "feature/test"]);
    }

    #[test]
    fn test_parse_git_output_with_empty_lines() {
        let output = "tag1\n\ntag2\n\n";
        let result = parse_git_output(output);
        assert_eq!(result, vec!["tag1", "tag2"]);
    }

    #[test]
    fn test_sort_descending_tags_semver() {
        let mut tags = vec![
            "v1.0.0".to_string(),
            "v2.0.0".to_string(),
            "v1.1.0".to_string(),
            "v1.10.0".to_string(),
        ];

        sort_descending(&mut tags);

        assert_eq!(tags, vec!["v2.0.0", "v1.10.0", "v1.1.0", "v1.0.0"]);
    }

    #[test]
    fn test_sort_descending_tags_mixed_semver_and_non_semver() {
        let mut tags = vec![
            "release-candidate".to_string(),
            "v1.2.0".to_string(),
            "v1.10.0".to_string(),
            "nightly".to_string(),
        ];

        sort_descending(&mut tags);

        assert_eq!(
            tags,
            vec!["v1.10.0", "v1.2.0", "release-candidate", "nightly"]
        );
    }

    #[test]
    fn test_parse_remote_branches_filters_head() {
        let output = "origin/HEAD\norigin/master\norigin/develop\n";
        let result = parse_remote_branches(output);
        assert_eq!(result, vec!["master", "develop"]);
    }

    #[test]
    fn test_parse_remote_branches_bare_origin_excluded() {
        // Some git versions shorten origin/HEAD to just "origin" (no slash).
        // split_once('/') returns None for bare "origin", so it's excluded.
        let output = "origin\norigin/master\norigin/develop\n";
        let result = parse_remote_branches(output);
        assert_eq!(result, vec!["master", "develop"]);
    }

    #[test]
    fn test_parse_remote_branches_preserves_slashes_in_branch_name() {
        // feature/login contains a slash — only the first slash (remote separator) is stripped
        let output = "origin/master\norigin/feature/login\n";
        let result = parse_remote_branches(output);
        assert_eq!(result, vec!["master", "feature/login"]);
    }

    #[test]
    fn test_parse_remote_branches_dedup_after_sort() {
        let output = "origin/master\norigin/master\norigin/develop\n";
        let mut branches = parse_remote_branches(output);
        branches.sort();
        branches.dedup();
        assert_eq!(branches, vec!["develop", "master"]);
    }

    #[test]
    fn test_parse_remote_branches_typical_output() {
        let output = "origin/HEAD\norigin/master\norigin/develop\norigin/feature/login\n";
        let mut branches = parse_remote_branches(output);
        branches.sort();
        branches.dedup();
        assert_eq!(branches, vec!["develop", "feature/login", "master"]);
    }
}
