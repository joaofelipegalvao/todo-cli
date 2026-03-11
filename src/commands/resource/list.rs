//! Handler for `todo resource list`.

use anyhow::Result;
use colored::Colorize;

use crate::cli::ResourceListArgs;
use crate::render::display_resources;
use crate::storage::Storage;
use crate::utils::tag_normalizer::has_tag;

pub fn execute(storage: &impl Storage, args: ResourceListArgs) -> Result<()> {
    let (_, _, notes) = storage.load_all()?;
    let resources = storage.load_resources()?;

    let mut visible: Vec<_> = resources.iter().filter(|r| !r.is_deleted()).collect();

    if let Some(ref tag) = args.tag {
        visible.retain(|r| has_tag(&r.tags, tag));
    }

    if let Some(filter_type) = args.r#type {
        visible.retain(|r| r.resource_type == Some(filter_type));
    }

    if visible.is_empty() {
        println!("{}", "No resources found.".dimmed());
        return Ok(());
    }

    display_resources(&visible, &notes);
    Ok(())
}
