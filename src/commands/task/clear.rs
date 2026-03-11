//! Handler for `todo clear`.
//!
//! Soft-deletes all visible tasks. Notes linked to deleted tasks have their
//! `task_id` cleared automatically.

use anyhow::Result;
use colored::Colorize;
use uuid::Uuid;

use crate::storage::Storage;
use crate::utils::confirm;

pub fn execute(storage: &impl Storage, yes: bool) -> Result<()> {
    let (mut tasks, projects, mut notes) = storage.load_all()?;

    let visible_count = tasks.iter().filter(|t| !t.is_deleted()).count();

    if visible_count == 0 {
        println!("{} No tasks to remove", "".blue());
        return Ok(());
    }

    if !yes {
        println!(
            "\n{} {} tasks will be permanently deleted!",
            "".yellow().bold(),
            visible_count
        );
        if !confirm("Type 'yes' to confirm:")? {
            println!("{} Clear cancelled.", "".yellow());
            return Ok(());
        }
    }

    let deleted_uuids: Vec<Uuid> = tasks
        .iter()
        .filter(|t| !t.is_deleted())
        .map(|t| t.uuid)
        .collect();

    for task in tasks.iter_mut().filter(|t| !t.is_deleted()) {
        task.soft_delete();
    }

    // Clear task_id from notes linked to deleted tasks
    let mut notes_updated = 0;
    for note in notes.iter_mut().filter(|n| !n.is_deleted()) {
        if let Some(tid) = note.task_id
            && deleted_uuids.contains(&tid)
        {
            note.task_id = None;
            note.touch();
            notes_updated += 1;
        }
    }

    storage.save_all(&tasks, &projects, &notes)?;

    println!(
        "{} {} tasks have been removed",
        "✓".green().bold(),
        visible_count
    );
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
