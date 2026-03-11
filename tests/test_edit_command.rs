//! Integration tests for edit command

mod helpers;

use helpers::{TestEnv, days_from_now};
use rustodo::cli::{AddArgs, EditArgs};
use rustodo::commands::task;
use rustodo::models::Priority;

#[test]
fn test_edit_text() {
    let env = TestEnv::new();

    // Setup: Create task
    task::add::execute(
        env.storage(),
        AddArgs {
            text: "Old text".to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();

    // Execute: Edit text
    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 1, // ID
            text: Some("New text".to_string()),
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
            clear_deps: false,
        },
    );

    assert!(result.is_ok());

    // Verify
    let tasks = env.load_tasks();
    assert_eq!(tasks[0].text, "New text");
}

#[test]
fn test_edit_priority() {
    let env = TestEnv::new();

    task::add::execute(
        env.storage(),
        AddArgs {
            text: "Task".to_string(),
            priority: Priority::Low,
            tag: vec![],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();

    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 1,
            text: None,
            priority: Some(Priority::High), // Change to High
            add_tag: vec![],
            remove_tag: vec![],
            project: None,
            clear_project: false,
            due: None,
            clear_due: false,
            clear_tags: false,
            add_dep: vec![],
            remove_dep: vec![],
            clear_deps: false,
        },
    );

    assert!(result.is_ok());

    let tasks = env.load_tasks();
    assert_eq!(tasks[0].priority, Priority::High);
}

#[test]
fn test_edit_add_invalid_tag_fails() {
    let env = TestEnv::new();

    task::add::execute(
        env.storage(),
        AddArgs {
            text: "Task".to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();

    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 1,
            text: None,
            priority: None,
            add_tag: vec!["invalid tag".to_string()],
            remove_tag: vec![],
            project: None,
            clear_project: false,
            due: None,
            clear_due: false,
            clear_tags: false,
            add_dep: vec![],
            remove_dep: vec![],
            clear_deps: false,
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
fn test_edit_add_tags_preserves_existing() {
    let env = TestEnv::new();

    // Setup: Task with one tag
    task::add::execute(
        env.storage(),
        AddArgs {
            text: "Task".to_string(),
            priority: Priority::Medium,
            tag: vec!["work".to_string()],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();

    // Execute: Add another tag
    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 1,
            text: None,
            priority: None,
            add_tag: vec!["urgent".to_string()],
            remove_tag: vec![],
            project: None,
            clear_project: false,
            due: None,
            clear_due: false,
            clear_tags: false,
            add_dep: vec![],
            remove_dep: vec![],
            clear_deps: false,
        },
    );

    assert!(result.is_ok());

    // Verify: Should have BOTH tags
    let tasks = env.load_tasks();
    assert_eq!(tasks[0].tags.len(), 2);
    assert!(tasks[0].tags.contains(&"work".to_string()));
    assert!(tasks[0].tags.contains(&"urgent".to_string()));
}

#[test]
fn test_edit_remove_specific_tag() {
    let env = TestEnv::new();

    // Setup: Task with multiple tags
    task::add::execute(
        env.storage(),
        AddArgs {
            text: "Task".to_string(),
            priority: Priority::Medium,
            tag: vec![
                "work".to_string(),
                "urgent".to_string(),
                "frontend".to_string(),
            ],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();

    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 1,
            text: None,
            priority: None,
            add_tag: vec![],
            remove_tag: vec!["urgent".to_string()],
            project: None,
            clear_project: false,
            due: None,
            clear_due: false,
            clear_tags: false,
            add_dep: vec![],
            remove_dep: vec![],
            clear_deps: false,
        },
    );

    assert!(result.is_ok());

    // Verify: Should have work and frontend, NOT urgent
    let tasks = env.load_tasks();
    assert_eq!(tasks[0].tags.len(), 2);
    assert!(tasks[0].tags.contains(&"work".to_string()));
    assert!(tasks[0].tags.contains(&"frontend".to_string()));
    assert!(!tasks[0].tags.contains(&"urgent".to_string()));
}

#[test]
fn test_edit_add_and_remove_tags_simultaneously() {
    let env = TestEnv::new();

    // Setup
    task::add::execute(
        env.storage(),
        AddArgs {
            text: "Task".to_string(),
            priority: Priority::Medium,
            tag: vec!["work".to_string(), "old".to_string()],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();

    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 1,
            text: None,
            priority: None,
            add_tag: vec!["new".to_string()],
            remove_tag: vec!["old".to_string()],
            project: None,
            clear_project: false,
            due: None,
            clear_due: false,
            clear_tags: false,
            add_dep: vec![],
            remove_dep: vec![],
            clear_deps: false,
        },
    );

    assert!(result.is_ok());

    // Verify
    let tasks = env.load_tasks();
    assert_eq!(tasks[0].tags.len(), 2);
    assert!(tasks[0].tags.contains(&"work".to_string()));
    assert!(tasks[0].tags.contains(&"new".to_string()));
    assert!(!tasks[0].tags.contains(&"old".to_string()));
}

#[test]
fn test_edit_clear_all_tags() {
    let env = TestEnv::new();

    // Setup
    task::add::execute(
        env.storage(),
        AddArgs {
            text: "Task".to_string(),
            priority: Priority::Medium,
            tag: vec!["work".to_string(), "urgent".to_string()],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();

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
            clear_tags: true,
            add_dep: vec![],
            remove_dep: vec![],
            clear_deps: false,
        },
    );

    assert!(result.is_ok());

    // Verify
    let tasks = env.load_tasks();
    assert!(tasks[0].tags.is_empty());
}

#[test]
fn test_edit_remove_nonexistent_tag_fails() {
    let env = TestEnv::new();

    task::add::execute(
        env.storage(),
        AddArgs {
            text: "Task".to_string(),
            priority: Priority::Medium,
            tag: vec!["work".to_string()],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();

    // Execute: Try to remove tag that doesn't exist
    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 1,
            text: None,
            priority: None,
            add_tag: vec![],
            remove_tag: vec!["nonexistent".to_string()],
            project: None,
            clear_project: false,
            due: None,
            clear_due: false,
            clear_tags: false,
            add_dep: vec![],
            remove_dep: vec![],
            clear_deps: false,
        },
    );

    // Should fail
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("None of the specified tags")
    );
}

#[test]
fn test_edit_invalid_id() {
    let env = TestEnv::new();

    // No tasks exist
    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 99,
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
            clear_deps: false,
        },
    );

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid"));
}

#[test]
fn test_edit_due_date() {
    let env = TestEnv::new();

    task::add::execute(
        env.storage(),
        AddArgs {
            text: "Task".to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: None,
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();

    let due_date = days_from_now(7);

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
            due: Some(due_date.to_string()),
            clear_due: false,
            clear_tags: false,
            add_dep: vec![],
            remove_dep: vec![],
            clear_deps: false,
        },
    );

    assert!(result.is_ok());

    let tasks = env.load_tasks();
    assert_eq!(tasks[0].due_date, Some(due_date));
}

#[test]
fn test_edit_clear_due_date() {
    let env = TestEnv::new();

    let due_date = days_from_now(7);
    task::add::execute(
        env.storage(),
        AddArgs {
            text: "Task".to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: None,
            due: Some(due_date.to_string()),
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();

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
            clear_due: true, // clear_due
            clear_tags: false,
            add_dep: vec![],
            remove_dep: vec![],
            clear_deps: false,
        },
    );

    assert!(result.is_ok());

    let tasks = env.load_tasks();
    assert!(tasks[0].due_date.is_none());
}
