//! Input validation for task data
//!
//! This module provides comprehensive validation functions to ensure data integrity
//! before persisting tasks to storage.

use crate::error::TodoError;
use crate::models::{Recurrence, Task};
use chrono::NaiveDate;
use uuid::Uuid;

/// Returns the indices (into `items`) of all non-deleted entries,
/// preserving their original order.
///
/// Works with any type — [`Task`], [`Project`], [`Note`], [`Resource`] — as
/// long as you supply an `is_deleted` predicate.
///
/// Use this to map a user-facing 1-based ID (counting only visible items)
/// back to a real position in the storage slice.
///
/// [`Project`]: crate::models::Project
/// [`Note`]: crate::models::Note
/// [`Resource`]: crate::models::Resource
pub fn visible_indices<T>(items: &[T], is_deleted: impl Fn(&T) -> bool) -> Vec<usize> {
    items
        .iter()
        .enumerate()
        .filter(|(_, item)| !is_deleted(item))
        .map(|(i, _)| i)
        .collect()
}

/// Resolves a 1-based visible ID to an immutable reference to the item.
///
/// Use for read-only operations (`show`, `preview`).
///
/// # Errors
/// Returns `TodoError::InvalidTaskId` if the ID is out of range.
pub fn resolve_visible<'a, T>(
    items: &'a [T],
    id: usize,
    is_deleted: impl Fn(&T) -> bool,
) -> Result<&'a T, TodoError> {
    let indices = visible_indices(items, is_deleted);
    validate_task_id(id, indices.len())?;
    Ok(&items[indices[id - 1]])
}

/// Resolves a 1-based visible ID to the real index in the slice.
///
/// Use for mutable operations (`edit`, `remove`).
///
/// # Errors
/// Returns `TodoError::InvalidTaskId` if the ID is out of range.
pub fn resolve_visible_index<T>(
    items: &[T],
    id: usize,
    is_deleted: impl Fn(&T) -> bool,
) -> Result<usize, TodoError> {
    let indices = visible_indices(items, is_deleted);
    validate_task_id(id, indices.len())?;
    Ok(indices[id - 1])
}

/// Resolves a user-facing 1-based ID to its UUID, considering only visible
/// (non-deleted) tasks.
///
/// # Errors
/// Returns `TodoError::InvalidTaskId` if the ID is out of range.
pub fn resolve_uuid_visible(id: usize, tasks: &[Task]) -> Result<Uuid, TodoError> {
    let indices = visible_indices(tasks, |t| t.is_deleted());
    validate_task_id(id, indices.len())?;
    Ok(tasks[indices[id - 1]].uuid)
}

/// Resolves a 1-based numeric task ID to its UUID.
///
/// **Prefer [`resolve_uuid_visible`] in command handlers.**
///
/// # Errors
/// Returns `TodoError::InvalidTaskId` if the ID is out of range.
pub fn resolve_uuid(id: usize, tasks: &[Task]) -> Result<Uuid, TodoError> {
    validate_task_id(id, tasks.len())?;
    Ok(tasks[id - 1].uuid)
}

/// Validates that a task ID is within valid range
///
/// Task IDs are 1-based (displayed to users as 1, 2, 3, etc.)
/// but stored in Vec at indices 0, 1, 2, etc.
///
/// # Arguments
///
/// * `id` - The task ID to validate (1-based)
/// * `max` - The maximum valid ID (total number of tasks)
///
/// # Errors
///
/// Returns `TodoError::InvalidTaskId` if the ID is 0 or greater than max.
pub fn validate_task_id(id: usize, max: usize) -> Result<(), TodoError> {
    if id == 0 || id > max {
        return Err(TodoError::InvalidTaskId { id, max });
    }
    Ok(())
}

/// Validates task text is not empty and within length limits
///
/// # Rules
///
/// - Text cannot be empty or whitespace-only
/// - Text cannot exceed 500 characters (trimmed)
///
/// # Errors
///
/// Returns:
/// - `TodoError::EmptyTaskText` if text is empty
/// - `TodoError::TaskTextTooLong` if text exceeds 500 characters
pub fn validate_task_text(text: &str) -> Result<(), TodoError> {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return Err(TodoError::EmptyTaskText);
    }

    const MAX_LENGTH: usize = 500;
    if trimmed.len() > MAX_LENGTH {
        return Err(TodoError::TaskTextTooLong {
            max: MAX_LENGTH,
            actual: trimmed.len(),
        });
    }

    Ok(())
}

