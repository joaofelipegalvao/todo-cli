//! Handler for `todo purge`.
//!
//! Permanently removes soft-deleted tombstones older than N days.
//! Tombstones must be kept long enough for sync to propagate deletions
//! across all devices — purging too early causes deleted tasks to reappear.

use anyhow::Result;
use chrono::Utc;
use colored::Colorize;

use crate::storage::Storage;
use crate::utils::confirm;

pub fn execute(storage: &impl Storage, days: u32, dry_run: bool, yes: bool) -> Result<()> {
    let mut tasks = storage.load()?;
    let cutoff = Utc::now() - chrono::Duration::days(days as i64);

    let to_purge: Vec<(usize, String)> = tasks
        .iter()
        .enumerate()
        .filter_map(|(i, t)| {
            t.deleted_at.and_then(|deleted_at| {
                if deleted_at <= cutoff {
                    Some((i, t.text.clone()))
                } else {
                    None
                }
            })
        })
        .collect();

    if to_purge.is_empty() {
        println!(
            "{}",
            format!(
                "\nNo tombstones older than {} day{} found.\n",
                days,
                if days == 1 { "" } else { "s" }
            )
            .dimmed()
        );
        return Ok(());
    }

    println!(
        "\n{} tombstone{} older than {} day{} would be permanently removed:\n",
        to_purge.len().to_string().yellow(),
        if to_purge.len() == 1 { "" } else { "s" },
        days,
        if days == 1 { "" } else { "s" },
    );
    for (_, text) in &to_purge {
        println!("  {} {}", "✗".dimmed(), text.dimmed());
    }
    println!();

    if dry_run {
        println!("{}", "Dry run — nothing was removed.".dimmed());
        return Ok(());
    }

    if !yes && !confirm("Permanently delete these tombstones? [y/N]:")? {
        println!("{}", "Purge cancelled.".dimmed());
        return Ok(());
    }

    let mut sorted_indices: Vec<usize> = to_purge.iter().map(|(i, _)| *i).collect();
    sorted_indices.sort_unstable_by(|a, b| b.cmp(a));
    for i in sorted_indices {
        tasks.remove(i);
    }

    storage.save(&tasks)?;

    println!(
        "{} Permanently removed {} tombstone{}.",
        "✓".green(),
        to_purge.len().to_string().green(),
        if to_purge.len() == 1 { "" } else { "s" },
    );

    Ok(())
}
