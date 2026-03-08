//! Handler for `todo resource show <ID>`.

use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;

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

    // ── Metadata (Type + Tags first, then URL) ────────────────────────────────
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
            let preview = note.title.as_deref().unwrap_or_else(|| {
                let b = note.body.as_str();
                if b.len() > 50 { &b[..50] } else { b }
            });
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
