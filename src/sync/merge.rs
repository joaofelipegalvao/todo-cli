//! Semantic 3-way merge for task lists.
//!
//! Merges two task lists using UUID as the stable identity key and
//! `updated_at` as the conflict-resolution tiebreaker (last-write-wins).
//!
//! # Algorithm
//!
//! Given a `local` list and a `remote` list:
//!
//! 1. Build a map `uuid → task` for each side.
//! 2. For each UUID present in either side:
//!    - Only in local  → keep
//!    - Only in remote → add
//!    - In both        → pick the version with the most recent `updated_at`
//! 3. Preserve original order: local tasks first, then remote-only tasks appended.
//!
//! # Deletion
//!
//! Deletion is not handled — a task deleted on one side but present on the
//! other will be re-added. Soft-delete support is deferred to a future version.

use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::models::Task;

/// Result of a merge operation.
#[derive(Debug, Default)]
pub struct MergeResult {
    /// Final merged task list.
    pub tasks: Vec<Task>,
    /// Number of tasks added from remote.
    pub added: usize,
    /// Number of tasks updated (remote version was newer).
    pub updated: usize,
    /// Number of tasks kept unchanged (local version was newer or equal).
    pub kept: usize,
}

/// Merges `remote` into `local` using UUID + `updated_at` (last-write-wins).
///
/// # Examples
///
/// ```
/// use rustodo::models::{Task, Priority};
/// use rustodo::sync::merge::merge;
///
/// let local = vec![Task::new("A".to_string(), Priority::Medium, vec![], None, None, None)];
/// let remote = vec![Task::new("B".to_string(), Priority::Medium, vec![], None, None, None)];
///
/// let result = merge(local, remote);
/// assert_eq!(result.tasks.len(), 2);
/// assert_eq!(result.added, 1);
/// ```
pub fn merge(local: Vec<Task>, remote: Vec<Task>) -> MergeResult {
    let remote_map: HashMap<Uuid, Task> = remote.into_iter().map(|t| (t.uuid, t)).collect();
    let local_uuids: HashSet<Uuid> = local.iter().map(|t| t.uuid).collect();

    let mut tasks = Vec::with_capacity(local.len());
    let mut added = 0;
    let mut updated = 0;
    let mut kept = 0;

    // Pass 1: iterate local tasks, picking best version for conflicts
    for local_task in local {
        match remote_map.get(&local_task.uuid) {
            None => {
                // Only in local — keep as-is
                tasks.push(local_task);
                kept += 1;
            }
            Some(remote_task) => {
                // In both — last-write-wins via updated_at
                let use_remote = match (local_task.updated_at, remote_task.updated_at) {
                    (Some(l), Some(r)) => r > l,
                    (None, Some(_)) => true, // remote has timestamp, local doesn't
                    (Some(_), None) => false, // local has timestamp, remote doesn't
                    (None, None) => false,   // neither has timestamp — keep local
                };

                if use_remote {
                    tasks.push(remote_task.clone());
                    updated += 1;
                } else {
                    tasks.push(local_task);
                    kept += 1;
                }
            }
        }
    }

    // Pass 2: append remote-only tasks (new tasks added on remote)
    for (uuid, remote_task) in remote_map {
        if !local_uuids.contains(&uuid) {
            tasks.push(remote_task);
            added += 1;
        }
    }

    MergeResult {
        tasks,
        added,
        updated,
        kept,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Priority, Task};
    use chrono::Utc;

    fn task(text: &str) -> Task {
        Task::new(text.to_string(), Priority::Medium, vec![], None, None, None)
    }

    #[test]
    fn test_merge_remote_only_task_is_added() {
        let local_a = task("A");
        let mut remote_a = task("A");
        remote_a.uuid = local_a.uuid;

        let remote_b = task("B");

        let result = merge(vec![local_a], vec![remote_a, remote_b]);
        assert_eq!(result.tasks.len(), 2);
        assert_eq!(result.added, 1);
    }

    #[test]
    fn test_merge_local_only_task_is_kept() {
        let local = vec![task("A"), task("B")];
        let remote = vec![task("A")];
        let result = merge(local, remote);
        // B is local-only — should be preserved
        assert!(result.tasks.iter().any(|t| t.text == "B"));
        assert_eq!(result.kept, 2); // A kept (same), B kept (local-only)
    }

    #[test]
    fn test_merge_remote_wins_when_newer() {
        let mut local_task = task("Original");
        let uuid = local_task.uuid;
        local_task.updated_at = Some(Utc::now() - chrono::Duration::seconds(60));

        let mut remote_task = task("Updated");
        remote_task.uuid = uuid;
        remote_task.updated_at = Some(Utc::now());

        let result = merge(vec![local_task], vec![remote_task]);
        assert_eq!(result.tasks[0].text, "Updated");
        assert_eq!(result.updated, 1);
    }

    #[test]
    fn test_merge_local_wins_when_newer() {
        let mut local_task = task("Local newer");
        let uuid = local_task.uuid;
        local_task.updated_at = Some(Utc::now());

        let mut remote_task = task("Remote older");
        remote_task.uuid = uuid;
        remote_task.updated_at = Some(Utc::now() - chrono::Duration::seconds(60));

        let result = merge(vec![local_task], vec![remote_task]);
        assert_eq!(result.tasks[0].text, "Local newer");
        assert_eq!(result.kept, 1);
    }

    #[test]
    fn test_merge_no_timestamp_keeps_local() {
        let mut local_task = task("Local");
        let uuid = local_task.uuid;
        local_task.updated_at = None;

        let mut remote_task = task("Remote");
        remote_task.uuid = uuid;
        remote_task.updated_at = None;

        let result = merge(vec![local_task], vec![remote_task]);
        assert_eq!(result.tasks[0].text, "Local");
        assert_eq!(result.kept, 1);
    }

    #[test]
    fn test_merge_empty_remote() {
        let local = vec![task("A"), task("B")];
        let result = merge(local, vec![]);
        assert_eq!(result.tasks.len(), 2);
        assert_eq!(result.kept, 2);
        assert_eq!(result.added, 0);
    }

    #[test]
    fn test_merge_empty_local() {
        let remote = vec![task("A"), task("B")];
        let result = merge(vec![], remote);
        assert_eq!(result.tasks.len(), 2);
        assert_eq!(result.added, 2);
        assert_eq!(result.kept, 0);
    }
}
