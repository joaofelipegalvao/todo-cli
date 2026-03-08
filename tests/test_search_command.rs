//! Integration tests for the `search` command
//!
//! Covers:
//! - Basic search finds matching tasks
//! - Case-insensitive matching
//! - No results returns error
//! - Status filter: pending only, done only, all
//! - Tag filter combined with search
//! - Project filter combined with search
//! - Multiple filters combined
//! - Partial match in description

mod helpers;
use helpers::TestEnv;
use rustodo::cli::AddArgs;
use rustodo::commands::{search, task_add, task_done};
use rustodo::models::{Priority, StatusFilter};
use rustodo::storage::Storage;

// ─── helpers ─────────────────────────────────────────────────────────────────

fn add_task(env: &TestEnv, text: &str, tags: Vec<&str>, project: Option<&str>) -> usize {
    task_add::execute(
        env.storage(),
        AddArgs {
            text: text.to_string(),
            priority: Priority::Medium,
            tag: tags.into_iter().map(|s| s.to_string()).collect(),
            project: project.map(|s| s.to_string()),
            due: None,
            recurrence: None,
            depends_on: vec![],
        },
    )
    .unwrap();
    env.task_count()
}

fn add_simple(env: &TestEnv, text: &str) -> usize {
    add_task(env, text, vec![], None)
}

// ─── basic search ─────────────────────────────────────────────────────────────

#[test]
fn test_search_finds_matching_task() {
    let env = TestEnv::new();
    add_simple(&env, "Buy milk");
    add_simple(&env, "Write tests");

    let result = search::execute(
        env.storage(),
        "milk".to_string(),
        vec![],
        None,
        StatusFilter::All,
    );
    assert!(result.is_ok());
}

#[test]
fn test_search_partial_match() {
    let env = TestEnv::new();
    add_simple(&env, "Implement authentication");
    add_simple(&env, "Write documentation");

    let result = search::execute(
        env.storage(),
        "auth".to_string(),
        vec![],
        None,
        StatusFilter::All,
    );
    assert!(result.is_ok());
}

#[test]
fn test_search_case_insensitive() {
    let env = TestEnv::new();
    add_simple(&env, "Buy MILK");

    let result = search::execute(
        env.storage(),
        "milk".to_string(),
        vec![],
        None,
        StatusFilter::All,
    );
    assert!(result.is_ok());
}

#[test]
fn test_search_no_results_fails() {
    let env = TestEnv::new();
    add_simple(&env, "Buy milk");

    let result = search::execute(
        env.storage(),
        "nonexistent".to_string(),
        vec![],
        None,
        StatusFilter::All,
    );
    assert!(result.is_err());
}

#[test]
fn test_search_empty_storage_fails() {
    let env = TestEnv::new();

    let result = search::execute(
        env.storage(),
        "anything".to_string(),
        vec![],
        None,
        StatusFilter::All,
    );
    assert!(result.is_err());
}

// ─── status filter ────────────────────────────────────────────────────────────

#[test]
fn test_search_status_all_returns_both() {
    let env = TestEnv::new();
    add_simple(&env, "Buy milk");
    add_simple(&env, "Buy bread");

    task_done::execute(env.storage(), 1).unwrap();

    let result = search::execute(
        env.storage(),
        "buy".to_string(),
        vec![],
        None,
        StatusFilter::All,
    );
    assert!(result.is_ok());

    // Verify both tasks match
    let tasks = env.load_tasks();
    let matching: Vec<_> = tasks
        .iter()
        .filter(|t| t.text.to_lowercase().contains("buy"))
        .collect();
    assert_eq!(matching.len(), 2);
}

#[test]
fn test_search_status_pending_excludes_done() {
    let env = TestEnv::new();
    add_simple(&env, "Buy milk");
    add_simple(&env, "Buy bread");

    task_done::execute(env.storage(), 1).unwrap();

    let result = search::execute(
        env.storage(),
        "buy".to_string(),
        vec![],
        None,
        StatusFilter::Pending,
    );
    assert!(result.is_ok());

    // Only pending tasks should match
    let tasks = env.load_tasks();
    let pending_matching: Vec<_> = tasks
        .iter()
        .filter(|t| t.text.to_lowercase().contains("buy") && !t.completed)
        .collect();
    assert_eq!(pending_matching.len(), 1);
    assert_eq!(pending_matching[0].text, "Buy bread");
}

