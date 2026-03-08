//! Handler for `todo sync` subcommands.
//!
//! Dispatches to four subcommands:
//!
//! | Subcommand | Description |
//! |---|---|
//! | `init <remote>` | Initialize Git repo and configure remote |
//! | `push` | Commit changes and push to remote |
//! | `pull` | Pull from remote and merge (Phase 2) |
//! | `status` | Show sync state |

use anyhow::{Context, Result, bail};
use colored::Colorize;

use crate::storage::{Storage, get_data_file_path, json};
use crate::sync::{config, git};

/// Subcommand variants for `todo sync`.
pub enum SyncCommand {
    Init { remote: String },
    Push,
    Pull,
    Status,
}

/// Executes a sync subcommand.
pub fn execute(storage: &impl Storage, cmd: SyncCommand) -> Result<()> {
    match cmd {
        SyncCommand::Init { remote } => init(storage, &remote),
        SyncCommand::Push => push(storage),
        SyncCommand::Pull => pull(storage),
        SyncCommand::Status => status(),
    }
}

// ── init ─────────────────────────────────────────────────────────────────────

fn init(_storage: &impl Storage, remote: &str) -> Result<()> {
    git::check_git_available()?;

    let data_dir = get_data_file_path()?
        .parent()
        .expect("todos.json always has a parent directory")
        .to_path_buf();

    // 1. git init (idempotent)
    git::init(&data_dir)?;
    println!("{} Git repository initialized", "✓".green());

    // 2. git remote add origin <url>
    git::add_remote(&data_dir, remote)?;
    println!("{} Remote set to {}", "✓".green(), remote.cyan());

    // 3. Initial commit of todos.json
    git::initial_commit(&data_dir)?;
    println!("{} todos.json committed", "✓".green());

    // 4. Persist remote URL to sync.toml
    config::save(&config::SyncConfig {
        remote: remote.to_string(),
    })?;
    println!("{} Saved sync config", "✓".green());

    println!();
    println!(
        "{}  Run {} to push your tasks.",
        "→".dimmed(),
        "todo sync push".bright_white()
    );

    Ok(())
}

// ── push ─────────────────────────────────────────────────────────────────────

fn push(storage: &impl Storage) -> Result<()> {
    git::check_git_available()?;
    let _cfg = config::require()?;

    let data_dir = get_data_file_path()?
        .parent()
        .expect("todos.json always has a parent directory")
        .to_path_buf();

    if !git::is_initialized(&data_dir) {
        bail!("Git repository not initialized. Run: todo sync init <remote>");
    }

    let current_tasks = storage.load()?;
    let message = build_commit_message(&current_tasks, &data_dir);
    let committed = git::commit(&data_dir, &message)?;

    if committed {
        println!("{} Committed: {}", "✓".green(), message.dimmed());
    } else {
        println!("{} Nothing to commit — todos.json is unchanged", "".blue());
    }

    git::push(&data_dir)?;
    println!("{} Pushed to remote", "✓".green());

    Ok(())
}

/// Builds a commit message by diffing current tasks against the last commit.
///
/// Compares UUIDs to detect added, removed, and completed tasks.
/// Falls back to a simple count message if there is no previous commit.
fn build_commit_message(current: &[crate::models::Task], data_dir: &std::path::Path) -> String {
    use std::collections::HashMap;
    use uuid::Uuid;

    let Some(prev_json) = git::last_committed_tasks_json(data_dir) else {
        // No previous commit — first push
        let done = current.iter().filter(|t| t.completed).count();
        let pending = current.len() - done;
        return format!(
            "sync: {} tasks ({} pending, {} done)",
            current.len(),
            pending,
            done
        );
    };

    let Ok(prev_tasks) = serde_json::from_str::<Vec<crate::models::Task>>(&prev_json) else {
        // Can't parse previous — fall back to counts
        let done = current.iter().filter(|t| t.completed).count();
        let pending = current.len() - done;
        return format!(
            "sync: {} tasks ({} pending, {} done)",
            current.len(),
            pending,
            done
        );
    };

    let prev_map: HashMap<Uuid, &crate::models::Task> =
        prev_tasks.iter().map(|t| (t.uuid, t)).collect();
    let curr_map: HashMap<Uuid, &crate::models::Task> =
        current.iter().map(|t| (t.uuid, t)).collect();

    let added = curr_map
        .keys()
        .filter(|id| !prev_map.contains_key(id))
        .count();
    let removed = prev_map
        .keys()
        .filter(|id| !curr_map.contains_key(id))
        .count();
    let completed = curr_map
        .iter()
        .filter(|(id, t)| t.completed && prev_map.get(id).map(|p| !p.completed).unwrap_or(false))
        .count();
    let uncompleted = curr_map
        .iter()
        .filter(|(id, t)| !t.completed && prev_map.get(id).map(|p| p.completed).unwrap_or(false))
        .count();

    let mut parts = vec![];
    if added > 0 {
        parts.push(format!("+{} added", added));
    }
    if removed > 0 {
        parts.push(format!("-{} removed", removed));
    }
    if completed > 0 {
        parts.push(format!("{} completed", completed));
    }
    if uncompleted > 0 {
        parts.push(format!("{} reopened", uncompleted));
    }

    if parts.is_empty() {
        "sync: metadata updated".to_string()
    } else {
        format!("sync: {}", parts.join(", "))
    }
}

// ── pull ─────────────────────────────────────────────────────────────────────

