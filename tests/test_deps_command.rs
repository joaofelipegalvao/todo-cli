//! Integration tests for the `deps` command
//!
//! Covers:
//! - Happy path: task with no deps, task with deps, task required by others
//! - Blocked status display
//! - All deps satisfied
//! - Invalid task ID
//! - Self-dependency prevention
//! - Cycle detection
//! - Dependency not found on remove
//! - Duplicate dependency on add

mod helpers;

use helpers::TestEnv;
use rustodo::cli::{AddArgs, EditArgs};
use rustodo::commands::task;
use rustodo::models::Priority;

// ─── helpers ────────────────────────────────────────────────────────────────

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

fn add_with_deps(env: &TestEnv, text: &str, depends_on: Vec<usize>) -> usize {
    task::add::execute(
        env.storage(),
        AddArgs {
            text: text.to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: None,
            due: None,
            recurrence: None,
            depends_on,
        },
    )
    .unwrap();
    env.task_count()
}

// ─── execute deps (just verifies it doesn't panic/error) ────────────────────

#[test]
fn test_deps_task_with_no_dependencies() {
    let env = TestEnv::new();
    add_simple(&env, "Standalone task");

    let result = task::deps::execute(env.storage(), 1);
    assert!(result.is_ok());
}

#[test]
fn test_deps_task_with_pending_dependency() {
    let env = TestEnv::new();
    add_simple(&env, "Setup database");
    add_with_deps(&env, "Run migrations", vec![1]);

    let result = task::deps::execute(env.storage(), 2);
    assert!(result.is_ok());

    // Verify blocking state via task model
    let tasks = env.load_tasks();
    assert!(
        tasks[1].is_blocked(&tasks),
        "task 2 should be blocked by task 1"
    );
}

#[test]
fn test_deps_task_with_completed_dependency() {
    let env = TestEnv::new();
    add_simple(&env, "Setup database");
    add_with_deps(&env, "Run migrations", vec![1]);

    task::done::execute(env.storage(), 1).unwrap();

    let result = task::deps::execute(env.storage(), 2);
    assert!(result.is_ok());

    let tasks = env.load_tasks();
    assert!(
        !tasks[1].is_blocked(&tasks),
        "task 2 should NOT be blocked after dep is done"
    );
}

#[test]
fn test_deps_task_required_by_others() {
    let env = TestEnv::new();
    add_simple(&env, "Core library");
    add_with_deps(&env, "Feature A", vec![1]);
    add_with_deps(&env, "Feature B", vec![1]);

    // deps on task 1 should mention it is required by tasks 2 and 3
    let result = task::deps::execute(env.storage(), 1);
    assert!(result.is_ok());

    let tasks = env.load_tasks();
    let task1_uuid = tasks[0].uuid;
    let dependents: Vec<usize> = tasks
        .iter()
        .enumerate()
        .filter(|(_, t)| t.depends_on.contains(&task1_uuid))
        .map(|(i, _)| i + 1)
        .collect();
    assert_eq!(dependents, vec![2, 3]);
}

#[test]
fn test_deps_all_dependencies_satisfied() {
    let env = TestEnv::new();
    add_simple(&env, "Dep A");
    add_simple(&env, "Dep B");
    add_with_deps(&env, "Final task", vec![1, 2]);

    task::done::execute(env.storage(), 1).unwrap();
    task::done::execute(env.storage(), 2).unwrap();

    let result = task::deps::execute(env.storage(), 3);
    assert!(result.is_ok());

    let tasks = env.load_tasks();
    assert!(!tasks[2].is_blocked(&tasks));
    assert!(tasks[2].blocking_deps(&tasks).is_empty());
}

#[test]
fn test_deps_partially_satisfied() {
    let env = TestEnv::new();
    add_simple(&env, "Dep A");
    add_simple(&env, "Dep B");
    add_with_deps(&env, "Final task", vec![1, 2]);

    task::done::execute(env.storage(), 1).unwrap();
    // Dep B still pending

    let tasks = env.load_tasks();
    let dep_b_uuid = tasks[1].uuid;
    let blocking = tasks[2].blocking_deps(&tasks);
    assert_eq!(blocking, vec![dep_b_uuid], "only dep B should be blocking");
}

