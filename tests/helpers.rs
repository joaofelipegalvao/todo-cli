#![allow(dead_code)]
//! Test utilities and fixtures

use chrono::NaiveDate;
use rustodo::models::{Priority, Recurrence, Task};
use rustodo::storage::Storage;
use rustodo::storage::memory::InMemoryStorage;

/// Test fixture providing an isolated in-memory storage environment
pub struct TestEnv {
    storage: InMemoryStorage,
}

impl TestEnv {
    /// Creates a new test environment with empty storage
    pub fn new() -> Self {
        Self {
            storage: InMemoryStorage::default(),
        }
    }

    /// Creates a test environment pre-populated with tasks
    pub fn with_tasks(tasks: Vec<Task>) -> Self {
        Self {
            storage: InMemoryStorage::with_tasks(tasks),
        }
    }

    /// Get reference to the storage
    pub fn storage(&self) -> &InMemoryStorage {
        &self.storage
    }

    /// Load all tasks from storage, excluding soft-deleted ones.
    ///
    /// This mirrors what every command handler sees: only visible tasks.
    /// Tests that check task text, order, or count should use this method.
    ///
    /// Use `load_all_tasks` when you specifically need to inspect tombstones.
    pub fn load_tasks(&self) -> Vec<Task> {
        self.storage
            .load()
            .expect("Failed to load tasks in test")
            .into_iter()
            .filter(|t| !t.is_deleted())
            .collect()
    }

    /// Load all tasks from storage, including soft-deleted tombstones.
    ///
    /// Useful for asserting that soft-delete propagates correctly in sync tests.
    pub fn load_all_tasks(&self) -> Vec<Task> {
        self.storage.load().expect("Failed to load tasks in test")
    }

    /// Save tasks to storage
    pub fn save_tasks(&self, tasks: &[Task]) {
        self.storage
            .save(tasks)
            .expect("Failed to save tasks in test")
    }

    /// Returns the number of visible (non-deleted) tasks.
    ///
    /// This is what the user sees — it matches the IDs shown in `todo list`.
    pub fn task_count(&self) -> usize {
        self.storage
            .load()
            .expect("Failed to load tasks in test")
            .iter()
            .filter(|t| !t.is_deleted())
            .count()
    }

    /// Returns the total number of tasks in storage, including tombstones.
    ///
    /// Useful for asserting that soft-deleted tasks are retained for sync.
    pub fn raw_task_count(&self) -> usize {
        self.storage.len()
    }

    /// Check if there are no visible tasks
    pub fn is_empty(&self) -> bool {
        self.task_count() == 0
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        Self::new()
    }
}

// === Sample Task Builders ===

/// Create a simple task with default values
pub fn simple_task(text: &str) -> Task {
    Task::new(text.to_string(), Priority::Medium, vec![], None, None, None)
}

/// Create a task with a specific priority
pub fn task_with_priority(text: &str, priority: Priority) -> Task {
    Task::new(text.to_string(), priority, vec![], None, None, None)
}

/// Create a task with tags
pub fn task_with_tags(text: &str, tags: Vec<&str>) -> Task {
    Task::new(
        text.to_string(),
        Priority::Medium,
        tags.into_iter().map(|s| s.to_string()).collect(),
        None,
        None,
        None,
    )
}

/// Create a task with a due date
pub fn task_with_due(text: &str, due_date: NaiveDate) -> Task {
    Task::new(
        text.to_string(),
        Priority::Medium,
        vec![],
        None,
        Some(due_date),
        None,
    )
}

/// Create a recurring task
pub fn recurring_task(text: &str, due_date: NaiveDate, recurrence: Recurrence) -> Task {
    Task::new(
        text.to_string(),
        Priority::Medium,
        vec![],
        None,
        Some(due_date),
        Some(recurrence),
    )
}

/// Create a fully customized task
pub fn custom_task(
    text: &str,
    priority: Priority,
    tags: Vec<&str>,
    due_date: Option<NaiveDate>,
    recurrence: Option<Recurrence>,
) -> Task {
    Task::new(
        text.to_string(),
        priority,
        tags.into_iter().map(|s| s.to_string()).collect(),
        None,
        due_date,
        recurrence,
    )
}

// === Date Helpers ===

/// Get today's date
pub fn today() -> NaiveDate {
    chrono::Local::now().naive_local().date()
}

/// Get yesterday's date
pub fn yesterday() -> NaiveDate {
    today() - chrono::Duration::days(1)
}

/// Get tomorrow's date
pub fn tomorrow() -> NaiveDate {
    today() + chrono::Duration::days(1)
}

/// Get date N days from today
pub fn days_from_now(days: i64) -> NaiveDate {
    today() + chrono::Duration::days(days)
}

/// Get date N days ago
pub fn days_ago(days: i64) -> NaiveDate {
    today() - chrono::Duration::days(days)
}
