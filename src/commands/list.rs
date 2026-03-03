//! Handler for `todo list`.
//!
//! Loads all tasks, applies filters (status, priority, due, tag, project,
//! recurrence) in sequence, optionally sorts the result, and delegates
//! rendering to [`display::display_lists`](crate::display::display_lists).

use anyhow::Result;

use crate::display::display_lists;
use crate::error::TodoError;
use crate::models::{DueFilter, Priority, Recurrence, RecurrenceFilter, SortBy, StatusFilter};
use crate::storage::Storage;

#[allow(clippy::too_many_arguments)]
pub fn execute(
    storage: &impl Storage,
    status: StatusFilter,
    priority: Option<Priority>,
    due: Option<DueFilter>,
    sort: Option<SortBy>,
    tag: Option<String>,
    project: Option<String>,
    recur: Option<RecurrenceFilter>,
) -> Result<()> {
    let all_tasks = storage.load()?;

    let mut indexed_tasks: Vec<(usize, &_)> = all_tasks
        .iter()
        .filter(|t| !t.is_deleted())
        .enumerate()
        .map(|(i, task)| (i + 1, task))
        .collect();

    indexed_tasks.retain(|(_, t)| t.matches_status(status));

    if let Some(pri) = priority {
        indexed_tasks.retain(|(_, t)| t.priority == pri);
    }

    if let Some(due_filter) = due {
        indexed_tasks.retain(|(_, t)| t.matches_due_filter(due_filter));
    }

    if let Some(tag_name) = &tag {
        let count_before = indexed_tasks.len();
        indexed_tasks.retain(|(_, t)| t.tags.contains(tag_name));
        if indexed_tasks.is_empty() && count_before > 0 {
            return Err(TodoError::TagNotFound(tag_name.to_owned()).into());
        }
    }

    if let Some(project_name) = &project {
        let count_before = indexed_tasks.len();
        indexed_tasks.retain(|(_, t)| {
            t.project
                .as_deref()
                .map(|p| p.to_lowercase() == project_name.to_lowercase())
                .unwrap_or(false)
        });
        if indexed_tasks.is_empty() && count_before > 0 {
            return Err(TodoError::ProjectNotFound(project_name.to_owned()).into());
        }
    }

    if let Some(recur_filter) = recur {
        indexed_tasks.retain(|(_, t)| match recur_filter {
            RecurrenceFilter::Daily => t.recurrence == Some(Recurrence::Daily),
            RecurrenceFilter::Weekly => t.recurrence == Some(Recurrence::Weekly),
            RecurrenceFilter::Monthly => t.recurrence == Some(Recurrence::Monthly),
            RecurrenceFilter::Recurring => t.recurrence.is_some(),
            RecurrenceFilter::NonRecurring => t.recurrence.is_none(),
        });
    }

    if indexed_tasks.is_empty() {
        return Err(TodoError::NoTasksFound.into());
    }

    if let Some(sort_by) = sort {
        match sort_by {
            SortBy::Priority => {
                indexed_tasks.sort_by(|(_, a), (_, b)| a.priority.order().cmp(&b.priority.order()));
            }
            SortBy::Due => {
                indexed_tasks.sort_by(|(_, a), (_, b)| match (a.due_date, b.due_date) {
                    (Some(da), Some(db)) => da.cmp(&db),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                });
            }
            SortBy::Created => {
                indexed_tasks.sort_by(|(_, a), (_, b)| a.created_at.cmp(&b.created_at));
            }
        }
    }

    let title = determine_title(status, priority, due, project.as_deref(), recur);
    let visible: Vec<_> = all_tasks
        .iter()
        .filter(|t| !t.is_deleted())
        .cloned()
        .collect();
    display_lists(&indexed_tasks, &title, &visible);
    Ok(())
}

fn determine_title(
    status: StatusFilter,
    priority: Option<Priority>,
    due: Option<DueFilter>,
    project: Option<&str>,
    recur: Option<RecurrenceFilter>,
) -> String {
    if let Some(p) = project {
        return format!("Tasks in project \"{}\"", p);
    }

    if let Some(recur_filter) = recur {
        return match (status, recur_filter) {
            (StatusFilter::Pending, RecurrenceFilter::Daily) => "Pending daily recurring tasks",
            (StatusFilter::Pending, RecurrenceFilter::Weekly) => "Pending weekly recurring tasks",
            (StatusFilter::Pending, RecurrenceFilter::Monthly) => "Pending monthly recurring tasks",
            (StatusFilter::Pending, RecurrenceFilter::Recurring) => "Pending recurring tasks",
            (StatusFilter::Pending, RecurrenceFilter::NonRecurring) => {
                "Pending non-recurring tasks"
            }
            (StatusFilter::Done, RecurrenceFilter::Daily) => "Completed daily recurring tasks",
            (StatusFilter::Done, RecurrenceFilter::Weekly) => "Completed weekly recurring tasks",
            (StatusFilter::Done, RecurrenceFilter::Monthly) => "Completed monthly recurring tasks",
            (StatusFilter::Done, RecurrenceFilter::Recurring) => "Completed recurring tasks",
            (StatusFilter::Done, RecurrenceFilter::NonRecurring) => "Completed non-recurring tasks",
            (StatusFilter::All, RecurrenceFilter::Daily) => "Daily recurring tasks",
            (StatusFilter::All, RecurrenceFilter::Weekly) => "Weekly recurring tasks",
            (StatusFilter::All, RecurrenceFilter::Monthly) => "Monthly recurring tasks",
            (StatusFilter::All, RecurrenceFilter::Recurring) => "Recurring tasks",
            (StatusFilter::All, RecurrenceFilter::NonRecurring) => "Non-recurring tasks",
        }
        .to_string();
    }

    match (status, priority, due) {
        (StatusFilter::Pending, Some(Priority::High), _) => "High priority pending tasks",
        (StatusFilter::Pending, Some(Priority::Medium), _) => "Medium priority pending tasks",
        (StatusFilter::Pending, Some(Priority::Low), _) => "Low priority pending tasks",
        (StatusFilter::Pending, None, Some(DueFilter::Overdue)) => "Pending overdue tasks",
        (StatusFilter::Pending, None, Some(DueFilter::Soon)) => "Pending tasks due soon",
        (StatusFilter::Pending, None, _) => "Pending tasks",
        (StatusFilter::Done, _, _) => "Completed tasks",
        (StatusFilter::All, Some(Priority::High), _) => "High priority tasks",
        (StatusFilter::All, Some(Priority::Medium), _) => "Medium priority tasks",
        (StatusFilter::All, Some(Priority::Low), _) => "Low priority tasks",
        (StatusFilter::All, None, Some(DueFilter::Overdue)) => "Overdue tasks",
        (StatusFilter::All, None, Some(DueFilter::Soon)) => "Tasks due soon",
        (StatusFilter::All, None, Some(DueFilter::WithDue)) => "Tasks with due date",
        (StatusFilter::All, None, Some(DueFilter::NoDue)) => "Tasks without due date",
        _ => "Tasks",
    }
    .to_string()
}
