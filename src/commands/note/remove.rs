//! Handler for `todo note remove <ID>`.

use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;
use crate::utils::validation::resolve_visible_index;

pub fn execute(storage: &impl Storage, id: usize, yes: bool) -> Result<()> {
    let (_, _, mut notes) = storage.load_all()?;

    let real_index = resolve_visible_index(&notes, id, |n| n.is_deleted())
        .map_err(|_| anyhow::anyhow!("Note #{} not found", id))?;

    let preview = notes[real_index].title.clone().unwrap_or_else(|| {
        let b = notes[real_index].body.as_str();
        if b.len() > 60 {
            b[..60].to_string()
        } else {
            b.to_string()
        }
    });

    if !yes {
        println!(
            "{} Remove note #{}: {}? [y/N] ",
            "!".yellow(),
            id,
            preview.bold()
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("{} Cancelled.", "".dimmed());
            return Ok(());
        }
    }

    notes[real_index].soft_delete();
    storage.save_notes(&notes)?;

    println!("{} Note #{} removed.", "✓".green(), id);
    Ok(())
}