/// Validates tags are properly formatted and unique
///
/// # Rules
///
/// - Tags cannot be empty or whitespace-only
/// - Tags cannot exceed 50 characters
/// - Tags can only contain alphanumeric characters, hyphens, and underscores
/// - No duplicate tags (case-insensitive)
///
/// # Errors
///
/// Returns:
/// - `TodoError::EmptyTag` if any tag is empty
/// - `TodoError::TagTooLong` if any tag exceeds 50 characters
/// - `TodoError::InvalidTagFormat` if any tag contains invalid characters
/// - `TodoError::DuplicateTag` if there are duplicate tags
pub fn validate_tags(tags: &[String]) -> Result<(), TodoError> {
    use std::collections::HashSet;

    const MAX_TAG_LENGTH: usize = 50;

    for tag in tags {
        let trimmed = tag.trim();

        if trimmed.is_empty() {
            return Err(TodoError::EmptyTag);
        }

        if trimmed.len() > MAX_TAG_LENGTH {
            return Err(TodoError::TagTooLong {
                max: MAX_TAG_LENGTH,
                actual: trimmed.len(),
            });
        }

        let valid_chars = trimmed
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_');

        if !valid_chars {
            return Err(TodoError::InvalidTagFormat {
                tag: trimmed.to_string(),
            });
        }
    }

    let mut seen = HashSet::new();
    for tag in tags {
        let lowercase = tag.to_lowercase();
        if !seen.insert(lowercase.clone()) {
            return Err(TodoError::DuplicateTag { tag: tag.clone() });
        }
    }

    Ok(())
}

/// Validates a project name is not empty and within length limits.
///
/// # Rules
///
/// - Name cannot be empty or whitespace-only
/// - Name cannot exceed 100 characters (trimmed)
///
/// # Errors
///
/// Returns:
/// - `TodoError::EmptyProjectName` if the name is empty
/// - `TodoError::ProjectNameTooLong` if the name exceeds 100 characters
pub fn validate_project_name(name: &str) -> Result<(), TodoError> {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        return Err(TodoError::EmptyProjectName);
    }

    const MAX_LENGTH: usize = 100;
    if trimmed.len() > MAX_LENGTH {
        return Err(TodoError::ProjectNameTooLong {
            max: MAX_LENGTH,
            actual: trimmed.len(),
        });
    }

    Ok(())
}

/// Validates due date is not in the past (for new tasks)
///
/// # Arguments
///
/// * `due_date` - Optional due date to validate
/// * `allow_past` - If true, allows past dates (for editing existing tasks)
///
/// # Errors
///
/// Returns `TodoError::DueDateInPast` if date is in the past and `allow_past` is false
pub fn validate_due_date(due_date: Option<NaiveDate>, allow_past: bool) -> Result<(), TodoError> {
    if let Some(due) = due_date
        && !allow_past
    {
        let today = chrono::Local::now().naive_local().date();
        if due < today {
            return Err(TodoError::DueDateInPast { date: due });
        }
    }
    Ok(())
}

/// Validates recurrence pattern has a due date
///
/// Recurring tasks MUST have a due date to calculate the next occurrence.
///
/// # Errors
///
/// Returns `TodoError::RecurrenceRequiresDueDate` if recurrence is set but due_date is None
pub fn validate_recurrence(
    recurrence: Option<Recurrence>,
    due_date: Option<NaiveDate>,
) -> Result<(), TodoError> {
    if recurrence.is_some() && due_date.is_none() {
        return Err(TodoError::RecurrenceRequiresDueDate);
    }
    Ok(())
}

