//! Handler for `todo resource clear`.
//!
//! Soft-deletes all visible resources. Notes that referenced the deleted
//! resources have those entries removed from `resource_ids` automatically.

use anyhow::Result;
use colored::Colorize;
use uuid::Uuid;

use crate::storage::Storage;
use crate::utils::confirm;

pub fn execute(storage: &impl Storage, yes: bool) -> Result<()> {
    let (_, _, mut notes, mut resources) = storage.load_all_with_resources()?;

    let visible_count = resources.iter().filter(|r| !r.is_deleted()).count();

    if visible_count == 0 {
        println!("{} No resources to remove", "".blue());
        return Ok(());
    }

    if !yes {
        println!(
            "\n{} {} resources will be permanently deleted!",
            "".yellow().bold(),
            visible_count
        );

        if !confirm("Type 'yes' to confirm:")? {
            println!("{} Clear cancelled.", "".yellow());
            return Ok(());
        }
    }

    let deleted_uuids: Vec<Uuid> = resources
        .iter()
        .filter(|r| !r.is_deleted())
        .map(|r| r.uuid)
        .collect();

    for resource in resources.iter_mut().filter(|r| !r.is_deleted()) {
        resource.soft_delete();
    }

    // Remove deleted resource UUIDs from notes that reference them
    let mut notes_updated = 0;
    for note in notes.iter_mut().filter(|n| !n.is_deleted()) {
        let before = note.resource_ids.len();
        note.resource_ids.retain(|id| !deleted_uuids.contains(id));
        if note.resource_ids.len() != before {
            note.touch();
            notes_updated += 1;
        }
    }

    storage.save_notes(&notes)?;
    storage.save_resources(&resources)?;

    println!(
        "{} {} resources have been removed",
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
