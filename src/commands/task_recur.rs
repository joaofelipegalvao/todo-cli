//! Handler for `todo recur <ID> <PATTERN>`.
//!
//! Sets or updates the recurrence pattern on a task. Requires the task to
//! already have a due date — without one there is no base date from which to
//! calculate the next occurrence.

use anyhow::Result;
use colored::Colorize;

use crate::models::Recurrence;
use crate::storage::Storage;
use crate::validation::validate_task_id;

pub fn execute(storage: &impl Storage, id: usize, pattern: Recurrence) -> Result<()> {
    let mut tasks = storage.load()?;
    validate_task_id(id, tasks.len())?;

    let index = id - 1;
    let task = &mut tasks[index];

    if task.due_date.is_none() {
        return Err(anyhow::anyhow!(
            "Task #{} has no due date. Add one with: todo edit {} --due YYYY-MM-DD",
            id,
            id
        ));
    }

    let old_recurrence = task.recurrence;
    task.recurrence = Some(pattern);

    if old_recurrence != Some(pattern) {
        task.touch();
    }

    storage.save(&tasks)?;

    match old_recurrence {
        Some(old) if old == pattern => {
            println!(
                "{} Recurrence already set to {} for task #{}",
                "".yellow(),
                pattern,
                id,
            );
        }
        Some(old) => {
            println!(
                "{} Updated recurrence for task #{}: {} → {}",
                "✓".green(),
                id,
                old,
                pattern
            );
        }
        None => {
            println!(
                "{} Set {} recurrence for task #{}",
                "✓".green(),
                id,
                pattern,
            );
        }
    }

    Ok(())
}
