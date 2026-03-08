//! Integration tests for add command

mod helpers;

use helpers::{TestEnv, days_from_now};
use rustodo::cli::AddArgs;
use rustodo::commands::task_add;
use rustodo::models::{Priority, Recurrence};

#[test]
fn test_add_simple_task() {
    let env = TestEnv::new();

    let result = task_add::execute(
        env.storage(),
        AddArgs {
            text: "Buy milk".to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    );

    assert!(result.is_ok());
    assert_eq!(env.task_count(), 1);

    let tasks = env.load_tasks();
    assert_eq!(tasks[0].text, "Buy milk");
    assert_eq!(tasks[0].priority, Priority::Medium);
    assert!(!tasks[0].completed);
    assert!(tasks[0].tags.is_empty());
    assert!(tasks[0].due_date.is_none());
    assert!(tasks[0].recurrence.is_none());
}

#[test]
fn test_add_task_with_all_metadata() {
    let env = TestEnv::new();

    let due_date = days_from_now(7);

    let result = task_add::execute(
        env.storage(),
        AddArgs {
            text: "Complete project".to_string(),
            priority: Priority::High,
            tag: vec!["work".to_string(), "urgent".to_string()],
            project: None,
            due: Some(due_date.to_string()),
            recurrence: Some(Recurrence::Weekly),
            depends_on: vec![],
        },
    );

    assert!(result.is_ok());

    let tasks = env.load_tasks();
    assert_eq!(tasks.len(), 1);

    let task = &tasks[0];
    assert_eq!(task.text, "Complete project");
    assert_eq!(task.priority, Priority::High);
    assert_eq!(task.tags, vec!["work", "urgent"]);
    assert_eq!(task.due_date, Some(due_date));
    assert_eq!(task.recurrence, Some(Recurrence::Weekly));
}

#[test]
fn test_add_multiple_tasks_preserves_order() {
    let env = TestEnv::new();

    task_add::execute(
        env.storage(),
        AddArgs {
            text: "Task 1".to_string(),
            priority: Priority::Low,
            tag: vec![],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();
    task_add::execute(
        env.storage(),
        AddArgs {
            text: "Task 2".to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();
    task_add::execute(
        env.storage(),
        AddArgs {
            text: "Task 3".to_string(),
            priority: Priority::High,
            tag: vec![],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();

    let tasks = env.load_tasks();
    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].text, "Task 1");
    assert_eq!(tasks[1].text, "Task 2");
    assert_eq!(tasks[2].text, "Task 3");
}

#[test]
fn test_add_recurring_task_requires_due_date() {
    let env = TestEnv::new();

    let result = task_add::execute(
        env.storage(),
        AddArgs {
            text: "Daily standup".to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: None,
            due: None, // No due date
            recurrence: Some(Recurrence::Daily),
            depends_on: vec![],
        },
    );

    // Should fail validation
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("must have a due date"));
}

#[test]
fn test_add_empty_text_fails() {
    let env = TestEnv::new();

    let result = task_add::execute(
        env.storage(),
        AddArgs {
            text: "".to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    );

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("cannot be empty"));
}

#[test]
fn test_add_whitespace_only_text_fails() {
    let env = TestEnv::new();

    let result = task_add::execute(
        env.storage(),
        AddArgs {
            text: "   \t\n  ".to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    );

    assert!(result.is_err());
}

#[test]
fn test_add_with_invalid_tags_fails() {
    let env = TestEnv::new();

    // Tag with spaces
    let result = task_add::execute(
        env.storage(),
        AddArgs {
            text: "Task".to_string(),
            priority: Priority::Medium,
            tag: vec!["invalid tag".to_string()],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    );

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid tag format")
    );
}

#[test]
fn test_add_with_duplicate_tags_fails() {
    let env = TestEnv::new();

    let result = task_add::execute(
        env.storage(),
        AddArgs {
            text: "Task".to_string(),
            priority: Priority::Medium,
            tag: vec!["work".to_string(), "Work".to_string()], // Case-insensitive duplicate
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    );

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Duplicate tag"));
}
