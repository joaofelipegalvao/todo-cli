//! Handler for `todo remove <ID>`.
//!
//! Soft-deletes a task after optional confirmation. Notes linked to the
//! removed task have their `task_id` cleared automatically.

use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;
use crate::utils::confirm;
use crate::utils::validation::{validate_task_id, visible_indices};

pub fn execute(storage: &impl Storage, id: usize, yes: bool) -> Result<()> {
    execute_inner(storage, id, yes, false)?;
    Ok(())
}

pub fn execute_silent(storage: &impl Storage, id: usize) -> Result<String> {
    execute_inner(storage, id, true, true)
}

fn execute_inner(storage: &impl Storage, id: usize, yes: bool, silent: bool) -> Result<String> {
    let (mut tasks, projects, mut notes) = storage.load_all()?;

    let vis = visible_indices(&tasks, |t| t.is_deleted());
    validate_task_id(id, vis.len())?;
    let real_index = vis[id - 1];
    let task_uuid = tasks[real_index].uuid;
    let task_text = tasks[real_index].text.clone();

    if !yes && !silent {
        println!("\n{} {}", "".yellow(), task_text.bright_white());
        if !confirm("Are you sure? [y/N]:")? {
            println!("{} Removal cancelled.", "".yellow());
            return Ok("Cancelled.".to_string());
        }
    }

    tasks[real_index].soft_delete();

    // Clear task_id from notes linked to this task
    for note in notes.iter_mut().filter(|n| !n.is_deleted()) {
        if note.task_id == Some(task_uuid) {
            note.task_id = None;
            note.touch();
        }
    }

    storage.save_all(&tasks, &projects, &notes)?;

    let msg = format!("Task removed: {}", task_text);
    if !silent {
        println!("{} {}", "✓".green(), msg.as_str().dimmed());
    }
    Ok(msg)
}
