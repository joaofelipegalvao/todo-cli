// src/commands/resource_remove.rs

//! Handler for `todo resource remove <ID>`.

use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;

pub fn execute(storage: &impl Storage, id: usize, yes: bool) -> Result<()> {
    let mut resources = storage.load_resources()?;

    let visible: Vec<usize> = resources
        .iter()
        .enumerate()
        .filter(|(_, r)| !r.is_deleted())
        .map(|(i, _)| i)
        .collect();

    let real_index = visible
        .get(id.saturating_sub(1))
        .copied()
        .ok_or_else(|| anyhow::anyhow!("Resource #{} not found", id))?;

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
    storage.save_resources(&resources)?;

    println!("{} Resource #{} removed.", "✓".green(), id);
    Ok(())
}