// ─── invalid task ID ────────────────────────────────────────────────────────

#[test]
fn test_deps_invalid_id_zero() {
    let env = TestEnv::new();
    add_simple(&env, "Task");

    let result = task::deps::execute(env.storage(), 0);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid"));
}

#[test]
fn test_deps_invalid_id_out_of_range() {
    let env = TestEnv::new();
    add_simple(&env, "Task");

    let result = task::deps::execute(env.storage(), 99);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid"));
}

#[test]
fn test_deps_empty_storage() {
    let env = TestEnv::new();

    let result = task::deps::execute(env.storage(), 1);
    assert!(result.is_err());
}

// ─── self-dependency prevention ─────────────────────────────────────────────

#[test]
fn test_add_self_dependency_fails() {
    let env = TestEnv::new();
    // Task ID will be 1, so depends_on=[1] is a self-reference
    let result = task::add::execute(
        env.storage(),
        AddArgs {
            text: "Self-referencing task".to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![1],
        },
    );

    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("itself") || msg.contains("self"),
        "got: {}",
        msg
    );
}

#[test]
fn test_edit_add_self_dependency_fails() {
    let env = TestEnv::new();
    add_simple(&env, "Task A");

    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 1,
            text: None,
            priority: None,
            add_tag: vec![],
            remove_tag: vec![],
            project: None,
            clear_project: false,
            due: None,
            clear_due: false,
            clear_tags: false,
            add_dep: vec![1],
            remove_dep: vec![],
            clear_deps: false,
        },
    );

    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("itself") || msg.contains("self"),
        "got: {}",
        msg
    );
}

// ─── cycle detection ────────────────────────────────────────────────────────

#[test]
fn test_edit_direct_cycle_fails() {
    let env = TestEnv::new();
    add_simple(&env, "A");
    add_with_deps(&env, "B", vec![1]); // B depends on A

    // Now try to make A depend on B → cycle A→B→A
    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 1,
            text: None,
            priority: None,
            add_tag: vec![],
            remove_tag: vec![],
            project: None,
            clear_project: false,
            due: None,
            clear_due: false,
            clear_tags: false,
            add_dep: vec![2], // add_dep: A depends on B
            remove_dep: vec![],
            clear_deps: false,
        },
    );

    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("cycle"), "got: {}", msg);
}

#[test]
fn test_edit_transitive_cycle_fails() {
    let env = TestEnv::new();
    add_simple(&env, "A");
    add_with_deps(&env, "B", vec![1]); // B → A
    add_with_deps(&env, "C", vec![2]); // C → B → A

    // Try to make A depend on C → cycle A→C→B→A
    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 1,
            text: None,
            priority: None,
            add_tag: vec![],
            remove_tag: vec![],
            project: None,
            clear_project: false,
            due: None,
            clear_due: false,
            clear_tags: false,
            add_dep: vec![3],
            remove_dep: vec![],
            clear_deps: false,
        },
    );

    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("cycle"), "got: {}", msg);
}

#[test]
fn test_edit_no_cycle_on_valid_dep() {
    let env = TestEnv::new();
    add_simple(&env, "A");
    add_simple(&env, "B");
    add_with_deps(&env, "C", vec![2]); // C → B

    // A depends on C is fine (A←C→B, no cycle involving A)
    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 1,
            text: None,
            priority: None,
            add_tag: vec![],
            remove_tag: vec![],
            project: None,
            clear_project: false,
            due: None,
            clear_due: false,
            clear_tags: false,
            add_dep: vec![3], // A depends on C
            remove_dep: vec![],
            clear_deps: false,
        },
    );

    assert!(result.is_ok());
}

// ─── duplicate dependency ────────────────────────────────────────────────────

#[test]
fn test_edit_duplicate_dependency_fails() {
    let env = TestEnv::new();
    add_simple(&env, "Dep");
    add_with_deps(&env, "Task", vec![1]);

    // Try to add dep 1 again
    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 2,
            text: None,
            priority: None,
            add_tag: vec![],
            remove_tag: vec![],
            project: None,
            clear_project: false,
            due: None,
            clear_due: false,
            clear_tags: false,
            add_dep: vec![1], // already a dep
            remove_dep: vec![],
            clear_deps: false,
        },
    );

    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("already"), "got: {}", msg);
}

