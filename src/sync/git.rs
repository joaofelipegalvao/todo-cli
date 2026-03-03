//! Git operations over the rustodo data directory.
//!
//! All Git commands are executed via [`std::process::Command`], delegating
//! authentication and configuration to the user's existing Git installation.
//!
//! The repository lives in the same directory as `todos.json`:
//!
//! ```text
//! ~/.local/share/rustodo/
//!   .git/
//!   todos.json
//!   sync.toml
//! ```
//!
//! # Workflow
//!
//! ```text
//! todo sync init <remote>  →  git init + git remote add origin + initial commit
//! todo sync push           →  git add todos.json + git commit -m "..." + git push
//! todo sync pull           →  git fetch + semantic merge + git rebase FETCH_HEAD
//! todo sync status         →  git status + git log summary
//! ```

use anyhow::{Context, Result, bail};

use std::path::Path;
use std::process::Command;

/// Runs a git command in the given working directory.
///
/// Returns stdout on success, or an error with stderr included.
fn git(dir: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .context("Failed to run git — is it installed and on PATH?")?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        bail!("git {} failed:\n{}", args.join(" "), stderr)
    }
}

/// Checks whether git is available on PATH.
pub fn check_git_available() -> Result<()> {
    Command::new("git")
        .arg("--version")
        .output()
        .context("git is not installed or not on PATH")?;
    Ok(())
}

/// Initializes a Git repository in `dir` if one does not already exist.
pub fn init(dir: &Path) -> Result<()> {
    if dir.join(".git").exists() {
        return Ok(());
    }
    git(dir, &["init", "-b", "main"])
        .or_else(|_| git(dir, &["init"]))
        .context("Failed to initialize git repository")?;

    std::fs::write(dir.join(".gitignore"), "sync.toml\n*.lock\n*.tmp\n")
        .context("Failed to create .gitignore")?;

    Ok(())
}

/// Adds a remote named `origin`. No-ops if `origin` already exists.
pub fn add_remote(dir: &Path, url: &str) -> Result<()> {
    // Check if origin already exists
    let remotes = git(dir, &["remote"])?;
    if remotes.lines().any(|r| r.trim() == "origin") {
        // Update URL in case user is reconfiguring
        git(dir, &["remote", "set-url", "origin", url])?;
    } else {
        git(dir, &["remote", "add", "origin", url])?;
    }
    Ok(())
}

/// Stages `todos.json` and creates an initial commit.
///
/// Creates `todos.json` with an empty task list if it does not exist yet.
/// No-ops if there is nothing to commit.
pub fn initial_commit(dir: &Path) -> Result<()> {
    let has_commits = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(dir)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if has_commits {
        return Ok(());
    }

    let todos_path = dir.join("todos.json");
    if !todos_path.exists() {
        std::fs::write(&todos_path, "[]\n").context("Failed to create todos.json")?;
    }

    let gitignore = dir.join(".gitignore");
    if gitignore.exists() {
        git(dir, &["add", ".gitignore"])?;
    }
    git(dir, &["add", "todos.json"])?;

    // Check if there is anything staged
    let status = git(dir, &["status", "--porcelain"])?;
    if status.is_empty() {
        return Ok(());
    }

    git(dir, &["commit", "-m", "sync: initial commit"])?;
    Ok(())
}

/// Stages `todos.json` and creates a descriptive commit.
///
/// `message` is built by the caller from diff statistics.
/// No-ops if `todos.json` has no changes since the last commit.
pub fn commit(dir: &Path, message: &str) -> Result<bool> {
    git(dir, &["add", "todos.json"])?;

    let status = git(dir, &["status", "--porcelain", "todos.json"])?;
    if status.is_empty() {
        return Ok(false);
    }

    let output = Command::new("git")
        .args(["commit", "-m", message])
        .current_dir(dir)
        .output()
        .context("Failed to run git commit")?;

    if output.status.success() {
        Ok(true)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        bail!("git commit failed:\n{}", stderr)
    }
}

