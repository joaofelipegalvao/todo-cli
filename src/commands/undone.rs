//! Handler for `todo undone <ID>`.
//!
//! Reverts a completed task back to pending status, clearing `completed_at`.

use anyhow::Result;
use colored::Colorize;

use crate::error::TodoError;
use crate::storage::Storage;
use crate::validation::{validate_task_id, visible_indices};

pub fn execute(storage: &impl Storage, id: usize) -> Result<()> {
    let mut tasks = storage.load()?;
    let vis = visible_indices(&tasks);
    validate_task_id(id, vis.len())?;
    let index = vis[id - 1];

    if !tasks[index].completed {
        return Err(TodoError::TaskAlreadyInStatus {
            id,
            status: "pending".to_owned(),
        }
        .into());
    }

    tasks[index].mark_undone();
    storage.save(&tasks)?;
    println!("{}", "✓ Task unmarked".yellow());
    Ok(())
}
