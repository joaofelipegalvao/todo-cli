//! Handler for `todo note clear`.
//!
//! Prompts the user for confirmation (unless `--yes` is passed) then
//! soft-deletes all visible (non-deleted) notes. Tasks, projects, and
//! resources are left untouched.

use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;
use crate::utils::confirm;

pub fn execute(storage: &impl Storage, yes: bool) -> Result<()> {
    let (_, _, mut notes) = storage.load_all()?;

    let visible_count = notes.iter().filter(|n| !n.is_deleted()).count();

    if visible_count == 0 {
        println!("{} No notes to remove", "".blue());
        return Ok(());
    }

    if !yes {
        println!(
            "\n{} {} notes will be permanently deleted!",
            "".yellow().bold(),
            visible_count
        );

        if !confirm("Type 'yes' to confirm:")? {
            println!("{} Clear cancelled.", "".yellow());
            return Ok(());
        }
    }

    for note in notes.iter_mut().filter(|n| !n.is_deleted()) {
        note.soft_delete();
    }

    storage.save_notes(&notes)?;

    println!(
        "{} {} notes have been removed",
        "✓".green().bold(),
        visible_count
    );
    Ok(())
}
