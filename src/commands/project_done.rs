//! Handler for `todo project done <ID>`.

use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;
use crate::validation::{validate_task_id, visible_indices};

pub fn execute(storage: &impl Storage, id: usize) -> Result<()> {
    execute_inner(storage, id, false)?;
    Ok(())
}

/// TUI variant: same logic, no stdout, returns a status string.
pub fn execute_silent(storage: &impl Storage, id: usize) -> Result<String> {
    execute_inner(storage, id, true)
}

fn execute_inner(storage: &impl Storage, id: usize, silent: bool) -> Result<String> {
    let (_, mut projects, _) = storage.load_all()?;

    let vis = visible_indices(&projects, |p| p.is_deleted());
    validate_task_id(id, vis.len())?;
    let project = &mut projects[vis[id - 1]];

    if project.completed {
        let msg = format!("Project {} is already done.", format!("#{}", id).green());
        if !silent {
            println!("{}", msg);
        }
        return Ok(msg);
    }

    project.mark_done();
    storage.save_projects(&projects)?;

    let msg = format!("Project {} marked as done.", format!("#{}", id).green());
    if !silent {
        println!("{}", msg);
    }

    Ok(msg)
}