#[test]
fn test_search_status_done_excludes_pending() {
    let env = TestEnv::new();
    add_simple(&env, "Buy milk");
    add_simple(&env, "Buy bread");

    task_done::execute(env.storage(), 1).unwrap();

    let result = search::execute(
        env.storage(),
        "buy".to_string(),
        vec![],
        None,
        StatusFilter::Done,
    );
    assert!(result.is_ok());

    let tasks = env.load_tasks();
    let done_matching: Vec<_> = tasks
        .iter()
        .filter(|t| t.text.to_lowercase().contains("buy") && t.completed)
        .collect();
    assert_eq!(done_matching.len(), 1);
    assert_eq!(done_matching[0].text, "Buy milk");
}

#[test]
fn test_search_status_pending_no_results_fails() {
    let env = TestEnv::new();
    add_simple(&env, "Buy milk");

    task_done::execute(env.storage(), 1).unwrap();

    // All "buy" tasks are done — pending search should fail
    let result = search::execute(
        env.storage(),
        "buy".to_string(),
        vec![],
        None,
        StatusFilter::Pending,
    );
    assert!(result.is_err());
}

#[test]
fn test_search_status_done_no_results_fails() {
    let env = TestEnv::new();
    add_simple(&env, "Buy milk");

    // Task is pending — done search should fail
    let result = search::execute(
        env.storage(),
        "buy".to_string(),
        vec![],
        None,
        StatusFilter::Done,
    );
    assert!(result.is_err());
}

// ─── tag filter ───────────────────────────────────────────────────────────────

#[test]
fn test_search_with_tag_filter() {
    let env = TestEnv::new();
    add_task(&env, "Fix bug in auth", vec!["work"], None);
    add_task(&env, "Fix bug in UI", vec!["personal"], None);

    let result = search::execute(
        env.storage(),
        "fix bug".to_string(),
        vec!["work".to_string()],
        None,
        StatusFilter::All,
    );
    assert!(result.is_ok());

    // Only "work" tagged task should match
    let tasks = env.load_tasks();
    let work_matches: Vec<_> = tasks
        .iter()
        .filter(|t| {
            t.text.to_lowercase().contains("fix bug") && t.tags.contains(&"work".to_string())
        })
        .collect();
    assert_eq!(work_matches.len(), 1);
}

#[test]
fn test_search_tag_filter_no_match_fails() {
    let env = TestEnv::new();
    add_task(&env, "Fix bug", vec!["work"], None);

    let result = search::execute(
        env.storage(),
        "fix bug".to_string(),
        vec!["personal".to_string()], // tag doesn't exist on matching task
        None,
        StatusFilter::All,
    );
    assert!(result.is_err());
}

// ─── project filter ───────────────────────────────────────────────────────────

#[test]
fn test_search_with_project_filter() {
    let env = TestEnv::new();
    add_task(&env, "Fix bug in API", vec![], Some("Backend"));
    add_task(&env, "Fix bug in button", vec![], Some("Frontend"));

    let result = search::execute(
        env.storage(),
        "fix bug".to_string(),
        vec![],
        Some("Backend".to_string()),
        StatusFilter::All,
    );
    assert!(result.is_ok());

    let tasks = env.load_tasks();
    let projects = env.storage().load_projects().unwrap();
    let backend_uuid = projects
        .iter()
        .find(|p| p.name == "Backend")
        .map(|p| p.uuid);
    let backend_matches: Vec<_> = tasks
        .iter()
        .filter(|t| t.text.to_lowercase().contains("fix bug") && t.project_id == backend_uuid)
        .collect();
    assert_eq!(backend_matches.len(), 1);
}

#[test]
fn test_search_project_filter_case_insensitive() {
    let env = TestEnv::new();
    add_task(&env, "Deploy service", vec![], Some("Backend"));

    let result = search::execute(
        env.storage(),
        "deploy".to_string(),
        vec![],
        Some("backend".to_string()), // lowercase
        StatusFilter::All,
    );
    assert!(result.is_ok());
}

// ─── combined filters ─────────────────────────────────────────────────────────

#[test]
fn test_search_tag_and_project_and_status() {
    let env = TestEnv::new();
    add_task(&env, "Fix critical bug", vec!["urgent"], Some("Backend"));
    add_task(&env, "Fix minor bug", vec!["work"], Some("Backend"));
    add_task(&env, "Fix UI bug", vec!["urgent"], Some("Frontend"));

    task_done::execute(env.storage(), 1).unwrap();

    // Pending + urgent + Backend
    let result = search::execute(
        env.storage(),
        "fix".to_string(),
        vec!["urgent".to_string()],
        Some("Backend".to_string()),
        StatusFilter::Pending,
    );
    // Task 1 matches but is done, Task 3 matches urgent but is Frontend
    assert!(result.is_err(), "no pending urgent Backend fix tasks");
}
