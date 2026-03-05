// src/commands/note_show.rs

//! Handler for `todo note show <ID>`.

use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;

pub fn execute(storage: &impl Storage, id: usize) -> Result<()> {
    let (tasks, projects, notes, resources) = storage.load_all_with_resources()?;

    let visible: Vec<_> = notes.iter().filter(|n| !n.is_deleted()).collect();

    let note = visible
        .get(id.saturating_sub(1))
        .ok_or_else(|| anyhow::anyhow!("Note #{} not found", id))?;

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

    if let Some(pid) = note.project_id {
        if let Some(project) = projects.iter().find(|p| p.uuid == pid) {
            println!("  {} {}", "Project:".dimmed(), project.name.cyan());
        }
    }

    if let Some(tid) = note.task_id {
        if let Some(task) = tasks.iter().find(|t| t.uuid == tid) {
            println!("  {} {}", "Task:".dimmed(), task.text.cyan());
        }
    }

    // ── Resources ─────────────────────────────────────────────────────────────
    if !note.resource_ids.is_empty() {
        println!("  {}", "Resources:".dimmed());
        for rid in &note.resource_ids {
            if let Some(resource) = resources.iter().find(|r| r.uuid == *rid && !r.is_deleted()) {
                let url_part = resource
                    .url
                    .as_deref()
                    .map(|u| format!(" — {}", u.dimmed()))
                    .unwrap_or_default();
                println!("    {} {}{}", "·".dimmed(), resource.title.cyan(), url_part);
            }
        }
    }

    println!("  {} {}", "Created:".dimmed(), note.created_at);
    println!();

    Ok(())
}
