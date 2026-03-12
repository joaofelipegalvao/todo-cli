//! Handler for `todo resource show <ID>`.

use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;

/// Returns a single-line preview of a note — title if set, otherwise first non-empty line.
fn note_preview(note: &crate::models::Note) -> String {
    if let Some(ref title) = note.title {
        return title.clone();
    }
    note.body
        .lines()
        .find(|l| !l.trim().is_empty())
        .map(|l| l.trim_start_matches('#').trim().to_string())
        .unwrap_or_default()
}

pub fn execute(storage: &impl Storage, id: usize) -> Result<()> {
    let (_, _, notes, resources) = storage.load_all_with_resources()?;

    let visible_resources: Vec<_> = resources.iter().filter(|r| !r.is_deleted()).collect();

    let resource = visible_resources
        .get(id.saturating_sub(1))
        .ok_or_else(|| anyhow::anyhow!("Resource #{} not found", id))?;

    let visible_notes: Vec<_> = notes.iter().filter(|n| !n.is_deleted()).collect();

    // ── Header ────────────────────────────────────────────────────────────────
    println!();
    println!(
        "  {}",
        format!("Resource #{}: {}", id, resource.title)
            .bold()
            .cyan()
    );
    println!("  {}", "─".repeat(50).dimmed());

    // ── Metadata ──────────────────────────────────────────────────────────────
    if let Some(rt) = resource.resource_type {
        println!("  {}  {}", "Type".dimmed(), rt.label().yellow());
    }

    if !resource.tags.is_empty() {
        println!(
            "  {}  {}",
            "Tags".dimmed(),
            resource
                .tags
                .iter()
                .map(|t| format!("#{}", t))
                .collect::<Vec<_>>()
                .join("  ")
                .cyan()
        );
    }

    if let Some(ref url) = resource.url {
        println!("  {}  {}", "URL".dimmed(), url.cyan().underline());
    }

    if let Some(ref desc) = resource.description {
        println!();
        for line in desc.lines() {
            println!("  {}", line);
        }
    }

    println!();
    println!("  {}", "─".repeat(50).dimmed());

    // ── Notes that reference this resource ────────────────────────────────────
    let referencing: Vec<(usize, _)> = visible_notes
        .iter()
        .enumerate()
        .filter(|(_, n)| n.references_resource(resource.uuid))
        .map(|(i, n)| (i + 1, n))
        .collect();

    if !referencing.is_empty() {
        println!();
        println!(
            "  {}  {}",
            "Used in notes".dimmed(),
            referencing.len().to_string().dimmed()
        );
        for (note_id, note) in &referencing {
            let preview = note_preview(note);
            println!(
                "    {}  {}",
                format!("#{}", note_id).dimmed(),
                preview.bright_white()
            );
        }
    }

    println!();
    println!(
        "  {}  {}",
        "Created".dimmed(),
        resource.created_at.format("%Y-%m-%d")
    );
    println!();

    Ok(())
}
