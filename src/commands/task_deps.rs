//! Handler for `todo deps <ID>`.
//!
//! Prints a dependency graph for a single task showing:
//! - Tasks it depends on, with their completion status
//! - Tasks that depend on it (reverse edges)
//! - Whether the task is currently blocked, and by which IDs

use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;
use crate::validation::{validate_task_id, visible_indices};

pub fn execute(storage: &impl Storage, id: usize) -> Result<()> {
    let tasks = storage.load()?;
    let vis = visible_indices(&tasks, |t| t.is_deleted());
    validate_task_id(id, vis.len())?;

    let real_index = vis[id - 1];
    let task = &tasks[real_index];

    println!(
        "\n{} #{}: {}\n",
        "Task".dimmed(),
        id,
        task.text.bright_white()
    );

    // Helper: visible ID for a real array index
    let vis_id =
        |real: usize| -> Option<usize> { vis.iter().position(|&i| i == real).map(|p| p + 1) };

    // === This task depends on ===
    if task.depends_on.is_empty() {
        println!("{}", "  No dependencies.".dimmed());
    } else {
        println!("{}:", "  Depends on".dimmed());
        for dep_uuid in &task.depends_on {
            if let Some(real_pos) = tasks.iter().position(|t| t.uuid == *dep_uuid) {
                let dep = &tasks[real_pos];
                let dep_vis_id = vis_id(real_pos);
                let status = if dep.completed {
                    "✓".green()
                } else {
                    "◦".red()
                };
                let label = if dep.completed {
                    dep.text.dimmed()
                } else {
                    dep.text.bright_white()
                };
                if let Some(did) = dep_vis_id {
                    println!("    {} #{} — {}", status, did, label);
                } else {
                    println!("    {} [deleted] — {}", status, label);
                }
            } else {
                println!("    {} — {}", "?".yellow(), "(task not found)".dimmed());
            }
        }
    }

    // === Tasks that depend on this one ===
    let task_uuid = task.uuid;
    let dependents: Vec<(usize, &_)> = tasks
        .iter()
        .enumerate()
        .filter(|(i, t)| *i != real_index && t.depends_on.contains(&task_uuid) && !t.is_deleted())
        .filter_map(|(real_pos, t)| vis_id(real_pos).map(|vid| (vid, t)))
        .collect();

    println!();
    if dependents.is_empty() {
        println!("{}", "  No tasks depend on this one.".dimmed());
    } else {
        println!("{}:", "  Required by".dimmed());
        for (dep_vis_id, dep_task) in &dependents {
            let status = if dep_task.completed {
                "✓".green()
            } else {
                "◦".yellow()
            };
            println!(
                "    {} #{} — {}",
                status,
                dep_vis_id,
                dep_task.text.bright_white()
            );
        }
    }

    // === Blocked status ===
    println!();
    let visible_tasks: Vec<_> = tasks.iter().filter(|t| !t.is_deleted()).cloned().collect();
    if task.is_blocked(&visible_tasks) {
        let blocking = task.blocking_deps(&visible_tasks);
        let ids = blocking
            .iter()
            .filter_map(|uuid| {
                let real_pos = tasks.iter().position(|t| t.uuid == *uuid)?;
                vis_id(real_pos).map(|vid| format!("#{}", vid))
            })
            .collect::<Vec<_>>()
            .join(", ");
        println!("  Blocked by: {}", ids.red());
    } else if !task.depends_on.is_empty() {
        println!("  {} All dependencies satisfied", "✓".green());
    }

    println!();
    Ok(())
}