// ─── remove dependency ───────────────────────────────────────────────────────

#[test]
fn test_edit_remove_existing_dependency() {
    let env = TestEnv::new();
    add_simple(&env, "Dep");
    add_with_deps(&env, "Task", vec![1]);

    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 2,
            text: None,
            priority: None,
            add_tag: vec![],
            remove_tag: vec![],
            project: None,
            clear_project: false,
            due: None,
            clear_due: false,
            clear_tags: false,
            add_dep: vec![],
            remove_dep: vec![1],
            clear_deps: false,
        },
    );

    assert!(result.is_ok());

    let tasks = env.load_tasks();
    assert!(tasks[1].depends_on.is_empty());
}

#[test]
fn test_edit_remove_nonexistent_dependency_fails() {
    let env = TestEnv::new();
    add_simple(&env, "Dep");
    add_simple(&env, "Task"); // No deps

    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 2,
            text: None,
            priority: None,
            add_tag: vec![],
            remove_tag: vec![],
            project: None,
            clear_project: false,
            due: None,
            clear_due: false,
            clear_tags: false,
            add_dep: vec![],
            remove_dep: vec![1],
            clear_deps: false,
        },
    );

    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("does not depend"), "got: {}", msg);
}

#[test]
fn test_edit_clear_all_dependencies() {
    let env = TestEnv::new();
    add_simple(&env, "A");
    add_simple(&env, "B");
    add_with_deps(&env, "C", vec![1, 2]);

    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 3,
            text: None,
            priority: None,
            add_tag: vec![],
            remove_tag: vec![],
            project: None,
            clear_project: false,
            due: None,
            clear_due: false,
            clear_tags: false,
            add_dep: vec![],
            remove_dep: vec![],
            clear_deps: true, // clear_deps
        },
    );

    assert!(result.is_ok());

    let tasks = env.load_tasks();
    assert!(tasks[2].depends_on.is_empty());
}

// ─── blocking_deps model tests ───────────────────────────────────────────────

#[test]
fn test_blocking_deps_returns_only_pending() {
    let env = TestEnv::new();
    add_simple(&env, "A");
    add_simple(&env, "B");
    add_with_deps(&env, "C", vec![1, 2]);

    task::done::execute(env.storage(), 1).unwrap();

    let tasks = env.load_tasks();
    let task_b_build = tasks[1].uuid;
    let blocking = tasks[2].blocking_deps(&tasks);
    assert_eq!(blocking, vec![task_b_build]);
}

#[test]
fn test_blocking_deps_empty_when_all_done() {
    let env = TestEnv::new();
    add_simple(&env, "A");
    add_simple(&env, "B");
    add_with_deps(&env, "C", vec![1, 2]);

    task::done::execute(env.storage(), 1).unwrap();
    task::done::execute(env.storage(), 2).unwrap();

    let tasks = env.load_tasks();
    assert!(tasks[2].blocking_deps(&tasks).is_empty());
}

// ─── deps not inherited by recurrence ────────────────────────────────────────

#[test]
fn test_recurrence_does_not_inherit_deps() {
    use helpers::days_from_now;
    use rustodo::models::Recurrence;

    let env = TestEnv::new();
    add_simple(&env, "Blocker");

    let due_str = format!("{}", days_from_now(1).format("%Y-%m-%d"));

    task::add::execute(
        env.storage(),
        AddArgs {
            text: "Recurring task".to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: None,
            due: Some(due_str),
            recurrence: Some(Recurrence::Daily),
            depends_on: vec![1],
        },
    )
    .unwrap();

    // Complete blocker first so task 2 can be completed
    task::done::execute(env.storage(), 1).unwrap();
    task::done::execute(env.storage(), 2).unwrap();

    // The newly created recurrence (task 3) should have no deps
    let tasks = env.load_tasks();
    let next = tasks.last().unwrap();
    assert!(
        next.depends_on.is_empty(),
        "recurrence should not inherit dependencies"
    );
}
