//! Handler for `todo resource show <ID>`.

use anyhow::Result;
use colored::Colorize;

use crate::storage::Storage;

pub fn execute(storage: &impl Storage, id: usize) -> Result<()> {
    let (_, _, notes, resources) = storage.load_all_with_resources()?;

    let visible: Vec<_> = resources.iter().filter(|r| !r.is_deleted()).collect();

    let resource = visible
        .get(id.saturating_sub(1))
        .ok_or_else(|| anyhow::anyhow!("Resource #{} not found", id))?;

    // ── Header ────────────────────────────────────────────────────────────────
    println!();
    println!("  {} {}", "Resource:".dimmed(), resource.title.bold());
    println!("  {}", "─".repeat(50).dimmed());

    // ── URL ───────────────────────────────────────────────────────────────────
    if let Some(ref url) = resource.url {
        println!();
        println!("  {} {}", "URL:".dimmed(), url.cyan().underline());
    }

    // ── Description ───────────────────────────────────────────────────────────
    if let Some(ref desc) = resource.description {
        println!();
        for line in desc.lines() {
            println!("  {}", line);
        }
    }

    println!();

    // ── Metadata ──────────────────────────────────────────────────────────────
    println!("  {}", "─".repeat(50).dimmed());

    if !resource.tags.is_empty() {
        println!(
            "  {} {}",
            "Tags:".dimmed(),
            resource
                .tags
                .iter()
                .map(|t| format!("#{}", t))
                .collect::<Vec<_>>()
                .join("  ")
                .cyan()
        );
    }

    // ── Notes that reference this resource ────────────────────────────────────
    let referencing: Vec<_> = notes
        .iter()
        .filter(|n| !n.is_deleted() && n.references_resource(resource.uuid))
        .collect();

    if !referencing.is_empty() {
        println!("  {} {}", "Used in notes:".dimmed(), referencing.len());
        for note in &referencing {
            let preview = note.title.as_deref().unwrap_or_else(|| {
                let b = note.body.as_str();
                if b.len() > 50 { &b[..50] } else { b }
            });
            println!("    {} {}", "·".dimmed(), preview.dimmed());
        }
    }

    println!("  {} {}", "Created:".dimmed(), resource.created_at);
    println!();

    Ok(())
}
