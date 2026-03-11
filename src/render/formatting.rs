use chrono::Local;
use colored::{ColoredString, Colorize};
use uuid::Uuid;

use crate::models::{Project, Task};

/// Resolves a `project_id` to its display name.
///
/// Returns `"—"` if the ID is `None` or the project is soft-deleted.
pub fn project_name(project_id: Option<Uuid>, projects: &[Project]) -> &str {
    project_id
        .and_then(|pid| projects.iter().find(|p| p.uuid == pid && !p.is_deleted()))
        .map(|p| p.name.as_str())
        .unwrap_or("—")
}

/// Colorizes a resolved project name.
///
/// - `magenta` for a real project name
/// - `dimmed` for `"—"` (no project)
pub fn project_colored(name: &str) -> ColoredString {
    if name == "—" {
        name.dimmed()
    } else {
        name.magenta()
    }
}

/// Converts a `NaiveDate` into a full date string (YYYY-MM-DD).
/// Shared by task list, project list, and any other due-date display.
pub fn due_relative_text(due: chrono::NaiveDate) -> String {
    due.format("%Y-%m-%d").to_string()
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
