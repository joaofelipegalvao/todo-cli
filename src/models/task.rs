use chrono::{DateTime, Local, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::filters::{DueFilter, StatusFilter};
use super::priority::Priority;
use super::recurrence::Recurrence;

/// Represents a single task in the todo list.
///
/// Each task contains a description, completion status, priority level,
/// optional tags for organization, optional due date for deadline tracking,
/// and recurrence pattern for repeating tasks.
///
/// # UUID for Sync
///
/// Each task has a stable UUID that uniquely identifies it across
/// different storage backends and sync operations. UUIDs are automatically
/// generated for new tasks and migrated for existing tasks on first load.
///
/// # Examples
///
/// ```
/// use rustodo::models::{Task, Priority};
///
/// let task = Task::new(
///     "Buy milk".to_string(),
///     Priority::Medium,
///     vec![],
///     None,
///     None,
///     None,
/// );
///
/// // UUID is automatically generated
/// assert!(!task.uuid.is_nil());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier for sync and conflict resolution.
    ///
    /// Automatically generated for new tasks via [`Uuid::new_v4()`].
    /// Old tasks without UUIDs are migrated on first load.
    #[serde(default)]
    pub uuid: Uuid,
    /// The task description/content
    pub text: String,
    /// Whether the task has been completed
    pub completed: bool,
    /// Priority level of the task
    pub priority: Priority,
    /// List of tags for categorization
    pub tags: Vec<String>,
    /// UUID of the project this task belongs to.
    ///
    /// Links to a [`Project`] entity. Use the project's `uuid` field.
    /// Old JSON with a `"project": "string"` field is migrated automatically
    /// by `JsonStorage::read_file` on first load.
    #[serde(default)]
    pub project_id: Option<Uuid>,

    /// Legacy string project name — kept for JSON migration only.
    ///
    /// Populated during deserialization of old files, then converted to
    /// `project_id` by `JsonStorage`. Should be `None` in all new tasks.
    #[serde(default, rename = "project", skip_serializing)]
    pub project_name_legacy: Option<String>,
    /// Optional due date for deadline tracking
    pub due_date: Option<NaiveDate>,
    /// Date when the task was created
    pub created_at: NaiveDate,
    /// Optional recurrence pattern (daily, weekly, monthly)
    pub recurrence: Option<Recurrence>,
    /// ID of the parent task (for recurring task chains)
    ///
    /// This links recurring tasks together, allowing:
    /// - Perfect deduplication even if text is edited
    /// - Tracking "families" of recurring tasks
    /// - Future features like `todo history <id>`
    #[serde(default)]
    pub parent_id: Option<Uuid>,
    /// IDs (1- based) of tasks that must be completed before this one
    #[serde(default)]
    pub depends_on: Vec<Uuid>,
    /// Date when the task was marked as completed.
    #[serde(default)]
    pub completed_at: Option<NaiveDate>,
    /// Timestamp of the last modification.
    ///
    /// Used by sync to determine which version of a task is more recent
    /// when merging changes from multiple devices. Updated automatically
    /// on every mutation via [`Task::touch`].
    ///
    /// Old tasks without this field are migrated with `None` on first load,
    /// which sync treats as "older than any real timestamp".
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
    /// Timestamp of soft deletion.
    ///
    /// When set, the task is considered deleted and hidden from all views.
    /// Kept in storage so that sync can propagate deletions across devices:
    /// if `deleted_at` is more recent than the remote's `updated_at`, the
    /// deletion wins (last-write-wins via [`Task::touch`] + `deleted_at`).
    ///
    /// `None` means the task is not deleted.
    #[serde(default)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Task {
    /// Creates a new pending task with a unique UUID.
    ///
    /// Sets `completed = false`, `created_at` to today, generates a new UUID,
    /// and leaves `parent_id`, `depends_on`, and `completed_at` at their zero values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustodo::models::{Task, Priority, Recurrence};
    /// use chrono::NaiveDate;
    ///
    /// // Minimal task
    /// let task = Task::new(
    ///     "Buy milk".to_string(),
    ///     Priority::Medium,
    ///     vec![],
    ///     None,
    ///     None,
    ///     None,
    /// );
    /// assert_eq!(task.text, "Buy milk");
    /// assert!(!task.completed);
    /// assert!(!task.uuid.is_nil()); // UUID is auto-generated
    ///
    /// // Task with all fields
    /// let due = NaiveDate::from_ymd_opt(2030, 6, 1).unwrap();
    /// let task = Task::new(
    ///     "Weekly review".to_string(),
    ///     Priority::High,
    ///     vec!["work".to_string()],
    ///     Some("Backend".to_string()),
    ///     Some(due),
    ///     Some(Recurrence::Weekly),
    /// );
    /// assert_eq!(task.priority, Priority::High);
    /// assert_eq!(task.recurrence, Some(Recurrence::Weekly));
    /// assert!(!task.uuid.is_nil());
    /// ```
    pub fn new(
        text: String,
        priority: Priority,
        tags: Vec<String>,
        project_id: Option<Uuid>,
        due_date: Option<NaiveDate>,
        recurrence: Option<Recurrence>,
    ) -> Self {
        Task {
            uuid: Uuid::new_v4(),
            text,
            completed: false,
            priority,
            tags,
            project_id,
            project_name_legacy: None,
            due_date,
            created_at: Local::now().naive_local().date(),
            recurrence,
            parent_id: None,
            depends_on: Vec::new(),
            completed_at: None,
            updated_at: Some(Utc::now()),
            deleted_at: None,
        }
    }

    /// Updates `updated_at` to the current UTC timestamp.
    ///
    /// Must be called after any mutation to ensure sync can determine
    /// which version of a task is more recent across devices.
    ///
    /// All command handlers (`edit`, `done`, `undone`, etc.) call this
    /// after modifying a task.
    pub fn touch(&mut self) {
        self.updated_at = Some(Utc::now());
    }

    /// Marks this task as soft-deleted.
    ///
    /// Sets `deleted_at` to the current UTC timestamp and calls [`touch`]
    /// so that sync can propagate the deletion via last-write-wins.
    /// The task remains in storage and is invisible to all normal views.
    ///
    /// Use [`is_deleted`] to filter deleted tasks out of lists.
    ///
    /// [`touch`]: Task::touch
    /// [`is_deleted`]: Task::is_deleted
    pub fn soft_delete(&mut self) {
        self.deleted_at = Some(Utc::now());
        self.touch();
    }

    /// Returns `true` if this task has been soft-deleted.
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    /// Marks this task as completed.
    pub fn mark_done(&mut self) {
        self.completed = true;
        self.completed_at = Some(Local::now().naive_local().date());
        self.touch();
    }

    /// Marks this task as pending (not completed).
    pub fn mark_undone(&mut self) {
        self.completed = false;
        self.completed_at = None;
        self.touch();
    }

    /// Checks if this is overdue.
    ///
    /// A task is considered overdue if it has a due date in the past
    /// and is not yet completed.
    pub fn is_overdue(&self) -> bool {
        if let Some(due) = self.due_date {
            let today = Local::now().naive_local().date();
            due < today && !self.completed
        } else {
            false
        }
    }

    /// Checks if this task is due soon (within the specified number of days).
    ///
    /// # Arguments
    ///
    /// * `days` - Number of days to look ahead
    ///
    /// # Returns
    ///
    /// `true` if the task is due within the specified number of days and
    /// is not yet completed, `false` otherwise.
    pub fn is_due_soon(&self, days: i64) -> bool {
        if let Some(due) = self.due_date {
            let today = Local::now().naive_local().date();
            let days_until = (due - today).num_days();
            days_until >= 0 && days_until <= days && !self.completed
        } else {
            false
        }
    }

    /// Checks if this task matches the given status filter.
    pub fn matches_status(&self, status: StatusFilter) -> bool {
        match status {
            StatusFilter::Pending => !self.completed,
            StatusFilter::Done => self.completed,
            StatusFilter::All => true,
        }
    }

    /// Checks if this task matches the given due date filter.
    pub fn matches_due_filter(&self, filter: DueFilter) -> bool {
        match filter {
            DueFilter::Overdue => self.is_overdue(),
            DueFilter::Soon => self.is_due_soon(7),
            DueFilter::WithDue => self.due_date.is_some(),
            DueFilter::NoDue => self.due_date.is_none(),
        }
    }

    /// Returns true if any dependency task is still pending.
    ///
    /// `all_tasks` is the full 0-indexed task list; IDs in `depends_on` are 1-based.
    pub fn is_blocked(&self, all_tasks: &[Task]) -> bool {
        self.depends_on.iter().any(|dep_uuid| {
            all_tasks
                .iter()
                .find(|t| t.uuid == *dep_uuid)
                .map(|t| !t.completed)
                .unwrap_or(false)
        })
    }

    /// Returns the IDs of blocking (still-pending) dependencies.
    pub fn blocking_deps(&self, all_tasks: &[Task]) -> Vec<Uuid> {
        self.depends_on
            .iter()
            .copied()
            .filter(|dep_uuid| {
                all_tasks
                    .iter()
                    .find(|t| t.uuid == *dep_uuid)
                    .map(|t| !t.completed)
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Creates a new task for the next recurrence cycle.
    ///
    /// # Arguments
    ///
    /// * `parent_uuid` - The UUID of the current task (to link recurring tasks)
    ///
    /// # Returns
    ///
    /// `Some(Task)` if the task is recurring and has a due date,
    /// `None` otherwise.
    ///
    /// # Behavior
    ///
    /// - Preserves: text, priority, tags, recurrence pattern
    /// - Resets: completed = false
    /// - Updates: due_date (calculated from recurrence), created_at (now), updated_at (now)
    /// - Generates: New UUID for the next occurrence
    /// - Sets: parent_id (to link the chain)
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use rustodo::models::{Task, Priority, Recurrence};
    ///
    /// let task = Task::new(
    ///     "Weekly review".to_string(),
    ///     Priority::Medium,
    ///     vec![],
    ///     None,
    ///     Some(NaiveDate::from_ymd_opt(2025, 2, 10).unwrap()),
    ///     Some(Recurrence::Weekly),
    /// );
    ///
    /// let parent_uuid = task.uuid;
    /// let next = task.create_next_recurrence(parent_uuid).unwrap();
    /// assert_eq!(
    ///     next.due_date,
    ///     Some(NaiveDate::from_ymd_opt(2025, 2, 17).unwrap())
    /// );
    /// assert!(next.parent_id.is_some());
    /// assert!(next.updated_at.is_some());
    /// assert_ne!(next.uuid, task.uuid);
    /// ```
    pub fn create_next_recurrence(&self, parent_uuid: Uuid) -> Option<Task> {
        let recurrence = self.recurrence?;
        let current_due = self.due_date?;
        let next_due = recurrence.next_date(current_due);

        let mut next_task = Task::new(
            self.text.clone(),
            self.priority,
            self.tags.clone(),
            self.project_id,
            Some(next_due),
            Some(recurrence),
        );

        next_task.parent_id = Some(parent_uuid);
        // Dependencies are NOT propagated to recurrences — each occurrence stands alone.
        Some(next_task)
    }

    #[allow(dead_code)]
    pub fn is_recurring(&self) -> bool {
        self.recurrence.is_some()
    }
}

/// Counts the tasks of a project by UUID, returning (total, completed).
///
/// # Example
///
/// ```
/// use rustodo::models::{Task, Project, Priority};
///
/// let project = Project::new("Work".into());
/// let tasks = vec![
///     Task::new("A".to_string(), Priority::Medium, vec![], Some(project.uuid), None, None),
///     Task::new("B".to_string(), Priority::Medium, vec![], Some(project.uuid), None, None),
/// ];
/// let (total, done) = rustodo::models::count_by_project(&tasks, project.uuid);
/// assert_eq!(total, 2);
/// assert_eq!(done, 0);
/// ```
pub fn count_by_project(tasks: &[Task], project_uuid: uuid::Uuid) -> (usize, usize) {
    let matching: Vec<_> = tasks
        .iter()
        .filter(|t| !t.is_deleted() && t.project_id == Some(project_uuid))
        .collect();

    let total = matching.len();
    let done = matching.iter().filter(|t| t.completed).count();
    (total, done)
}

/// Detects a dependency cycle using iterative DFS.
///
/// Returns `Err` with the cycle description if adding `dep_id → task_id`
/// would create a cycle, `Ok(())` otherwise.
///
/// `tasks` is the full 0-indexed list; IDs are 1-based.
pub(crate) fn detect_cycle(
    tasks: &[Task],
    task_uuid: Uuid,
    new_dep_uuid: Uuid,
) -> Result<(), String> {
    // Would adding "task_id depends on new_dep_id" create a cycle?
    // A cycle exists if task_id is reachable FROM new_dep_id via depends_on edges.
    // i.e. check if task_id appears in the transitive deps of new_dep_id.
    let mut visited = std::collections::HashSet::new();
    let mut stack = vec![new_dep_uuid];

    while let Some(current_uuid) = stack.pop() {
        if current_uuid == task_uuid {
            // Resolver UUIDs → números para o display
            let task_num = tasks
                .iter()
                .position(|t| t.uuid == task_uuid)
                .map(|i| i + 1)
                .unwrap_or(0);
            let dep_num = tasks
                .iter()
                .position(|t| t.uuid == new_dep_uuid)
                .map(|i| i + 1)
                .unwrap_or(0);

            return Err(format!(
                "Adding this dependency would create a cycle: \
                task #{} → task #{} → ... → task #{}",
                task_num, dep_num, task_num
            ));
        }

        if visited.insert(current_uuid)
            && let Some(t) = tasks.iter().find(|t| t.uuid == current_uuid)
        {
            for &d in &t.depends_on {
                stack.push(d);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    fn make_task(text: &str) -> Task {
        Task::new(text.to_string(), Priority::Medium, vec![], None, None, None)
    }

    #[test]
    fn test_is_blocked_no_deps() {
        let task = make_task("A");
        assert!(!task.is_blocked(&[]));
    }

    #[test]
    fn test_is_blocked_pending_dep() {
        let dep = make_task("Dep");
        let dep_uuid = dep.uuid;
        let mut task = make_task("Task");
        task.depends_on = vec![dep_uuid];
        assert!(task.is_blocked(&[dep]));
    }

    #[test]
    fn test_is_blocked_completed_dep() {
        let mut dep = make_task("Dep");
        let dep_uuid = dep.uuid;
        dep.completed = true;
        let mut task = make_task("Task");
        task.depends_on = vec![dep_uuid];
        assert!(!task.is_blocked(&[dep]));
    }

    #[test]
    fn test_detect_cycle_direct() {
        let mut tasks = vec![make_task("A"), make_task("B")];
        // A depends on B
        tasks[0].depends_on = vec![tasks[1].uuid];
        // Adding B depends on A should fail
        let result = detect_cycle(&tasks, tasks[1].uuid, tasks[0].uuid);
        assert!(result.is_err());
    }

    #[test]
    fn test_detect_no_cycle() {
        let tasks = vec![make_task("A"), make_task("B"), make_task("C")];
        // A->B, adding C->A should be fine
        let result = detect_cycle(&tasks, tasks[2].uuid, tasks[0].uuid);
        assert!(result.is_ok());
    }

    #[test]
    fn test_detect_transitive_cycle() {
        let mut tasks = vec![make_task("A"), make_task("B"), make_task("C")];
        // A depends on B, B depends on C
        tasks[0].depends_on = vec![tasks[1].uuid];
        tasks[1].depends_on = vec![tasks[2].uuid];
        // Adding C depends on A should fail (C->A->B->C)
        let result = detect_cycle(&tasks, tasks[2].uuid, tasks[0].uuid);
        assert!(result.is_err());
    }

    #[test]
    fn test_blocking_deps_returns_pending_only() {
        let mut dep1 = make_task("Dep1");
        dep1.completed = true;
        let dep1_uuid = dep1.uuid;
        let dep2 = make_task("Dep2");
        let dep2_uuid = dep2.uuid;
        let mut task = make_task("Task");
        task.depends_on = vec![dep1_uuid, dep2_uuid];
        let blocking = task.blocking_deps(&[dep1, dep2]);
        assert_eq!(blocking, vec![dep2_uuid]);
    }

    #[test]
    fn test_updated_at_set_on_new() {
        let task = make_task("A");
        assert!(task.updated_at.is_some());
    }

    #[test]
    fn test_deleted_at_none_on_new() {
        let task = make_task("A");
        assert!(task.deleted_at.is_none());
        assert!(!task.is_deleted());
    }

    #[test]
    fn test_soft_delete_sets_deleted_at() {
        let mut task = make_task("A");
        assert!(!task.is_deleted());
        task.soft_delete();
        assert!(task.is_deleted());
        assert!(task.deleted_at.is_some());
    }

    #[test]
    fn test_soft_delete_also_updates_updated_at() {
        let mut task = make_task("A");
        let before = task.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(5));
        task.soft_delete();
        assert!(task.updated_at > before);
    }

    #[test]
    fn test_soft_delete_deleted_at_lte_updated_at() {
        let mut task = make_task("A");
        task.soft_delete();
        assert!(task.updated_at >= task.deleted_at);
    }

    #[test]
    fn test_touch_updates_timestamp() {
        let mut task = make_task("A");
        let before = task.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(5));
        task.touch();
        assert!(task.updated_at > before);
    }

    #[test]
    fn test_mark_done_updates_timestamp() {
        let mut task = make_task("A");
        let before = task.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(5));
        task.mark_done();
        assert!(task.updated_at > before);
    }

    #[test]
    fn test_mark_undone_updates_timestamp() {
        let mut task = make_task("A");
        task.mark_done();
        let before = task.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(5));
        task.mark_undone();
        assert!(task.updated_at > before);
    }

    fn make_recurring(recurrence: Option<Recurrence>, due: Option<NaiveDate>) -> Task {
        Task::new(
            "Test".to_string(),
            Priority::Medium,
            vec![],
            None,
            due,
            recurrence,
        )
    }

    #[test]
    fn test_daily_recurrence() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 10).unwrap();
        let task = make_recurring(Some(Recurrence::Daily), Some(date));
        let parent_uuid = task.uuid;
        let next = task.create_next_recurrence(parent_uuid).unwrap();
        assert_eq!(
            next.due_date,
            Some(NaiveDate::from_ymd_opt(2026, 2, 11).unwrap())
        );
        assert_eq!(next.parent_id, Some(parent_uuid));
    }

    #[test]
    fn test_weekly_recurrence() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 10).unwrap();
        let task = make_recurring(Some(Recurrence::Weekly), Some(date));
        let parent_uuid = task.uuid;
        let next = task.create_next_recurrence(parent_uuid).unwrap();
        assert_eq!(
            next.due_date,
            Some(NaiveDate::from_ymd_opt(2026, 2, 17).unwrap())
        );
    }

    #[test]
    fn test_monthly_recurrence() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 10).unwrap();
        let task = make_recurring(Some(Recurrence::Monthly), Some(date));
        let parent_uuid = task.uuid;
        let next = task.create_next_recurrence(parent_uuid).unwrap();
        assert_eq!(
            next.due_date,
            Some(NaiveDate::from_ymd_opt(2026, 3, 10).unwrap())
        );
    }

    #[test]
    fn test_monthly_boundary_case() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 31).unwrap();
        let task = make_recurring(Some(Recurrence::Monthly), Some(date));
        let parent_uuid = task.uuid;
        let next = task.create_next_recurrence(parent_uuid).unwrap();
        assert_eq!(
            next.due_date,
            Some(NaiveDate::from_ymd_opt(2026, 2, 28).unwrap())
        );
    }

    #[test]
    fn test_no_recurrence_returns_none() {
        let task = make_recurring(None, Some(NaiveDate::from_ymd_opt(2026, 2, 10).unwrap()));
        let parent_uuid = task.uuid;
        assert!(task.create_next_recurrence(parent_uuid).is_none());
    }

    #[test]
    fn test_no_due_date_returns_none() {
        let task = make_recurring(Some(Recurrence::Daily), None);
        let parent_uuid = task.uuid;
        assert!(task.create_next_recurrence(parent_uuid).is_none());
    }

    #[test]
    fn test_recurrence_next_is_not_deleted() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 10).unwrap();
        let mut task = make_recurring(Some(Recurrence::Daily), Some(date));
        task.soft_delete();
        let parent_uuid = task.uuid;
        let next = task.create_next_recurrence(parent_uuid).unwrap();
        assert!(
            !next.is_deleted(),
            "next recurrence must not inherit deleted_at"
        );
    }

    #[test]
    fn test_project_preserved_in_recurrence() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 10).unwrap();
        let project_uuid = Uuid::new_v4();
        let mut task = make_recurring(Some(Recurrence::Daily), Some(date));
        task.project_id = Some(project_uuid);
        let parent_uuid = task.uuid;
        let next = task.create_next_recurrence(parent_uuid).unwrap();
        assert_eq!(next.project_id, Some(project_uuid));
    }

    #[test]
    fn test_deps_not_propagated_to_recurrence() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 10).unwrap();
        let mut task = make_recurring(Some(Recurrence::Daily), Some(date));
        task.depends_on = vec![Uuid::new_v4(), Uuid::new_v4()];
        let parent_uuid = task.uuid;
        let next = task.create_next_recurrence(parent_uuid).unwrap();
        assert!(
            next.depends_on.is_empty(),
            "recurrences should not inherit dependencies"
        );
    }

    #[test]
    fn test_count_by_project_basic() {
        let project_uuid = Uuid::new_v4();
        let other_uuid = Uuid::new_v4();
        let mut t1 = Task::new(
            "A".to_string(),
            Priority::Medium,
            vec![],
            Some(project_uuid),
            None,
            None,
        );
        let t2 = Task::new(
            "B".to_string(),
            Priority::Medium,
            vec![],
            Some(project_uuid),
            None,
            None,
        );
        let _t3 = Task::new(
            "C".to_string(),
            Priority::Medium,
            vec![],
            Some(other_uuid),
            None,
            None,
        );
        t1.completed = true;

        let tasks = vec![t1, t2, _t3];
        let (total, done) = count_by_project(&tasks, project_uuid);
        assert_eq!(total, 2);
        assert_eq!(done, 1);
    }

    #[test]
    fn test_count_by_project_excludes_deleted() {
        let project_uuid = Uuid::new_v4();
        let mut t1 = Task::new(
            "A".to_string(),
            Priority::Medium,
            vec![],
            Some(project_uuid),
            None,
            None,
        );
        t1.soft_delete();
        let t2 = Task::new(
            "B".to_string(),
            Priority::Medium,
            vec![],
            Some(project_uuid),
            None,
            None,
        );

        let tasks = vec![t1, t2];
        let (total, _) = count_by_project(&tasks, project_uuid);
        assert_eq!(total, 1, "deleted tasks should not be counted");
    }

    #[test]
    fn test_count_by_project_case_insensitive() {
        // UUID-based lookup is exact — no case insensitivity needed
        let project_uuid = Uuid::new_v4();
        let t1 = Task::new(
            "A".to_string(),
            Priority::Medium,
            vec![],
            Some(project_uuid),
            None,
            None,
        );
        let tasks = vec![t1];

        let (total, _) = count_by_project(&tasks, project_uuid);
        assert_eq!(total, 1);
    }
}
