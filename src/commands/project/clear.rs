//! Handler for `todo project clear`.
//!
//! Soft-deletes all visible projects. Tasks and notes that were linked
//! to deleted projects have their `project_id` cleared automatically.

use anyhow::Result;
use colored::Colorize;
use uuid::Uuid;

use crate::storage::Storage;
use crate::utils::confirm;

pub fn execute(storage: &impl Storage, yes: bool) -> Result<()> {
    let (mut tasks, mut projects, mut notes) = storage.load_all()?;

    let visible_count = projects.iter().filter(|p| !p.is_deleted()).count();

    if visible_count == 0 {
        println!("{} No projects to remove", "".blue());
        return Ok(());
    }

    if !yes {
        println!(
            "\n{} {} projects will be permanently deleted!",
            "".yellow().bold(),
            visible_count
        );
        if !confirm("Type 'yes' to confirm:")? {
            println!("{} Clear cancelled.", "".yellow());
            return Ok(());
        }
    }

    let deleted_uuids: Vec<Uuid> = projects
        .iter()
        .filter(|p| !p.is_deleted())
        .map(|p| p.uuid)
        .collect();

    for project in projects.iter_mut().filter(|p| !p.is_deleted()) {
        project.soft_delete();
    }

    // Clear project_id from tasks and notes linked to deleted projects
    let mut tasks_updated = 0;
    for task in tasks.iter_mut().filter(|t| !t.is_deleted()) {
        if let Some(pid) = task.project_id
            && deleted_uuids.contains(&pid)
        {
            task.project_id = None;
            task.touch();
            tasks_updated += 1;
        }
    }

    let mut notes_updated = 0;
    for note in notes.iter_mut().filter(|n| !n.is_deleted()) {
        if let Some(pid) = note.project_id
            && deleted_uuids.contains(&pid)
        {
            note.project_id = None;
            note.touch();
            notes_updated += 1;
        }
    }

    storage.save_all(&tasks, &projects, &notes)?;

    println!(
        "{} {} projects have been removed",
        "✓".green().bold(),
        visible_count
    );
    if tasks_updated > 0 {
        println!(
            "  {} {} task{} unlinked",
            "·".dimmed(),
            tasks_updated,
            if tasks_updated == 1 { "" } else { "s" }
        );
    }
    if notes_updated > 0 {
        println!(
            "  {} {} note{} unlinked",
            "·".dimmed(),
            notes_updated,
            if notes_updated == 1 { "" } else { "s" }
        );
    }

    Ok(())
}
