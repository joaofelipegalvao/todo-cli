//! Integration tests for the `purge` command
//!
//! Covers:
//! - No tombstones found (empty storage, no deleted tasks)
//! - Dry run previews without removing
//! - Purge with --days 0 removes all tombstones
//! - Purge respects --days threshold (keeps recent tombstones)
//! - Purge only removes deleted tasks, not active ones
//! - Purge with multiple tombstones
//! - Raw storage still has correct count after purge

mod helpers;

use helpers::TestEnv;
use rustodo::cli::AddArgs;
use rustodo::commands::{add, purge, remove};
use rustodo::models::Priority;

// ─── helpers ─────────────────────────────────────────────────────────────────

fn add_simple(env: &TestEnv, text: &str) {
    add::execute(
        env.storage(),
        AddArgs {
            text: text.to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();
}

// ─── no tombstones ────────────────────────────────────────────────────────────

#[test]
fn test_purge_empty_storage_no_error() {
    let env = TestEnv::new();
    let result = purge::execute(env.storage(), 30, false, true);
    assert!(result.is_ok());
}

#[test]
fn test_purge_no_deleted_tasks_no_error() {
    let env = TestEnv::new();
    add_simple(&env, "Active task");

    let result = purge::execute(env.storage(), 30, false, true);
    assert!(result.is_ok());
    assert_eq!(env.task_count(), 1, "active task should be untouched");
}

// ─── dry run ─────────────────────────────────────────────────────────────────

#[test]
fn test_purge_dry_run_does_not_remove() {
    let env = TestEnv::new();
    add_simple(&env, "Task to delete");
    remove::execute(env.storage(), 1, true).unwrap();

    assert_eq!(env.task_count(), 0);
    assert_eq!(env.raw_task_count(), 1, "tombstone should exist");

    purge::execute(env.storage(), 0, true, true).unwrap(); // dry_run = true

    assert_eq!(env.raw_task_count(), 1, "dry run must not remove tombstone");
}

// ─── days = 0: remove all tombstones ─────────────────────────────────────────

#[test]
fn test_purge_days_zero_removes_all_tombstones() {
    let env = TestEnv::new();
    add_simple(&env, "Delete me");
    remove::execute(env.storage(), 1, true).unwrap();

    assert_eq!(env.raw_task_count(), 1);

    purge::execute(env.storage(), 0, false, true).unwrap();

    assert_eq!(
        env.raw_task_count(),
        0,
        "tombstone should be permanently removed"
    );
}

#[test]
fn test_purge_days_zero_removes_multiple_tombstones() {
    let env = TestEnv::new();
    add_simple(&env, "A");
    add_simple(&env, "B");
    add_simple(&env, "C");

    remove::execute(env.storage(), 1, true).unwrap();
    remove::execute(env.storage(), 1, true).unwrap();
    remove::execute(env.storage(), 1, true).unwrap();

    assert_eq!(env.raw_task_count(), 3);
    assert_eq!(env.task_count(), 0);

    purge::execute(env.storage(), 0, false, true).unwrap();

    assert_eq!(env.raw_task_count(), 0);
}

// ─── days threshold ───────────────────────────────────────────────────────────

#[test]
fn test_purge_high_days_threshold_keeps_recent_tombstones() {
    let env = TestEnv::new();
    add_simple(&env, "Recently deleted");
    remove::execute(env.storage(), 1, true).unwrap();

    // Tombstone is brand new — should NOT be purged with a 30-day threshold
    purge::execute(env.storage(), 30, false, true).unwrap();

    assert_eq!(
        env.raw_task_count(),
        1,
        "recent tombstone should be kept with 30-day threshold"
    );
}

#[test]
fn test_purge_days_zero_is_the_only_threshold_that_catches_new_tombstones() {
    let env = TestEnv::new();
    add_simple(&env, "Just deleted");
    remove::execute(env.storage(), 1, true).unwrap();

    // days=1 should NOT catch a tombstone created milliseconds ago
    purge::execute(env.storage(), 1, false, true).unwrap();
    assert_eq!(
        env.raw_task_count(),
        1,
        "1-day threshold should keep brand-new tombstone"
    );

    // days=0 SHOULD catch it
    purge::execute(env.storage(), 0, false, true).unwrap();
    assert_eq!(env.raw_task_count(), 0);
}

// ─── only tombstones are removed, not active tasks ───────────────────────────

#[test]
fn test_purge_does_not_remove_active_tasks() {
    let env = TestEnv::new();
    add_simple(&env, "Keep me");
    add_simple(&env, "Delete me");

    remove::execute(env.storage(), 2, true).unwrap();

    assert_eq!(env.task_count(), 1);
    assert_eq!(env.raw_task_count(), 2);

    purge::execute(env.storage(), 0, false, true).unwrap();

    assert_eq!(env.task_count(), 1, "active task must survive purge");
    assert_eq!(env.raw_task_count(), 1, "only tombstone should be removed");

    let tasks = env.load_tasks();
    assert_eq!(tasks[0].text, "Keep me");
}

#[test]
fn test_purge_mixed_active_and_tombstones() {
    let env = TestEnv::new();
    add_simple(&env, "Active A");
    add_simple(&env, "Delete B");
    add_simple(&env, "Active C");
    add_simple(&env, "Delete D");

    // Visible: A(1), B(2), C(3), D(4)
    remove::execute(env.storage(), 2, true).unwrap(); // Delete B
    // Visible: A(1), C(2), D(3)
    remove::execute(env.storage(), 3, true).unwrap(); // Delete D

    // After removing B: visible = [Active A, Active C, Delete D] → ids 1,2,3
    // Remove id 2 = Active C? No — let's remove id 3 (Delete D)
    // Re-think: remove B then D
    // State: Active A(1), Delete B(2), Active C(3), Delete D(4)
    // remove id 2 → B deleted. Visible: A(1), C(2), D(3)
    // remove id 3 → D deleted. Visible: A(1), C(2)
    assert_eq!(env.task_count(), 2);
    assert_eq!(env.raw_task_count(), 4);

    purge::execute(env.storage(), 0, false, true).unwrap();

    assert_eq!(env.task_count(), 2);
    assert_eq!(env.raw_task_count(), 2);

    let tasks = env.load_tasks();
    assert!(tasks.iter().any(|t| t.text == "Active A"));
    assert!(tasks.iter().any(|t| t.text == "Active C"));
}
