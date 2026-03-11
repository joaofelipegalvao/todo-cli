//! Integration tests for the `undone` command
//!
//! Covers:
//! - Revert completed task to pending
//! - Cannot undone an already pending task
//! - Invalid ID
//! - completed_at is cleared on undone
//! - Multiple tasks: only target is reverted

mod helpers;

use helpers::TestEnv;
use rustodo::cli::AddArgs;
use rustodo::commands::task;
use rustodo::models::Priority;

// ─── helpers ─────────────────────────────────────────────────────────────────

fn add_simple(env: &TestEnv, text: &str) -> usize {
    task::add::execute(
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
    env.task_count()
}

// ─── happy path ───────────────────────────────────────────────────────────────

#[test]
fn test_undone_reverts_completed_task() {
    let env = TestEnv::new();
    add_simple(&env, "Task");

    task::done::execute(env.storage(), 1).unwrap();

    let result = task::undone::execute(env.storage(), 1);
    assert!(result.is_ok());

    let tasks = env.load_tasks();
    assert!(!tasks[0].completed);
}

#[test]
fn test_undone_clears_completed_at() {
    let env = TestEnv::new();
    add_simple(&env, "Task");

    task::done::execute(env.storage(), 1).unwrap();

    // Verify completed_at was set
    let tasks = env.load_tasks();
    assert!(tasks[0].completed_at.is_some());

    task::undone::execute(env.storage(), 1).unwrap();

    // Verify completed_at was cleared
    let tasks = env.load_tasks();
    assert!(tasks[0].completed_at.is_none());
}

#[test]
fn test_undone_allows_redone_after() {
    let env = TestEnv::new();
    add_simple(&env, "Task");

    task::done::execute(env.storage(), 1).unwrap();
    task::undone::execute(env.storage(), 1).unwrap();

    // Should be able to complete again
    let result = task::done::execute(env.storage(), 1);
    assert!(result.is_ok());

    let tasks = env.load_tasks();
    assert!(tasks[0].completed);
}

#[test]
fn test_undone_only_affects_target_task() {
    let env = TestEnv::new();
    add_simple(&env, "Task A");
    add_simple(&env, "Task B");
    add_simple(&env, "Task C");

    task::done::execute(env.storage(), 1).unwrap();
    task::done::execute(env.storage(), 2).unwrap();
    task::done::execute(env.storage(), 3).unwrap();

    task::undone::execute(env.storage(), 2).unwrap();

    let tasks = env.load_tasks();
    assert!(tasks[0].completed, "Task A should still be done");
    assert!(!tasks[1].completed, "Task B should be pending");
    assert!(tasks[2].completed, "Task C should still be done");
}

#[test]
fn test_undone_then_done_cycle() {
    let env = TestEnv::new();
    add_simple(&env, "Task");

    for _ in 0..3 {
        task::done::execute(env.storage(), 1).unwrap();
        task::undone::execute(env.storage(), 1).unwrap();
    }

    let tasks = env.load_tasks();
    assert!(!tasks[0].completed);
}

// ─── error cases ─────────────────────────────────────────────────────────────

#[test]
fn test_undone_already_pending_fails() {
    let env = TestEnv::new();
    add_simple(&env, "Task");

    // Never completed — should fail
    let result = task::undone::execute(env.storage(), 1);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("already"));
}

#[test]
fn test_undone_id_zero_fails() {
    let env = TestEnv::new();
    add_simple(&env, "Task");

    let result = task::undone::execute(env.storage(), 0);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid"));
}

#[test]
fn test_undone_id_out_of_range_fails() {
    let env = TestEnv::new();
    add_simple(&env, "Task");

    let result = task::undone::execute(env.storage(), 99);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid"));
}

#[test]
fn test_undone_empty_storage_fails() {
    let env = TestEnv::new();

    let result = task::undone::execute(env.storage(), 1);
    assert!(result.is_err());
}
