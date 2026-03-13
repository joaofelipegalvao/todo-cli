//! Handler for `todo note show <ID>`.

use anyhow::Result;
use colored::Colorize;

use crate::models::NoteFormat;
use crate::storage::Storage;
use crate::utils::validation::resolve_visible;

pub fn execute(storage: &impl Storage, id: usize) -> Result<()> {
    let (tasks, projects, notes, resources) = storage.load_all_with_resources()?;

    let note = resolve_visible(&notes, id, |n| n.is_deleted())
        .map_err(|_| anyhow::anyhow!("Note #{} not found", id))?;

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
    if note.format == NoteFormat::Markdown {
        let first_line = note
            .body
            .lines()
            .find(|l| !l.trim().is_empty())
            .map(|l| l.trim_start_matches('#').trim())
            .unwrap_or("");
        println!("  {}", first_line.bold());
        println!();
        println!(
            "  {} {}",
            "tip:".dimmed(),
            format!("run `todo note preview {}` to render markdown", id).dimmed()
        );
    } else {
        for line in note.body.lines() {
            println!("  {}", line);
        }
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

    // ── Resources ─────────────────────────────────────────────────────────────
    if !note.resource_ids.is_empty() {
        println!("  {}", "Resources:".dimmed());
        for rid in &note.resource_ids {
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
