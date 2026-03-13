//! Handler for `todo project undone <ID>`.

use anyhow::Result;
use colored::Colorize;

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
    let (_, mut projects, _) = storage.load_all()?;

    let real_index = resolve_visible_index(&projects, id, |p| p.is_deleted())
        .map_err(|_| anyhow::anyhow!("Project #{} not found", id))?;

    let project = &mut projects[real_index];

    if !project.completed {
        let msg = format!(
            "Project {} is already pending.",
            format!("#{}", id).yellow()
        );
        if !silent {
            println!("{}", msg);
        }
        return Ok(msg);
    }

    project.mark_undone();
    storage.save_projects(&projects)?;

    let msg = format!("Project {} marked as pending.", format!("#{}", id).yellow());
    if !silent {
        println!("{}", msg);
    }

    Ok(msg)
}
