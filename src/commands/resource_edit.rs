//! Handler for `todo resource edit <ID>`.
//!
//! Applies partial updates to an existing resource. Only fields explicitly
//! provided are changed; everything else is preserved.

use anyhow::Result;
use colored::Colorize;

use crate::cli::ResourceEditArgs;
use crate::storage::Storage;

pub fn execute(storage: &impl Storage, args: ResourceEditArgs) -> Result<()> {
    let mut resources = storage.load_resources()?;

    let visible: Vec<usize> = resources
        .iter()
        .enumerate()
        .filter(|(_, r)| !r.is_deleted())
        .map(|(i, _)| i)
        .collect();

    let real_index = visible
        .get(args.id.saturating_sub(1))
        .copied()
        .ok_or_else(|| anyhow::anyhow!("Resource #{} not found", args.id))?;

    let resource = &mut resources[real_index];
    let mut changes = Vec::new();

    // ── title ─────────────────────────────────────────────────────────────────
    if let Some(new_title) = args.title {
        if new_title.trim().is_empty() {
            return Err(anyhow::anyhow!("Resource title cannot be empty"));
        }
        if resource.title != new_title {
            resource.title = new_title.clone();
            changes.push(format!("title → {}", new_title.bright_white()));
        }
    }

    // ── type ──────────────────────────────────────────────────────────────────
    if args.clear_type {
        if resource.resource_type.is_some() {
            resource.resource_type = None;
            changes.push("type → cleared".dimmed().to_string());
        }
    } else if let Some(new_type) = args.r#type
        && resource.resource_type != Some(new_type)
    {
        resource.resource_type = Some(new_type);
        changes.push(format!("type → {}", new_type.label().cyan()));
    }

    // ── url ───────────────────────────────────────────────────────────────────
    if args.clear_url {
        if resource.url.is_some() {
            resource.url = None;
            changes.push("url → cleared".dimmed().to_string());
        }
    } else if let Some(new_url) = args.url
        && resource.url.as_deref() != Some(new_url.as_str())
    {
        resource.url = Some(new_url.clone());
        changes.push(format!("url → {}", new_url.cyan()));
    }

    // ── description ───────────────────────────────────────────────────────────
    if args.clear_description {
        if resource.description.is_some() {
            resource.description = None;
            changes.push("description → cleared".dimmed().to_string());
        }
    } else if let Some(new_desc) = args.description
        && resource.description.as_deref() != Some(new_desc.as_str())
    {
        resource.description = Some(new_desc.clone());
        changes.push(format!("description → {}", new_desc.bright_white()));
    }

    // ── tags ──────────────────────────────────────────────────────────────────
    if args.clear_tags {
        if !resource.tags.is_empty() {
            let old = resource.tags.clone();
            resource.tags.clear();
            changes.push(format!("tags cleared → was [{}]", old.join(", ").dimmed()));
        }
    } else {
        if !args.remove_tag.is_empty() {
            let mut removed = Vec::new();
            resource.tags.retain(|t| {
                if args.remove_tag.contains(t) {
                    removed.push(t.clone());
                    false
                } else {
                    true
                }
            });
            if !removed.is_empty() {
                changes.push(format!("removed tags → [{}]", removed.join(", ").red()));
            }
        }
        if !args.add_tag.is_empty() {
            let mut added = Vec::new();
            for tag in &args.add_tag {
                if !resource.tags.contains(tag) {
                    resource.tags.push(tag.clone());
                    added.push(tag.clone());
                }
            }
            if !added.is_empty() {
                changes.push(format!("added tags → [{}]", added.join(", ").cyan()));
            }
        }
    }

    // ── persist ───────────────────────────────────────────────────────────────
    if changes.is_empty() {
        println!(
            "{} No changes made (values are already set to the specified values).",
            "".blue()
        );
        return Ok(());
    }

    resources[real_index].touch();
    storage.save_resources(&resources)?;

    println!("{} Resource #{} updated:", "✓".green(), args.id);
    for change in &changes {
        println!("  • {}", change);
    }

    Ok(())
}