/// Reads todos.json from the last commit and returns its content.
///
/// Returns `None` if there are no commits yet.
pub fn last_committed_tasks_json(dir: &Path) -> Option<String> {
    git(dir, &["show", "HEAD:todos.json"]).ok()
}

/// Returns (ahead, behind) commit counts relative to origin.
///
/// Returns (0, 0) if the remote tracking branch doesn't exist yet.
pub fn ahead_behind(dir: &Path) -> (usize, usize) {
    let output = match git(dir, &["rev-list", "--left-right", "--count", "HEAD...@{u}"]) {
        Ok(s) => s,
        Err(_) => return (0, 0),
    };

    let mut parts = output.split_whitespace();
    let ahead = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let behind = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (ahead, behind)
}

/// Pushes the current branch to `origin`.
pub fn push(dir: &Path) -> Result<()> {
    git(dir, &["push", "--set-upstream", "origin", "HEAD"]).context("Failed to push to remote")?;
    Ok(())
}

/// Fetches from remote without merging.
pub fn fetch(dir: &Path) -> Result<()> {
    git(dir, &["fetch", "origin"]).context("Failed to fetch from remote")?;
    Ok(())
}

/// Reads todos.json from FETCH_HEAD — the pure remote state before any merge.
///
/// Returns `None` if FETCH_HEAD does not exist yet.
pub fn fetch_head_tasks_json(dir: &Path) -> Result<Option<String>> {
    match git(dir, &["show", "FETCH_HEAD:todos.json"]) {
        Ok(s) => Ok(Some(s)),
        Err(_) => Ok(None),
    }
}

pub enum PullResult {
    /// Remote is identical to local — nothing to do.
    UpToDate,
    /// Fetch + merge succeeded. Carries the remote todos.json captured
    /// *before* `git merge` ran, so it is always a clean, non-concatenated list.
    Merged(String),
    /// Git detected a conflict in todos.json — caller resolves via :2/:3.
    Conflict(String),
}

/// Fetches from remote, captures the remote todos.json **before** merging,
/// then runs `git merge FETCH_HEAD`.
///
/// Separating fetch from merge lets the semantic merge in sync.rs receive
/// two clean task lists, preventing duplicates caused by git's naive
/// JSON array concatenation.
pub fn pull(dir: &Path) -> Result<PullResult> {
    // 1. Fetch only — no merge yet
    fetch(dir)?;

    // 2. Capture remote state before any merge happens
    let remote_json = match fetch_head_tasks_json(dir)? {
        Some(j) => j,
        None => return Ok(PullResult::UpToDate),
    };

    // 3. Rebase local branch on top of FETCH_HEAD
    let result = git(dir, &["rebase", "FETCH_HEAD"]);

    match result {
        Ok(_) => Ok(PullResult::Merged(remote_json)),
        Err(_) => {
            let status = git(dir, &["status", "--porcelain"])?;
            if status.contains("todos.json") {
                Ok(PullResult::Conflict(remote_json))
            } else {
                Err(anyhow::anyhow!("Merge failed for unknown reason"))
            }
        }
    }
}

pub fn read_ours(dir: &Path) -> Result<String> {
    git(dir, &["show", ":2:todos.json"]).context("Failed to read local version of todos.json")
}

pub fn read_theirs(dir: &Path) -> Result<String> {
    git(dir, &["show", ":3:todos.json"]).context("Failed to read remote version of todos.json")
}

pub fn finish_merge(dir: &Path) -> Result<()> {
    git(dir, &["add", "todos.json"])?;
    git(dir, &["commit", "--no-edit"])?;
    Ok(())
}