/// Validates a complete task before saving
///
/// Runs all validation checks on a task.
///
/// # Arguments
///
/// * `task` - The task to validate
/// * `is_new` - If true, disallows past due dates; if false, allows them
///
/// # Errors
///
/// Returns the first validation error encountered, or Ok(()) if all checks pass
#[allow(dead_code)]
pub fn validate_task(task: &Task, is_new: bool) -> Result<(), TodoError> {
    validate_task_text(&task.text)?;
    validate_tags(&task.tags)?;
    validate_due_date(task.due_date, !is_new)?;
    validate_recurrence(task.recurrence, task.due_date)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Priority;

    fn make_task(text: &str) -> Task {
        Task::new(text.to_string(), Priority::Medium, vec![], None, None, None)
    }

    #[test]
    fn test_visible_indices_excludes_deleted() {
        let mut tasks = vec![make_task("A"), make_task("B"), make_task("C")];
        tasks[1].soft_delete();
        let indices = visible_indices(&tasks, |t| t.is_deleted());
        assert_eq!(indices, vec![0, 2]);
    }

    #[test]
    fn test_visible_indices_all_visible() {
        let tasks = vec![make_task("A"), make_task("B")];
        assert_eq!(visible_indices(&tasks, |t| t.is_deleted()), vec![0, 1]);
    }

    #[test]
    fn test_visible_indices_all_deleted() {
        let mut tasks = vec![make_task("A"), make_task("B")];
        tasks[0].soft_delete();
        tasks[1].soft_delete();
        assert!(visible_indices(&tasks, |t| t.is_deleted()).is_empty());
    }

    #[test]
    fn test_resolve_visible_returns_correct_item() {
        let mut tasks = vec![make_task("A"), make_task("B"), make_task("C")];
        tasks[1].soft_delete();
        let item = resolve_visible(&tasks, 2, |t| t.is_deleted()).unwrap();
        assert_eq!(item.text, "C");
    }

    #[test]
    fn test_resolve_visible_out_of_range() {
        let tasks = vec![make_task("A")];
        assert!(resolve_visible(&tasks, 2, |t| t.is_deleted()).is_err());
    }

    #[test]
    fn test_resolve_visible_index_returns_real_index() {
        let mut tasks = vec![make_task("A"), make_task("B"), make_task("C")];
        tasks[1].soft_delete();
        let idx = resolve_visible_index(&tasks, 2, |t| t.is_deleted()).unwrap();
        assert_eq!(idx, 2);
    }

    #[test]
    fn test_resolve_visible_index_out_of_range() {
        let tasks = vec![make_task("A")];
        assert!(resolve_visible_index(&tasks, 2, |t| t.is_deleted()).is_err());
    }

    #[test]
    fn test_resolve_uuid_visible_skips_deleted() {
        let mut tasks = vec![make_task("A"), make_task("B"), make_task("C")];
        let uuid_a = tasks[0].uuid;
        let uuid_c = tasks[2].uuid;
        tasks[1].soft_delete();

        assert_eq!(resolve_uuid_visible(1, &tasks).unwrap(), uuid_a);
        assert_eq!(resolve_uuid_visible(2, &tasks).unwrap(), uuid_c);
        assert!(resolve_uuid_visible(3, &tasks).is_err());
    }

    #[test]
    fn test_validate_project_name_valid() {
        assert!(validate_project_name("Work").is_ok());
        assert!(validate_project_name("My project").is_ok());
        assert!(validate_project_name("Rust learning 2026").is_ok());
        assert!(validate_project_name("  trimmed  ").is_ok());
    }

    #[test]
    fn test_validate_project_name_empty() {
        assert!(validate_project_name("").is_err());
        assert!(validate_project_name("   ").is_err());
    }

    #[test]
    fn test_validate_project_name_too_long() {
        let long = "x".repeat(101);
        assert!(validate_project_name(&long).is_err());
        let exactly_max = "x".repeat(100);
        assert!(validate_project_name(&exactly_max).is_ok());
    }

    #[test]
    fn test_validate_task_id() {
        assert!(validate_task_id(1, 10).is_ok());
        assert!(validate_task_id(10, 10).is_ok());
        assert!(validate_task_id(0, 10).is_err());
        assert!(validate_task_id(11, 10).is_err());
    }

    #[test]
    fn test_validate_task_text() {
        assert!(validate_task_text("Valid task").is_ok());
        assert!(validate_task_text("  spaces  ").is_ok());
        assert!(validate_task_text("a").is_ok());

        assert!(validate_task_text("").is_err());
        assert!(validate_task_text("   ").is_err());
        assert!(validate_task_text("\t\n").is_err());

        let too_long = "x".repeat(501);
        assert!(validate_task_text(&too_long).is_err());

        let exactly_max = "x".repeat(500);
        assert!(validate_task_text(&exactly_max).is_ok());
    }

    #[test]
    fn test_validate_tags() {
        assert!(validate_tags(&["work".to_string()]).is_ok());
        assert!(validate_tags(&["work-urgent".to_string()]).is_ok());
        assert!(validate_tags(&["task_1".to_string()]).is_ok());
        assert!(
            validate_tags(&[
                "work".to_string(),
                "urgent".to_string(),
                "high-priority".to_string(),
            ])
            .is_ok()
        );

        assert!(validate_tags(&["".to_string()]).is_err());
        assert!(validate_tags(&["work".to_string(), "".to_string()]).is_err());

        assert!(validate_tags(&["work@home".to_string()]).is_err());
        assert!(validate_tags(&["tag with spaces".to_string()]).is_err());
        assert!(validate_tags(&["tag/slash".to_string()]).is_err());

        let long_tag = "x".repeat(51);
        assert!(validate_tags(&[long_tag]).is_err());

        assert!(validate_tags(&["work".to_string(), "work".to_string()]).is_err());
        assert!(validate_tags(&["work".to_string(), "Work".to_string()]).is_err());
        assert!(validate_tags(&["work".to_string(), "WORK".to_string()]).is_err());
    }

    #[test]
    fn test_validate_due_date() {
        use chrono::Local;

        let today = Local::now().naive_local().date();
        let yesterday = today - chrono::Duration::days(1);
        let tomorrow = today + chrono::Duration::days(1);

        assert!(validate_due_date(Some(tomorrow), false).is_ok());
        assert!(validate_due_date(Some(tomorrow), true).is_ok());
        assert!(validate_due_date(Some(today), false).is_ok());
        assert!(validate_due_date(Some(today), true).is_ok());
        assert!(validate_due_date(Some(yesterday), false).is_err());
        assert!(validate_due_date(Some(yesterday), true).is_ok());
        assert!(validate_due_date(None, false).is_ok());
        assert!(validate_due_date(None, true).is_ok());
    }

    #[test]
    fn test_validate_recurrence() {
        use chrono::Local;

        let today = Local::now().naive_local().date();
        let future = today + chrono::Duration::days(7);

        assert!(validate_recurrence(Some(Recurrence::Daily), None).is_err());
        assert!(validate_recurrence(Some(Recurrence::Weekly), None).is_err());
        assert!(validate_recurrence(Some(Recurrence::Monthly), None).is_err());

        assert!(validate_recurrence(Some(Recurrence::Daily), Some(future)).is_ok());
        assert!(validate_recurrence(Some(Recurrence::Weekly), Some(future)).is_ok());
        assert!(validate_recurrence(Some(Recurrence::Monthly), Some(future)).is_ok());

        assert!(validate_recurrence(None, None).is_ok());
        assert!(validate_recurrence(None, Some(future)).is_ok());
    }

    #[test]
    fn test_validate_task_new() {
        use chrono::Local;

        let today = Local::now().naive_local().date();
        let future = today + chrono::Duration::days(7);

        let task = Task::new(
            "Valid task".to_string(),
            Priority::Medium,
            vec!["work".to_string()],
            None,
            Some(future),
            None,
        );
        assert!(validate_task(&task, true).is_ok());

        let mut invalid = task.clone();
        invalid.text = "".to_string();
        assert!(validate_task(&invalid, true).is_err());

        let mut invalid = task.clone();
        invalid.tags = vec!["invalid tag with spaces".to_string()];
        assert!(validate_task(&invalid, true).is_err());

        let yesterday = today - chrono::Duration::days(1);
        let mut invalid = task.clone();
        invalid.due_date = Some(yesterday);
        assert!(validate_task(&invalid, true).is_err());

        let mut invalid = task.clone();
        invalid.recurrence = Some(Recurrence::Daily);
        invalid.due_date = None;
        assert!(validate_task(&invalid, true).is_err());
    }

    #[test]
    fn test_validate_task_existing() {
        use chrono::Local;

        let today = Local::now().naive_local().date();
        let yesterday = today - chrono::Duration::days(1);

        let task = Task::new(
            "Past task".to_string(),
            Priority::High,
            vec![],
            None,
            Some(yesterday),
            None,
        );
        assert!(validate_task(&task, false).is_ok());
    }
}
