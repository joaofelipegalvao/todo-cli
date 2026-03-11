//! Handler for `todo resource add`.

use anyhow::Result;
use colored::Colorize;

use crate::cli::ResourceAddArgs;
use crate::models::Resource;
use crate::services::tag_service::collect_all_tag_names;
use crate::storage::Storage;
use crate::utils::tag_normalizer::normalize_tags;

pub fn execute(storage: &impl Storage, args: ResourceAddArgs) -> Result<()> {
    let mut resources = storage.load_resources()?;
    let tasks = storage.load()?;
    let notes = storage.load_notes()?;

    // ── Duplicate warning (by URL if present, otherwise by title) ─────────────
    let duplicate = if let Some(ref url) = args.url {
        resources
            .iter()
            .filter(|r| !r.is_deleted())
            .find(|r| r.url.as_deref() == Some(url.as_str()))
    } else {
        resources
            .iter()
            .filter(|r| !r.is_deleted())
            .find(|r| r.title.to_lowercase() == args.title.to_lowercase())
    };

    if let Some(existing) = duplicate {
        let visible_id = resources
            .iter()
            .filter(|r| !r.is_deleted())
            .position(|r| r.uuid == existing.uuid)
            .map(|i| i + 1)
            .unwrap_or(0);
        let reason = if args.url.is_some() { "URL" } else { "title" };
        eprintln!(
            "{} Resource with same {} \"{}\" already exists (#{}). Add anyway? [y/N] ",
            "".yellow(),
            reason,
            existing.title,
            visible_id
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            println!("{}", "Cancelled.".dimmed());
            return Ok(());
        }
    }

    let existing_tags = collect_all_tag_names(&tasks, &notes, &resources);
    let (normalized_tags, normalization_messages) = normalize_tags(args.tag, &existing_tags);

    let mut resource = Resource::new(args.title);
    resource.resource_type = args.r#type;
    resource.url = args.url;
    resource.description = args.description;
    resource.tags = normalized_tags;

    let visible_id = resources.iter().filter(|r| !r.is_deleted()).count() + 1;
    resources.push(resource);
    storage.save_resources(&resources)?;

    for msg in &normalization_messages {
        println!("  {} Tag normalized: {}", "~".yellow(), msg.yellow());
    }
    println!("{} Added resource #{}", "✓".green(), visible_id);

    Ok(())
}
