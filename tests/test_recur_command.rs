//! Integration tests for the `projects` command
//!
//! Covers:
//! - Happy path: single project, multiple projects, mixed (with/without project)
//! - Task counts (pending vs done)
//! - No projects found
//! - Project filter in `list` command
//! - Project assignment and removal via `edit`
//! - Case-insensitive project filter
//! - Project name validation

mod helpers;
use helpers::TestEnv;
use rustodo::cli::{AddArgs, EditArgs};
use rustodo::commands::{project, task};
use rustodo::models::{Priority, SortBy, StatusFilter};
use rustodo::storage::Storage;

// ─── helpers ─────────────────────────────────────────────────────────────────

fn add_task(env: &TestEnv, text: &str, project: Option<&str>) -> usize {
    task::add::execute(
        env.storage(),
        AddArgs {
            text: text.to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: project.map(|s| s.to_string()),
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();
    env.task_count()
}

// ─── projects command ─────────────────────────────────────────────────────────

#[test]
fn test_projects_no_projects_fails() {
    let env = TestEnv::new();
    add_task(&env, "Task without project", None);

    let result = project::list::execute(env.storage());
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.to_lowercase().contains("project"), "got: {}", msg);
}

#[test]
fn test_projects_empty_storage_fails() {
    let env = TestEnv::new();

    let result = project::list::execute(env.storage());
    assert!(result.is_err());
}

#[test]
fn test_projects_single_project() {
    let env = TestEnv::new();
    add_task(&env, "Task A", Some("Backend"));
    add_task(&env, "Task B", Some("Backend"));

    let result = project::list::execute(env.storage());
    assert!(result.is_ok());

    // Verify counts via storage
    let tasks = env.load_tasks();
    let projects = env.storage().load_projects().unwrap();
    let backend_uuid = projects
        .iter()
        .find(|p| p.name == "Backend")
        .map(|p| p.uuid);
    let backend_tasks: Vec<_> = tasks
        .iter()
        .filter(|t| t.project_id == backend_uuid)
        .collect();
    assert_eq!(backend_tasks.len(), 2);
}

#[test]
fn test_projects_multiple_projects() {
    let env = TestEnv::new();
    add_task(&env, "API endpoint", Some("Backend"));
    add_task(&env, "Button component", Some("Frontend"));
    add_task(&env, "Write docs", Some("Docs"));

    let result = project::list::execute(env.storage());
    assert!(result.is_ok());

    let projects = env.storage().load_projects().unwrap();
    let active_projects: Vec<_> = projects.iter().filter(|p| !p.is_deleted()).collect();
    assert_eq!(active_projects.len(), 3);
}

#[test]
fn test_projects_pending_and_done_counts() {
    let env = TestEnv::new();
    add_task(&env, "Task A", Some("Backend"));
    add_task(&env, "Task B", Some("Backend"));
    add_task(&env, "Task C", Some("Backend"));

    task::done::execute(env.storage(), 1).unwrap();

    let tasks = env.load_tasks();
    let projects = env.storage().load_projects().unwrap();
    let backend_uuid = projects
        .iter()
        .find(|p| p.name == "Backend")
        .map(|p| p.uuid);
    let backend: Vec<_> = tasks
        .iter()
        .filter(|t| t.project_id == backend_uuid)
        .collect();

    let pending = backend.iter().filter(|t| !t.completed).count();
    let done_count = backend.iter().filter(|t| t.completed).count();

    assert_eq!(pending, 2);
    assert_eq!(done_count, 1);
    assert_eq!(backend.len(), 3);
}

#[test]
fn test_projects_mixed_with_and_without_project() {
    let env = TestEnv::new();
    add_task(&env, "Task with project", Some("Backend"));
    add_task(&env, "Task without project", None);
    add_task(&env, "Another without project", None);

    let result = project::list::execute(env.storage());
    assert!(result.is_ok());

    // Only 1 project should exist
    let tasks = env.load_tasks();
    let with_project = tasks.iter().filter(|t| t.project_id.is_some()).count();
    let without_project = tasks.iter().filter(|t| t.project_id.is_none()).count();
    assert_eq!(with_project, 1);
    assert_eq!(without_project, 2);
}

#[test]
fn test_projects_all_tasks_completed() {
    let env = TestEnv::new();
    add_task(&env, "Task A", Some("Backend"));
    add_task(&env, "Task B", Some("Backend"));

    task::done::execute(env.storage(), 1).unwrap();
    task::done::execute(env.storage(), 2).unwrap();

    let result = project::list::execute(env.storage());
    assert!(result.is_ok());

    let tasks = env.load_tasks();
    let projects = env.storage().load_projects().unwrap();
    let backend_uuid = projects
        .iter()
        .find(|p| p.name == "Backend")
        .map(|p| p.uuid);
    let all_done = tasks
        .iter()
        .filter(|t| t.project_id == backend_uuid)
        .all(|t| t.completed);
    assert!(all_done);
}

// ─── list --project filter ────────────────────────────────────────────────────

#[test]
fn test_list_filter_by_project() {
    let env = TestEnv::new();
    add_task(&env, "Backend task 1", Some("Backend"));
    add_task(&env, "Backend task 2", Some("Backend"));
    add_task(&env, "Frontend task", Some("Frontend"));
    add_task(&env, "No project task", None);

    let result = task::list::execute(
        env.storage(),
        StatusFilter::All,
        None,
        None,
        None,
        vec![],
        Some("Backend".to_string()),
        None,
    );
    assert!(result.is_ok());
}

#[test]
fn test_list_filter_by_project_case_insensitive() {
    let env = TestEnv::new();
    add_task(&env, "Backend task", Some("Backend"));

    // lowercase "backend" should match "Backend"
    let result = task::list::execute(
        env.storage(),
        StatusFilter::All,
        None,
        None,
        None,
        vec![],
        Some("backend".to_string()),
        None,
    );
    assert!(result.is_ok());
}

#[test]
fn test_list_filter_by_nonexistent_project_fails() {
    let env = TestEnv::new();
    add_task(&env, "Task", Some("Backend"));

    let result = task::list::execute(
        env.storage(),
        StatusFilter::All,
        None,
        None,
        None,
        vec![],
        Some("Nonexistent".to_string()),
        None,
    );
    assert!(result.is_err());
}

#[test]
fn test_list_filter_project_with_status() {
    let env = TestEnv::new();
    add_task(&env, "Done task", Some("Backend"));
    add_task(&env, "Pending task", Some("Backend"));

    task::done::execute(env.storage(), 1).unwrap();

    let result = task::list::execute(
        env.storage(),
        StatusFilter::Pending,
        None,
        None,
        None,
        vec![],
        Some("Backend".to_string()),
        None,
    );
    assert!(result.is_ok());
}

#[test]
fn test_list_filter_project_with_sort() {
    use helpers::days_from_now;

    let env = TestEnv::new();

    task::add::execute(
        env.storage(),
        AddArgs {
            text: "Later task".to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: Some("Backend".to_string()),
            due: Some(days_from_now(10).to_string()),
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();

    task::add::execute(
        env.storage(),
        AddArgs {
            text: "Earlier task".to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: Some("Backend".to_string()),
            due: Some(days_from_now(2).to_string()),
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();

    let result = task::list::execute(
        env.storage(),
        StatusFilter::All,
        None,
        None,
        Some(SortBy::Due),
        vec![],
        Some("Backend".to_string()),
        None,
    );
    assert!(result.is_ok());
}

// ─── project via edit ─────────────────────────────────────────────────────────

#[test]
fn test_edit_assign_project() {
    let env = TestEnv::new();
    add_task(&env, "Task without project", None);

    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 1,
            text: None,
            priority: None,
            add_tag: vec![],
            remove_tag: vec![],
            project: Some("Backend".to_string()),
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
    let projects = env.storage().load_projects().unwrap();
    let backend_uuid = projects
        .iter()
        .find(|p| p.name == "Backend")
        .map(|p| p.uuid);
    assert_eq!(tasks[0].project_id, backend_uuid);
}

#[test]
fn test_edit_change_project() {
    let env = TestEnv::new();
    add_task(&env, "Task", Some("Backend"));

    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 1,
            text: None,
            priority: None,
            add_tag: vec![],
            remove_tag: vec![],
            project: Some("Frontend".to_string()),
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
    let projects = env.storage().load_projects().unwrap();
    let frontend_uuid = projects
        .iter()
        .find(|p| p.name == "Frontend")
        .map(|p| p.uuid);
    assert_eq!(tasks[0].project_id, frontend_uuid);
}

#[test]
fn test_edit_clear_project() {
    let env = TestEnv::new();
    add_task(&env, "Task", Some("Backend"));

    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 1,
            text: None,
            priority: None,
            add_tag: vec![],
            remove_tag: vec![],
            project: None,
            clear_project: true, // clear_project
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
    assert!(tasks[0].project_id.is_none());
}

#[test]
fn test_edit_no_change_when_same_project() {
    let env = TestEnv::new();
    add_task(&env, "Task", Some("Backend"));

    // Setting to same value should report "no changes"
    let result = task::edit::execute(
        env.storage(),
        EditArgs {
            id: 1,
            text: None,
            priority: None,
            add_tag: vec![],
            remove_tag: vec![],
            project: Some("Backend".to_string()),
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
}

// ─── project name validation ──────────────────────────────────────────────────

#[test]
fn test_add_empty_project_name_fails() {
    let env = TestEnv::new();

    let result = task::add::execute(
        env.storage(),
        AddArgs {
            text: "Task".to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: Some("".to_string()),
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    );

    assert!(result.is_err());
}

#[test]
fn test_add_project_name_too_long_fails() {
    let env = TestEnv::new();

    let result = task::add::execute(
        env.storage(),
        AddArgs {
            text: "Task".to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: Some("x".repeat(101)),
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    );

    assert!(result.is_err());
}

#[test]
fn test_add_project_name_exactly_max_length_ok() {
    let env = TestEnv::new();

    let result = task::add::execute(
        env.storage(),
        AddArgs {
            text: "Task".to_string(),
            priority: Priority::Medium,
            tag: vec![],
            project: Some("x".repeat(100)),
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    );

    assert!(result.is_ok());
}
