//! Handler for `todo resource remove <ID>`.
//!
//! Soft-deletes a resource after optional confirmation. Notes that referenced
//! the removed resource have it removed from `resource_ids` automatically.

use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;
use crate::utils::validation::resolve_visible_index;

pub fn execute(storage: &impl Storage, id: usize, yes: bool) -> Result<()> {
    let (_, _, mut notes, mut resources) = storage.load_all_with_resources()?;

    let real_index = resolve_visible_index(&resources, id, |r| r.is_deleted())
        .map_err(|_| anyhow::anyhow!("Resource #{} not found", id))?;

    let resource_uuid = resources[real_index].uuid;
    let title = resources[real_index].title.clone();

    if !yes {
        println!(
            "{} Remove resource #{}: {}? [y/N] ",
            "!".yellow(),
            id,
            title.bold()
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("{} Cancelled.", "".dimmed());
            return Ok(());
        }
    }

    resources[real_index].soft_delete();

    // Remove this resource UUID from notes that reference it
    for note in notes.iter_mut().filter(|n| !n.is_deleted()) {
        let before = note.resource_ids.len();
        note.resource_ids.retain(|id| *id != resource_uuid);
        if note.resource_ids.len() != before {
            note.touch();
        }
    }

    storage.save_notes(&notes)?;
    storage.save_resources(&resources)?;

    println!("{} Resource #{} removed.", "✓".green(), id);
    Ok(())
}
