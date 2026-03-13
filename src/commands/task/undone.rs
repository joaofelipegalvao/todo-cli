//! Handler for `todo undone <ID>`.
//!
//! Reverts a completed task back to pending status, clearing `completed_at`.

use anyhow::Result;
use colored::Colorize;

use crate::error::TodoError;
use crate::storage::Storage;
use crate::utils::validation::resolve_visible_index;

pub fn execute(storage: &impl Storage, id: usize) -> Result<()> {
    execute_inner(storage, id, false)?;
    Ok(())
}

/// TUI variant: same logic, no stdout, returns a status string.
pub fn execute_silent(storage: &impl Storage, id: usize) -> Result<String> {
    execute_inner(storage, id, true)
}

fn execute_inner(storage: &impl Storage, id: usize, silent: bool) -> Result<String> {
    let mut tasks = storage.load()?;

    let index = resolve_visible_index(&tasks, id, |t| t.is_deleted())
        .map_err(|_| anyhow::anyhow!("Task #{} not found", id))?;

    if !tasks[index].completed {
        return Err(TodoError::TaskAlreadyInStatus {
            id,
            status: "pending".to_owned(),
        }
        .into());
    }

    tasks[index].mark_undone();
    storage.save(&tasks)?;

    if !silent {
        let id_colored = format!("#{}", id).yellow();
        println!("Task {} marked as pending.", id_colored);
    }

    Ok(format!("Task #{} marked as pending.", id))
}
