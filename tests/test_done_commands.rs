// ! Integration tests for done command with dependency blocking

use std::vec;

use rustodo::{cli::AddArgs, commands::task, models::Priority};

use crate::helpers::TestEnv;

mod helpers;

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

#[test]
fn test_done_blocked_by_pending_dep() {
    let env = TestEnv::new();

    let _dep_id = add_simple(&env, "Setup database");
    let task_id = add_with_deps(&env, "Run migrations", vec![1]);

    let result = task::done::execute(env.storage(), task_id);

    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("blocked"),
        "error should mention 'blocked', got: {}",
        msg
    );
    assert!(
        msg.contains("#1"),
        "error should list blocking task ID, got: {}",
        msg
    );

    let tasks = env.load_tasks();
    assert!(!tasks[task_id - 1].completed);
}

#[test]
fn test_done_unblocked_after_dep_completed() {
    let env = TestEnv::new();

    add_simple(&env, "Setup database");
    let task_id = add_with_deps(&env, "Run migrations", vec![1]);

    task::done::execute(env.storage(), 1).unwrap();

    let result = task::done::execute(env.storage(), task_id);
    assert!(result.is_ok());

    let tasks = env.load_tasks();
    assert!(tasks[task_id - 1].completed);
}

#[test]
fn test_done_no_deps_works_normally() {
    let env = TestEnv::new();
    add_simple(&env, "Independent task");

    let result = task::done::execute(env.storage(), 1);
    assert!(result.is_ok());

    let tasks = env.load_tasks();
    assert!(tasks[0].completed)
}

#[test]
fn test_done_blocked_by_one_of_multiple_deps() {
    let env = TestEnv::new();

    add_simple(&env, "Dep A");
    add_simple(&env, "Dep B");
    let task_id = add_with_deps(&env, "Final task", vec![1, 2]);

    task::done::execute(env.storage(), 1).unwrap();

    let result = task::done::execute(env.storage(), task_id);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("#2"));
}

#[test]
fn test_done_unblocked_all_deps_completed() {
    let env = TestEnv::new();

    add_simple(&env, "Dep A");
    add_simple(&env, "Dep B");
    let task_id = add_with_deps(&env, "Final task", vec![1, 2]);

    task::done::execute(env.storage(), 1).unwrap();
    task::done::execute(env.storage(), 2).unwrap();

    let result = task::done::execute(env.storage(), task_id);
    assert!(result.is_ok());
}

#[test]
fn test_done_already_completed_fails() {
    let env = TestEnv::new();
    add_simple(&env, "Task");

    task::done::execute(env.storage(), 1).unwrap();

    let result = task::done::execute(env.storage(), 1);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("already"));
}

#[test]
fn test_done_chain_must_be_completed_in_order() {
    let env = TestEnv::new();

    add_simple(&env, "A");
    add_with_deps(&env, "B", vec![1]);
    add_with_deps(&env, "C", vec![2]);

    assert!(task::done::execute(env.storage(), 3).is_err());
    assert!(task::done::execute(env.storage(), 2).is_err());

    task::done::execute(env.storage(), 1).unwrap();
    task::done::execute(env.storage(), 2).unwrap();
    task::done::execute(env.storage(), 3).unwrap();

    let tasks = env.load_tasks();
    assert!(tasks.iter().all(|t| t.completed));
}