/// Returns a short status summary.
///
/// Shows the current branch, last commit, and whether there are
/// uncommitted changes to `todos.json`.
pub fn status(dir: &Path) -> Result<String> {
    let branch =
        git(dir, &["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or_else(|_| "unknown".to_string());

    let last_commit = git(dir, &["log", "-1", "--format=%h %s (%cr)"])
        .unwrap_or_else(|_| "no commits yet".to_string());

    let dirty = git(dir, &["status", "--porcelain"])?;
    let todos_changed = dirty.lines().any(|l| l.contains("todos.json"));

    let (ahead, behind) = ahead_behind(dir);
    let ahead_behind_str = format!("↑{} ↓{}", ahead, behind);

    let mut lines = vec![
        format!("Branch:       {}", branch),
        format!("Last commit:  {}", last_commit),
        format!("Ahead/behind: {}", ahead_behind_str),
    ];

    if todos_changed {
        lines.push("todos.json:   modified (not yet committed)".to_string());
    } else {
        lines.push("todos.json:   clean".to_string());
    }

    Ok(lines.join("\n"))
}

/// Returns true if the data directory contains an initialized git repo.
pub fn is_initialized(dir: &Path) -> bool {
    dir.join(".git").exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_repo() -> TempDir {
        let temp = TempDir::new().unwrap();
        let dir = temp.path();

        // Configure git identity for the test repo
        git(dir, &["init", "-b", "main"])
            .or_else(|_| git(dir, &["init"]))
            .unwrap();
        git(dir, &["config", "user.email", "test@rustodo"]).unwrap();
        git(dir, &["config", "user.name", "rustodo-test"]).unwrap();

        temp
    }

    #[test]
    fn test_check_git_available() {
        assert!(check_git_available().is_ok());
    }

    #[test]
    fn test_init_creates_git_dir() {
        let temp = TempDir::new().unwrap();
        init(temp.path()).unwrap();
        assert!(temp.path().join(".git").exists());
    }

    #[test]
    fn test_init_is_idempotent() {
        let temp = TempDir::new().unwrap();
        init(temp.path()).unwrap();
        // Second call should not fail
        init(temp.path()).unwrap();
    }

    #[test]
    fn test_commit_no_changes_returns_false() {
        let temp = setup_repo();
        let dir = temp.path();

        fs::write(dir.join("todos.json"), "[]").unwrap();
        git(dir, &["add", "todos.json"]).unwrap();
        git(dir, &["commit", "-m", "initial"]).unwrap();

        // No changes — commit should return false
        let committed = commit(dir, "sync: no changes").unwrap();
        assert!(!committed);
    }

    #[test]
    fn test_commit_with_changes_returns_true() {
        let temp = setup_repo();
        let dir = temp.path();

        fs::write(dir.join("todos.json"), "[]").unwrap();
        git(dir, &["add", "todos.json"]).unwrap();
        git(dir, &["commit", "-m", "initial"]).unwrap();

        // Modify file
        fs::write(dir.join("todos.json"), "[{}]").unwrap();

        let committed = commit(dir, "sync: 1 task added").unwrap();
        assert!(committed);
    }

    #[test]
    fn test_is_initialized() {
        let temp = TempDir::new().unwrap();
        assert!(!is_initialized(temp.path()));
        init(temp.path()).unwrap();
        assert!(is_initialized(temp.path()));
    }

    #[test]
    fn test_status_after_init_and_commit() {
        let temp = setup_repo();
        let dir = temp.path();

        fs::write(dir.join("todos.json"), "[]").unwrap();
        git(dir, &["add", "todos.json"]).unwrap();
        git(dir, &["commit", "-m", "initial"]).unwrap();

        let s = status(dir).unwrap();
        assert!(s.contains("todos.json:   clean"));
        assert!(s.contains("Last commit:"));
    }

    #[test]
    fn test_status_shows_dirty_when_modified() {
        let temp = setup_repo();
        let dir = temp.path();

        fs::write(dir.join("todos.json"), "[]").unwrap();
        git(dir, &["add", "todos.json"]).unwrap();
        git(dir, &["commit", "-m", "initial"]).unwrap();

        // Modify without committing
        fs::write(dir.join("todos.json"), "[{}]").unwrap();

        let s = status(dir).unwrap();
        assert!(s.contains("todos.json:   modified"));
    }
}
