//! Handler for `todo purge`.
//!
//! Permanently removes soft-deleted tombstones older than N days across all
//! entity types: tasks, projects, notes, and resources.
//!
//! Tombstones must be kept long enough for sync to propagate deletions
//! across all devices — purging too early causes deleted tasks to reappear.

use anyhow::Result;
use chrono::Utc;
use colored::Colorize;

use crate::storage::Storage;
use crate::utils::confirm;

/// A purgeable tombstone with enough info to display and remove it.
struct Tombstone {
    index: usize,
    label: String,
}

/// Collects tombstone indices and labels from a slice of entities that expose
/// `deleted_at: Option<DateTime<Utc>>` and a display label.
macro_rules! collect_tombstones {
    ($items:expr, $cutoff:expr, $label_fn:expr) => {
        $items
            .iter()
            .enumerate()
            .filter_map(|(i, item)| {
                item.deleted_at.and_then(|deleted_at| {
                    if deleted_at <= $cutoff {
                        Some(Tombstone {
                            index: i,
                            label: $label_fn(item),
                        })
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>()
    };
}

/// Removes items at the given tombstone indices (descending to preserve indices).
fn purge_indices<T>(items: &mut Vec<T>, tombstones: &[Tombstone]) {
    let mut indices: Vec<usize> = tombstones.iter().map(|t| t.index).collect();
    indices.sort_unstable_by(|a, b| b.cmp(a));
    for i in indices {
        items.remove(i);
    }
}

pub fn execute(storage: &impl Storage, days: u32, dry_run: bool, yes: bool) -> Result<()> {
    let (mut tasks, mut projects, mut notes, mut resources) = storage.load_all_with_resources()?;

    let cutoff = Utc::now() - chrono::Duration::days(days as i64);

    let task_tombs = collect_tombstones!(&tasks, cutoff, |t: &crate::models::Task| t.text.clone());
    let project_tombs = collect_tombstones!(&projects, cutoff, |p: &crate::models::Project| p
        .name
        .clone());
    let note_tombs = collect_tombstones!(&notes, cutoff, |n: &crate::models::Note| n
        .title
        .clone()
        .unwrap_or_else(|| {
            let b = n.body.as_str();
            if b.len() > 60 {
                b[..60].to_string()
            } else {
                b.to_string()
            }
        }));
    let resource_tombs = collect_tombstones!(&resources, cutoff, |r: &crate::models::Resource| r
        .title
        .clone());

    let total = task_tombs.len() + project_tombs.len() + note_tombs.len() + resource_tombs.len();

    if total == 0 {
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

    // ── preview ───────────────────────────────────────────────────────────────

    println!(
        "\n{} tombstone{} older than {} day{} would be permanently removed:\n",
        total.to_string().yellow(),
        if total == 1 { "" } else { "s" },
        days,
        if days == 1 { "" } else { "s" },
    );

    let print_section = |label: &str, tombs: &[Tombstone]| {
        if !tombs.is_empty() {
            println!("  {}:", label.dimmed());
            for t in tombs {
                println!("    {} {}", "✗".dimmed(), t.label.dimmed());
            }
        }
    };

    print_section("tasks", &task_tombs);
    print_section("projects", &project_tombs);
    print_section("notes", &note_tombs);
    print_section("resources", &resource_tombs);
    println!();

    if dry_run {
        println!("{}", "Dry run — nothing was removed.".dimmed());
        return Ok(());
    }

    if !yes && !confirm("Permanently delete these tombstones? [y/N]:")? {
        println!("{}", "Purge cancelled.".dimmed());
        return Ok(());
    }

    // ── purge ─────────────────────────────────────────────────────────────────

    purge_indices(&mut tasks, &task_tombs);
    purge_indices(&mut projects, &project_tombs);
    purge_indices(&mut notes, &note_tombs);
    purge_indices(&mut resources, &resource_tombs);

    storage.save_all(&tasks, &projects, &notes)?;
    storage.save_resources(&resources)?;

    println!(
        "{} Permanently removed {} tombstone{}.",
        "✓".green(),
        total.to_string().green(),
        if total == 1 { "" } else { "s" },
    );

    Ok(())
}
