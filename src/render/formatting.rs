use chrono::Local;
use colored::{ColoredString, Colorize};

use crate::models::Task;

/// Generates a human-readable due date description.
///
/// # Returns
///
/// A relative string based on today's date:
/// - `"late N days"` if the due date is in the past
/// - `"due today"` if due today
/// - `"in N days"` if due in the future
///
/// Returns an empty string if the task has no due date.
///
/// # Examples
///
/// ```no_run
/// use rustodo::models::{Task, Priority};
///
/// // Note: the exact output depends on today's date at runtime.
/// let task = Task::new("Buy milk".to_string(), Priority::Medium, vec![], None, None, None);
/// // returns "" when task has no due date
/// ```
/// Converts a `NaiveDate` into a human-readable relative string.
/// Shared by task list, project list, and any other due-date display.
pub fn due_relative_text(due: chrono::NaiveDate) -> String {
    let today = Local::now().naive_local().date();
    let days = (due - today).num_days();
    match days {
        d if d < 0 => {
            let abs_d = d.abs();
            format!("late {} day{}", abs_d, if abs_d == 1 { "" } else { "s" })
        }
        0 => "today".to_string(),
        1 => "tomorrow".to_string(),
        d => format!("in {} day{}", d, if d == 1 { "" } else { "s" }),
    }
}

pub fn get_due_text(task: &Task) -> String {
    task.due_date.map(due_relative_text).unwrap_or_default()
}

/// Returns a colored version of the due date text based on urgency.
///
/// Color coding:
/// - Red (bold): Overdue
/// - Yellow (bold): Due today
/// - Yellow: Due within 7 days
/// - Cyan: Due later
/// - Dimmed: Completed tasks
pub fn get_due_colored(task: &Task, text: &str) -> ColoredString {
    if text.is_empty() {
        return "".normal();
    }

    if task.completed {
        return text.dimmed();
    }

    if let Some(due) = task.due_date {
        let today = Local::now().naive_local().date();
        let days_until = (due - today).num_days();

        if days_until < 0 {
            text.red().bold()
        } else if days_until == 0 {
            text.yellow().bold()
        } else if days_until <= 7 {
            text.yellow()
        } else {
            text.cyan()
        }
    } else {
        text.normal()
    }
}

/// Truncates text to `max` characters, appending `...` if cut.
///
/// Uses `.chars()` to avoid splitting multi-byte UTF-8 sequences.
pub fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        text.to_string()
    } else {
        let truncated: String = text.chars().take(max.saturating_sub(3)).collect();
        format!("{}...", truncated)
    }
}
