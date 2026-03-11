//! Handler for `todo context <ID>`.
//!
//! Shows everything linked to a task: project, dependencies, notes, and
//! resources (via notes). A "knowledge hub" centred on a single task.

use anyhow::Result;
use colored::Colorize;

use crate::render::formatting::truncate;
use crate::storage::Storage;
use crate::utils::validation::{validate_task_id, visible_indices};

pub fn execute(storage: &impl Storage, id: usize) -> Result<()> {
    let (tasks, projects, notes, resources) = storage.load_all_with_resources()?;

    let vis = visible_indices(&tasks, |t| t.is_deleted());
    validate_task_id(id, vis.len())?;

    let real_index = vis[id - 1];
    let task = &tasks[real_index];

    let all_visible: Vec<_> = tasks.iter().filter(|t| !t.is_deleted()).cloned().collect();
    let is_blocked = !task.completed && task.is_blocked(&all_visible);

    // ── Header ────────────────────────────────────────────────────────────────
    println!();
    println!(
        "  {}",
        format!("Task #{}: {}", id, task.text).bold().bright_white()
    );
    println!("  {}", "─".repeat(50).dimmed());

    // ── Details ───────────────────────────────────────────────────────────────
    let status = if task.completed {
        "done".green().to_string()
    } else if is_blocked {
        "blocked".red().to_string()
    } else {
        "pending".yellow().to_string()
    };
    println!("  {}  {}", "Status".dimmed(), status);
    println!("  {}  {}", "Priority".dimmed(), task.priority.letter());
    let urgency = task.urgency_score(&all_visible);
    let urgency_colored = {
        let s = format!("{:.1}", urgency);
        if urgency >= 10.0 {
            s.red().bold().to_string()
        } else if urgency >= 6.0 {
            s.yellow().to_string()
        } else if urgency >= 3.0 {
            s.normal().to_string()
        } else {
            s.dimmed().to_string()
        }
    };
    println!("  {}  {}", "Urgency".dimmed(), urgency_colored);

    if let Some(pid) = task.project_id
        && let Some(project) = projects.iter().find(|p| p.uuid == pid)
    {
        println!("  {}  {}", "Project".dimmed(), project.name.magenta());
    }

    if let Some(due) = task.due_date {
        println!("  {}  {}", "Due".dimmed(), due);
    }

    if !task.tags.is_empty() {
        println!(
            "  {}  {}",
            "Tags".dimmed(),
            task.tags
                .iter()
                .map(|t| format!("#{}", t))
                .collect::<Vec<_>>()
                .join(" ")
                .cyan()
        );
    }

    // ── Dependencies ──────────────────────────────────────────────────────────
    if !task.depends_on.is_empty() {
        println!();
        println!("  {}", "Depends on".dimmed());
        for dep_uuid in &task.depends_on {
            if let Some(dep) = all_visible.iter().find(|t| t.uuid == *dep_uuid) {
                let dep_vis_id = vis
                    .iter()
                    .position(|&i| tasks[i].uuid == *dep_uuid)
                    .map(|p| p + 1);
                let status = if dep.completed {
                    "D".green()
                } else {
                    "P".yellow()
                };
                let text = truncate(&dep.text, 40);
                let id_str = dep_vis_id
                    .map(|i| format!("#{}", i))
                    .unwrap_or_else(|| "?".to_string());
                println!(
                    "    {}  {}  {}",
                    id_str.dimmed(),
                    status,
                    text.bright_white()
                );
            }
        }
    }

    // ── Notes linked to this task ─────────────────────────────────────────────
    let task_notes: Vec<_> = notes
        .iter()
        .filter(|n| !n.is_deleted() && n.task_id == Some(task.uuid))
        .collect();

    let all_visible_notes: Vec<_> = notes.iter().filter(|n| !n.is_deleted()).collect();

    if !task_notes.is_empty() {
        println!();
        println!("  {}", "Notes".dimmed());
        for note in &task_notes {
            let note_vis_id = all_visible_notes
                .iter()
                .position(|n| n.uuid == note.uuid)
                .map(|i| i + 1)
                .unwrap_or(0);

            let preview = note.title.as_deref().unwrap_or_else(|| {
                let b = note.body.as_str();
                if b.len() > 50 { &b[..50] } else { b }
            });
            println!(
                "    {}  {}",
                format!("#{}", note_vis_id).dimmed(),
                truncate(preview, 50).bright_white()
            );
        }
    }

    // ── Resources (via notes linked to this task) ─────────────────────────────
    let all_visible_resources: Vec<_> = resources.iter().filter(|r| !r.is_deleted()).collect();

    let mut linked_resource_uuids: Vec<_> = task_notes
        .iter()
        .flat_map(|n| &n.resource_ids)
        .copied()
        .collect();
    linked_resource_uuids.dedup();

    let linked_resources: Vec<_> = linked_resource_uuids
        .iter()
        .filter_map(|uuid| all_visible_resources.iter().find(|r| r.uuid == *uuid))
        .collect();

    if !linked_resources.is_empty() {
        println!();
        println!("  {}", "Resources".dimmed());
        for resource in &linked_resources {
            let res_vis_id = all_visible_resources
                .iter()
                .position(|r| r.uuid == resource.uuid)
                .map(|i| i + 1)
                .unwrap_or(0);

            let url = resource
                .url
                .as_deref()
                .map(|u| format!("  {}", truncate(u, 40).dimmed()))
                .unwrap_or_default();

            println!(
                "    {}  {}{}",
                format!("#{}", res_vis_id).dimmed(),
                truncate(&resource.title, 40).bright_white(),
                url
            );
        }
    }

    if task_notes.is_empty() && linked_resources.is_empty() && task.depends_on.is_empty() {
        println!();
        println!(
            "  {}",
            "No linked notes, resources or dependencies.".dimmed()
        );
    }

    println!();
    Ok(())
}
