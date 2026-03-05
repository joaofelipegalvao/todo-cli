//! Handler for `todo project remove <ID>`.

use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;

pub fn execute(storage: &impl Storage, id: usize, yes: bool) -> Result<()> {
    let (_, mut projects, _) = storage.load_all()?;

    let visible: Vec<usize> = projects
        .iter()
        .enumerate()
        .filter(|(_, p)| !p.is_deleted())
        .map(|(i, _)| i)
        .collect();

    let real_index = visible
        .get(id.saturating_sub(1))
        .copied()
        .ok_or_else(|| anyhow::anyhow!("Project #{} not found", id))?;

    let name = projects[real_index].name.clone();

    if !yes {
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
            return Ok(());
        }
    }

    projects[real_index].soft_delete();
    storage.save_projects(&projects)?;

    println!("{} Project #{} ({}) removed.", "✓".green(), id, name.cyan());
    Ok(())
}
