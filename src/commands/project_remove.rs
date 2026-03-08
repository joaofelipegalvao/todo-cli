//! Handler for `todo project remove <ID>`.
//!
//! Soft-deletes a project after optional confirmation. Tasks and notes linked
//! to the removed project have their `project_id` cleared automatically.

use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;
use crate::validation::{validate_task_id, visible_indices};

pub fn execute(storage: &impl Storage, id: usize, yes: bool) -> Result<()> {
    execute_inner(storage, id, yes, false)?;
    Ok(())
}

pub fn execute_silent(storage: &impl Storage, id: usize) -> Result<String> {
    execute_inner(storage, id, true, true)
}

fn execute_inner(storage: &impl Storage, id: usize, yes: bool, silent: bool) -> Result<String> {
    let (mut tasks, mut projects, mut notes) = storage.load_all()?;

    let vis = visible_indices(&projects, |p| p.is_deleted());
    validate_task_id(id, vis.len())?;
    let real_index = vis[id - 1];
    let project_uuid = projects[real_index].uuid;
    let name = projects[real_index].name.clone();

    if !yes && !silent {
        println!(
            "{} Remove project #{}: {}? [y/N] ",
            "!".yellow(),
            id,
            name.bold()
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("{} Cancelled.", "".dimmed());
            return Ok("Cancelled.".to_string());
        }
    }

    projects[real_index].soft_delete();

    // Clear project_id from tasks and notes linked to this project
    for task in tasks.iter_mut().filter(|t| !t.is_deleted()) {
        if task.project_id == Some(project_uuid) {
            task.project_id = None;
            task.touch();
        }
    }
    for note in notes.iter_mut().filter(|n| !n.is_deleted()) {
        if note.project_id == Some(project_uuid) {
            note.project_id = None;
            note.touch();
        }
    }

    storage.save_all(&tasks, &projects, &notes)?;

    let msg = format!("Project #{} ({}) removed.", id, name);
    if !silent {
        println!("{} {}", "✓".green(), msg.as_str().cyan());
    }
    Ok(msg)
}
