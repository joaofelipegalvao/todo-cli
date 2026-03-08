// src/commands/note_show.rs

//! Handler for `todo note show <ID>`.

use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;

pub fn execute(storage: &impl Storage, id: usize) -> Result<()> {
    let (tasks, projects, notes, resources) = storage.load_all_with_resources()?;

    let visible_notes: Vec<_> = notes.iter().filter(|n| !n.is_deleted()).collect();

    let note = visible_notes
        .get(id.saturating_sub(1))
        .ok_or_else(|| anyhow::anyhow!("Note #{} not found", id))?;

    // Build global resource ID lookup
    let visible_resources: Vec<_> = resources.iter().filter(|r| !r.is_deleted()).collect();

    // ── Header ────────────────────────────────────────────────────────────────
    println!();
    if let Some(ref title) = note.title {
        println!("  {} {}", "Note:".dimmed(), title.bold());
    } else {
        println!("  {}", format!("Note #{}", id).bold());
    }

    println!("  {}", "─".repeat(50).dimmed());

    // ── Body ──────────────────────────────────────────────────────────────────
    println!();
    for line in note.body.lines() {
        println!("  {}", line);
    }
    println!();

    // ── Metadata ──────────────────────────────────────────────────────────────
    println!("  {}", "─".repeat(50).dimmed());

    if let Some(ref lang) = note.language {
        println!("  {} {}", "Language:".dimmed(), lang.yellow());
    }

    if !note.tags.is_empty() {
        println!(
            "  {} {}",
            "Tags:".dimmed(),
            note.tags
                .iter()
                .map(|t| format!("#{}", t))
                .collect::<Vec<_>>()
                .join("  ")
                .cyan()
        );
    }

    if let Some(pid) = note.project_id
        && let Some(project) = projects.iter().find(|p| p.uuid == pid)
    {
        println!("  {} {}", "Project:".dimmed(), project.name.cyan());
    }

    if let Some(tid) = note.task_id
        && let Some(task) = tasks.iter().find(|t| t.uuid == tid)
    {
        println!("  {} {}", "Task:".dimmed(), task.text.cyan());
    }

    // ── Resources (with global IDs) ───────────────────────────────────────────
    if !note.resource_ids.is_empty() {
        println!("  {}", "Resources:".dimmed());
        for rid in &note.resource_ids {
            // Find global display ID = position in visible_resources
            let found = visible_resources
                .iter()
                .enumerate()
                .find(|(_, r)| r.uuid == *rid);

            if let Some((idx, resource)) = found {
                let resource_id = idx + 1;
                let url_part = resource
                    .url
                    .as_deref()
                    .map(|u| format!(" — {}", u.dimmed()))
                    .unwrap_or_default();
                println!(
                    "    {} {} {}{}",
                    "·".dimmed(),
                    format!("#{}", resource_id).dimmed(),
                    resource.title.cyan(),
                    url_part
                );
            }
        }
    }

    println!(
        "  {} {}",
        "Created:".dimmed(),
        note.created_at.format("%Y-%m-%d")
    );
    println!();

    Ok(())
}