fn pull(storage: &impl Storage) -> Result<()> {
    git::check_git_available()?;
    let _cfg = config::require()?;

    let data_dir = get_data_file_path()?
        .parent()
        .expect("todos.json always has a parent directory")
        .to_path_buf();

    if !git::is_initialized(&data_dir) {
        bail!("Git repository not initialized. Run: todo sync init <remote>");
    }

    // 1. Snapshot local state before pull
    let local_tasks = storage.load()?;

    // 2. Commit any pending local changes before rebase
    let message = build_commit_message(&local_tasks, &data_dir);
    git::commit(&data_dir, &message)?;

    // 3. Pull from remote
    println!("{}", "Pulling from remote…".dimmed());

    match git::pull(&data_dir)? {
        git::PullResult::UpToDate => {
            println!("  {} Already up to date", "".blue());
        }

        // remote_json capturado antes do git merge — lista limpa, sem concatenação
        git::PullResult::Merged(remote_json) => {
            // 4. Parse do estado puro do remote (suporta v1 bare array e v2 envelope)
            let remote_tasks = json::parse_tasks_from_str(&remote_json)
                .context("Failed to parse remote todos.json")?;

            // 5. Semantic merge: local vs remote (ambos limpos)
            let result = crate::sync::merge::merge(local_tasks, remote_tasks);

            // 6. Salva e commita
            storage.save(&result.tasks)?;
            git::commit(&data_dir, "sync: merge")?;

            print_merge_result(&result);

            // 7. Purge automático de tombstones > 30 dias após sync bem-sucedido
            purge_old_tombstones(storage);
        }

        git::PullResult::Conflict(remote_json) => {
            let ours_json = git::read_ours(&data_dir)?;
            let ours = json::parse_tasks_from_str(&ours_json)
                .context("Failed to parse local todos.json")?;
            let theirs = json::parse_tasks_from_str(&remote_json)
                .context("Failed to parse remote todos.json")?;

            let result = crate::sync::merge::merge(ours, theirs);
            storage.save(&result.tasks)?;
            git::finish_merge(&data_dir)?;

            println!("{} Conflict resolved via semantic merge", "✓".green());
            print_merge_result(&result);

            // Purge automático de tombstones > 30 dias após sync bem-sucedido
            purge_old_tombstones(storage);
        }
    }

    println!("{} Pull complete", "✓".green());
    Ok(())
}

fn print_merge_result(result: &crate::sync::merge::MergeResult) {
    if result.added > 0 {
        println!(
            "  {} {} task(s) added from remote",
            "↓".cyan(),
            result.added
        );
    }
    if result.updated > 0 {
        println!(
            "  {} {} task(s) updated (remote was newer)",
            "↺".cyan(),
            result.updated
        );
    }
}

// ── status ───────────────────────────────────────────────────────────────────

fn status() -> Result<()> {
    git::check_git_available()?;

    let data_dir = get_data_file_path()?
        .parent()
        .expect("todos.json always has a parent directory")
        .to_path_buf();

    if !git::is_initialized(&data_dir) {
        bail!("Git repository not initialized. Run: todo sync init <remote>");
    }

    let cfg = config::load()?;

    println!("\n{}\n", "Sync Status".bright_white().bold());

    match cfg {
        Some(c) => println!("  {:<14} {}", "Remote:".dimmed(), c.remote.cyan()),
        None => println!(
            "  {:<14} {}",
            "Remote:".dimmed(),
            "(not configured)".yellow()
        ),
    }

    let s = git::status(&data_dir)?;
    for line in s.lines() {
        println!("  {}", line.dimmed());
    }

    println!();
    Ok(())
}

// ── auto-purge ────────────────────────────────────────────────────────────────

/// Silently purges tombstones older than 30 days after a successful sync.
///
/// Runs without user interaction and without printing errors — purge is
/// best-effort maintenance and should never interrupt the sync flow.
fn purge_old_tombstones(storage: &impl Storage) {
    const PURGE_DAYS: i64 = 30;
    let cutoff = chrono::Utc::now() - chrono::Duration::days(PURGE_DAYS);

    let Ok((mut tasks, mut projects, mut notes, mut resources)) = storage.load_all_with_resources()
    else {
        return;
    };

    tasks.retain(|t| t.deleted_at.is_none_or(|d| d > cutoff));
    projects.retain(|p| p.deleted_at.is_none_or(|d| d > cutoff));
    notes.retain(|n| n.deleted_at.is_none_or(|d| d > cutoff));
    resources.retain(|r| r.deleted_at.is_none_or(|d| d > cutoff));

    let _ = storage.save_all(&tasks, &projects, &notes);
    let _ = storage.save_resources(&resources);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::config::SyncConfig;

    #[test]
    fn test_sync_command_variants_exist() {
        let _ = SyncCommand::Push;
        let _ = SyncCommand::Pull;
        let _ = SyncCommand::Status;
        let _ = SyncCommand::Init {
            remote: "git@github.com:user/tasks.git".to_string(),
        };
    }

    #[test]
    fn test_sync_config_require_error_message() {
        let result: Result<SyncConfig, _> = toml::from_str("");
        assert!(result.is_err());
    }

    #[test]
    fn test_build_commit_message_no_changes() {
        use crate::models::{Priority, Task};
        let tasks = vec![Task::new(
            "A".to_string(),
            Priority::Medium,
            vec![],
            None,
            None,
            None,
        )];
        let msg = build_commit_message(&tasks, std::path::Path::new("/nonexistent"));
        assert!(msg.contains("1 tasks"));
    }
}
