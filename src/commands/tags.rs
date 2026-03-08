//! Handler for `todo tags` and `todo tags <TAG>`.
//!
//! Without argument: shows all tags with counts across tasks, notes, resources.
//! With argument: shows a hub view of everything linked to that tag.

use anyhow::Result;
use colored::Colorize;

use crate::error::TodoError;
use crate::models::{Note, Resource, Task};
use crate::services::tag_service;
use crate::storage::Storage;
use crate::utils::tag_normalizer::has_tag;

pub fn execute(storage: &impl Storage, filter: Option<String>) -> Result<()> {
    let (tasks, _, notes, resources) = storage.load_all_with_resources()?;

    match filter {
        Some(tag) => show_tag_hub(&tasks, &notes, &resources, &tag),
        None => show_all_tags(&tasks, &notes, &resources),
    }
}

// ── show all tags ─────────────────────────────────────────────────────────────

fn show_all_tags(tasks: &[Task], notes: &[Note], resources: &[Resource]) -> Result<()> {
    let stats = tag_service::collect_tags(tasks, notes, resources);

    if stats.is_empty() {
        return Err(TodoError::NoTagsFound.into());
    }

    let name_w = stats.iter().map(|s| s.name.len()).max().unwrap_or(0);

    println!("\nTags:\n");

    for stat in &stats {
        let mut parts = Vec::new();
        if stat.tasks > 0 {
            parts.push(format!(
                "{} task{}",
                stat.tasks,
                if stat.tasks == 1 { "" } else { "s" }
            ));
        }
        if stat.notes > 0 {
            parts.push(format!(
                "{} note{}",
                stat.notes,
                if stat.notes == 1 { "" } else { "s" }
            ));
        }
        if stat.resources > 0 {
            parts.push(format!(
                "{} resource{}",
                stat.resources,
                if stat.resources == 1 { "" } else { "s" }
            ));
        }
        println!(
            "  {:<name_w$}  ({})",
            stat.name.cyan(),
            parts.join(", ").dimmed(),
            name_w = name_w,
        );
    }

    println!();
    Ok(())
}

// ── show tag hub ──────────────────────────────────────────────────────────────

fn show_tag_hub(tasks: &[Task], notes: &[Note], resources: &[Resource], tag: &str) -> Result<()> {
    // All visible tasks — used to resolve real display IDs
    let all_visible: Vec<_> = tasks.iter().filter(|t| !t.is_deleted()).collect();

    let matched_tasks: Vec<_> = all_visible
        .iter()
        .filter(|t| has_tag(&t.tags, tag))
        .collect();

    let matched_notes: Vec<_> = notes
        .iter()
        .filter(|n| !n.is_deleted())
        .filter(|n| has_tag(&n.tags, tag))
        .collect();

    // All visible resources — used to resolve real display IDs
    let all_visible_resources: Vec<_> = resources.iter().filter(|r| !r.is_deleted()).collect();

    let matched_resources: Vec<_> = all_visible_resources
        .iter()
        .filter(|r| has_tag(&r.tags, tag))
        .collect();

    // All visible notes — used to resolve real display IDs
    let all_visible_notes: Vec<_> = notes.iter().filter(|n| !n.is_deleted()).collect();

    if matched_tasks.is_empty() && matched_notes.is_empty() && matched_resources.is_empty() {
        return Err(TodoError::TagNotFound(tag.to_owned()).into());
    }

    println!();
    println!("  {}", format!("Tag: #{}", tag).bold().cyan());
    println!("  {}", "─".repeat(40).dimmed());

    if !matched_tasks.is_empty() {
        println!();
        println!("  {}", "Tasks".dimmed());
        for task in &matched_tasks {
            // Real display ID = position in all visible tasks
            let vis_id = all_visible
                .iter()
                .position(|t| t.uuid == task.uuid)
                .map(|i| i + 1)
                .unwrap_or(0);

            let status = if task.completed {
                "✓".green()
            } else {
                "·".yellow()
            };
            println!(
                "    {:<4}  {}  {}",
                format!("#{}", vis_id).dimmed(),
                status,
                task.text.bright_white()
            );
        }
    }

    if !matched_notes.is_empty() {
        println!();
        println!("  {}", "Notes".dimmed());
        for note in &matched_notes {
            let vis_id = all_visible_notes
                .iter()
                .position(|n| n.uuid == note.uuid)
                .map(|i| i + 1)
                .unwrap_or(0);

            let preview = note.title.as_deref().unwrap_or_else(|| {
                let b = note.body.as_str();
                if b.len() > 50 { &b[..50] } else { b }
            });
            println!(
                "    {:<4}  {}",
                format!("#{}", vis_id).dimmed(),
                preview.bright_white()
            );
        }
    }

    if !matched_resources.is_empty() {
        println!();
        println!("  {}", "Resources".dimmed());
        for resource in &matched_resources {
            let vis_id = all_visible_resources
                .iter()
                .position(|r| r.uuid == resource.uuid)
                .map(|i| i + 1)
                .unwrap_or(0);

            let url = resource
                .url
                .as_deref()
                .map(|u| format!("  {}", u.dimmed()))
                .unwrap_or_default();
            println!(
                "    {:<4}  {}{}",
                format!("#{}", vis_id).dimmed(),
                resource.title.bright_white(),
                url
            );
        }
    }

    println!();
    Ok(())
}
