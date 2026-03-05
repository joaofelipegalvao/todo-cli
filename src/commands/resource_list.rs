//! Handler for `todo resource list`.

use anyhow::Result;
use colored::Colorize;

use crate::cli::ResourceListArgs;
use crate::storage::Storage;

pub fn execute(storage: &impl Storage, args: ResourceListArgs) -> Result<()> {
    let resources = storage.load_resources()?;

    let mut visible: Vec<_> = resources.iter().filter(|r| !r.is_deleted()).collect();

    // Filter by tag
    if let Some(ref tag) = args.tag {
        visible.retain(|r| {
            r.tags
                .iter()
                .any(|t| t.to_lowercase() == tag.to_lowercase())
        });
    }

    if visible.is_empty() {
        println!("{}", "No resources found.".dimmed());
        return Ok(());
    }

    println!("\nResources:\n");
    for (i, resource) in visible.iter().enumerate() {
        let id = i + 1;

        // URL label
        let url_label = resource
            .url
            .as_deref()
            .map(|u| format!(" {}", u.dimmed()))
            .unwrap_or_default();

        // Tags
        let tags_label = if resource.tags.is_empty() {
            String::new()
        } else {
            format!(" #{}", resource.tags.join(" #").dimmed())
        };

        println!(
            "  {}  {}{}{}",
            format!("#{}", id).dimmed(),
            resource.title.bold(),
            url_label,
            tags_label,
        );

        if let Some(ref desc) = resource.description {
            let preview = if desc.len() > 80 { &desc[..80] } else { desc };
            println!("      {}", preview.dimmed());
        }
    }

    println!();
    Ok(())
}
