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
//! Soft-delete is propagated via last-write-wins on `deleted_at`:
//!
//! - If only one side has `deleted_at`, that timestamp is compared against the
//!   other side's `updated_at` to decide which wins.
//! - If both sides have `deleted_at`, the most recent one wins (stays deleted).
//! - A task deleted locally but absent from remote is kept as deleted so the
//!   tombstone can propagate on the next push.
//! - A task deleted on remote but not yet seen locally is merged in as deleted.

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
                // In both — last-write-wins.
                //
                // The "effective timestamp" of a version is the most recent of
                // `updated_at` and `deleted_at` — whichever action happened last.
                let effective = |t: &Task| -> Option<chrono::DateTime<chrono::Utc>> {
                    match (t.updated_at, t.deleted_at) {
                        (Some(u), Some(d)) => Some(u.max(d)),
                        (Some(u), None) => Some(u),
                        (None, Some(d)) => Some(d),
                        (None, None) => None,
                    }
                };

                let use_remote = match (effective(&local_task), effective(remote_task)) {
                    (Some(l), Some(r)) => r > l,
                    (None, Some(_)) => true, // remote has a timestamp, local doesn't
                    (Some(_), None) => false, // local has a timestamp, remote doesn't
                    (None, None) => false,   // neither has a timestamp — keep local
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

    // ── Soft-delete propagation ───────────────────────────────────────────────

    /// Remote soft-deleted the task more recently → local should adopt deletion.
    #[test]
    fn test_merge_remote_deletion_wins_over_stale_local() {
        let mut local_task = task("Buy milk");
        local_task.updated_at = Some(Utc::now() - chrono::Duration::seconds(120));

        let mut remote_task = local_task.clone();
        remote_task.soft_delete();

        let result = merge(vec![local_task], vec![remote_task]);
        assert_eq!(result.tasks.len(), 1);
        assert!(result.tasks[0].is_deleted(), "remote deletion should win");
        assert_eq!(result.updated, 1);
    }

    /// Local edited the task more recently than the remote deletion → keep local (undeleted).
    #[test]
    fn test_merge_local_edit_wins_over_stale_remote_deletion() {
        let mut remote_task = task("Buy milk");
        remote_task.deleted_at = Some(Utc::now() - chrono::Duration::seconds(60));
        remote_task.updated_at = Some(Utc::now() - chrono::Duration::seconds(60));

        let mut local_task = remote_task.clone();
        local_task.deleted_at = None;
        local_task.updated_at = Some(Utc::now());

        let result = merge(vec![local_task], vec![remote_task]);
        assert_eq!(result.tasks.len(), 1);
        assert!(
            !result.tasks[0].is_deleted(),
            "local un-deletion should win"
        );
        assert_eq!(result.kept, 1);
    }

    /// Task deleted on both sides — most recent deletion timestamp wins, task stays deleted.
    #[test]
    fn test_merge_both_deleted_keeps_most_recent() {
        let base = task("Buy milk");

        let mut local_task = base.clone();
        local_task.deleted_at = Some(Utc::now() - chrono::Duration::seconds(120));
        local_task.updated_at = local_task.deleted_at;

        let mut remote_task = base.clone();
        remote_task.soft_delete();

        let result = merge(vec![local_task], vec![remote_task.clone()]);
        assert_eq!(result.tasks.len(), 1);
        assert!(result.tasks[0].is_deleted());
        assert_eq!(result.tasks[0].deleted_at, remote_task.deleted_at);
        assert_eq!(result.updated, 1);
    }

    /// A task deleted locally but not on remote should be kept as tombstone.
    #[test]
    fn test_merge_local_only_deleted_task_preserved_as_tombstone() {
        let mut local_task = task("Ghost task");
        local_task.soft_delete();

        let result = merge(vec![local_task.clone()], vec![]);
        assert_eq!(result.tasks.len(), 1);
        assert!(
            result.tasks[0].is_deleted(),
            "tombstone must be kept for sync propagation"
        );
        assert_eq!(result.kept, 1);
    }

    /// A task deleted on remote but absent locally arrives as deleted.
    #[test]
    fn test_merge_remote_only_deleted_task_arrives_as_deleted() {
        let mut remote_task = task("Remote ghost");
        remote_task.soft_delete();

        let result = merge(vec![], vec![remote_task]);
        assert_eq!(result.tasks.len(), 1);
        assert!(result.tasks[0].is_deleted());
        assert_eq!(result.added, 1);
    }

    /// Normal last-write-wins still works correctly alongside soft-delete logic.
    #[test]
    fn test_merge_normal_task_unaffected_by_soft_delete_logic() {
        let local_a = task("A");
        let mut remote_a = local_a.clone();
        remote_a.updated_at = Some(Utc::now() + chrono::Duration::seconds(5));
        remote_a.text = "A updated".to_string();

        let remote_b = task("B");

        let result = merge(vec![local_a], vec![remote_a, remote_b]);
        assert_eq!(result.tasks.len(), 2);
        assert_eq!(result.tasks[0].text, "A updated");
        assert_eq!(result.updated, 1);
        assert_eq!(result.added, 1);
    }
}
