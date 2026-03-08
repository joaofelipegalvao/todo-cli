//! Integration tests for the `remove` command
//!
//! Covers:
//! - Remove existing task
//! - Remove with --yes flag (skip confirmation)
//! - Invalid ID (zero, out of range, empty storage)
//! - Task order after removal (IDs shift)
//! - Remove first, middle, last task

mod helpers;

use helpers::TestEnv;
use rustodo::cli::AddArgs;
use rustodo::commands::{task_add, task_remove};
use rustodo::models::Priority;

// ─── helpers ─────────────────────────────────────────────────────────────────

fn add_simple(env: &TestEnv, text: &str) -> usize {
    task_add::execute(
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
fn test_remove_single_task() {
    let env = TestEnv::new();
    add_simple(&env, "Task to remove");

    let result = task_remove::execute(env.storage(), 1, true);
    assert!(result.is_ok());
    assert_eq!(env.task_count(), 0);
}

#[test]
fn test_remove_reduces_task_count() {
    let env = TestEnv::new();
    add_simple(&env, "Task A");
    add_simple(&env, "Task B");
    add_simple(&env, "Task C");

    task_remove::execute(env.storage(), 2, true).unwrap();

    assert_eq!(env.task_count(), 2);
}

#[test]
fn test_remove_first_task() {
    let env = TestEnv::new();
    add_simple(&env, "First");
    add_simple(&env, "Second");
    add_simple(&env, "Third");

    task_remove::execute(env.storage(), 1, true).unwrap();

    let tasks = env.load_tasks();
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].text, "Second");
    assert_eq!(tasks[1].text, "Third");
}

#[test]
fn test_remove_middle_task() {
    let env = TestEnv::new();
    add_simple(&env, "First");
    add_simple(&env, "Middle");
    add_simple(&env, "Last");

    task_remove::execute(env.storage(), 2, true).unwrap();

    let tasks = env.load_tasks();
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].text, "First");
    assert_eq!(tasks[1].text, "Last");
}

#[test]
fn test_remove_last_task() {
    let env = TestEnv::new();
    add_simple(&env, "First");
    add_simple(&env, "Last");

    task_remove::execute(env.storage(), 2, true).unwrap();

    let tasks = env.load_tasks();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].text, "First");
}

#[test]
fn test_remove_correct_task_by_text() {
    let env = TestEnv::new();
    add_simple(&env, "Keep me");
    add_simple(&env, "Remove me");
    add_simple(&env, "Keep me too");

    task_remove::execute(env.storage(), 2, true).unwrap();

    let tasks = env.load_tasks();
    assert_eq!(tasks.len(), 2);
    assert!(tasks.iter().all(|t| t.text != "Remove me"));
    assert!(tasks.iter().any(|t| t.text == "Keep me"));
    assert!(tasks.iter().any(|t| t.text == "Keep me too"));
}

#[test]
fn test_remove_with_yes_flag_skips_confirmation() {
    let env = TestEnv::new();
    add_simple(&env, "Task");

    // yes=true should not prompt and just remove
    let result = task_remove::execute(env.storage(), 1, true);
    assert!(result.is_ok());
    assert_eq!(env.task_count(), 0);
}

// ─── invalid IDs ─────────────────────────────────────────────────────────────

#[test]
fn test_remove_id_zero_fails() {
    let env = TestEnv::new();
    add_simple(&env, "Task");

    let result = task_remove::execute(env.storage(), 0, true);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid"));
}

#[test]
fn test_remove_id_out_of_range_fails() {
    let env = TestEnv::new();
    add_simple(&env, "Task");

    let result = task_remove::execute(env.storage(), 99, true);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid"));
}

#[test]
fn test_remove_from_empty_storage_fails() {
    let env = TestEnv::new();

    let result = task_remove::execute(env.storage(), 1, true);
    assert!(result.is_err());
}

// ─── remove all one by one ────────────────────────────────────────────────────

#[test]
fn test_remove_all_tasks_one_by_one() {
    let env = TestEnv::new();
    add_simple(&env, "A");
    add_simple(&env, "B");
    add_simple(&env, "C");

    // Always remove ID 1 since list shifts after each removal
    task_remove::execute(env.storage(), 1, true).unwrap();
    task_remove::execute(env.storage(), 1, true).unwrap();
    task_remove::execute(env.storage(), 1, true).unwrap();

    assert_eq!(env.task_count(), 0);
}

#[test]
fn test_remove_preserves_task_metadata() {
    let env = TestEnv::new();
    add_simple(&env, "Keep this");
    task_add::execute(
        env.storage(),
        AddArgs {
            text: "Remove this".to_string(),
            priority: Priority::High,
            tag: vec!["work".to_string()],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();

    task_remove::execute(env.storage(), 2, true).unwrap();

    let tasks = env.load_tasks();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].text, "Keep this");
    assert_eq!(tasks[0].priority, Priority::Medium);
}
