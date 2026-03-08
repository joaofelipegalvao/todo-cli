//! Handler for `todo norecur <ID>`.
//!
//! Removes the recurrence pattern from a single task without deleting it.
//! The task remains and can still be completed manually.

use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;
use crate::validation::validate_task_id;

pub fn execute(storage: &impl Storage, id: usize) -> Result<()> {
    let mut tasks = storage.load()?;
    validate_task_id(id, tasks.len())?;

    let index = id - 1;
    let task = &mut tasks[index];

    let Some(old_pattern) = task.recurrence.take() else {
        println!("{} Task #{} has no recurrence", "".yellow(), id);
        return Ok(());
    };

    task.touch();

    storage.save(&tasks)?;

    println!(
        "{} Removed {} recurrence from task #{}",
        "✓".green(),
        old_pattern,
        id,
    );

    Ok(())
}
