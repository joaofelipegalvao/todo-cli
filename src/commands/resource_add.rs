//! Handler for `todo resource add`.

use anyhow::Result;
use colored::Colorize;

use crate::cli::ResourceAddArgs;
use crate::models::Resource;
use crate::storage::Storage;

pub fn execute(storage: &impl Storage, args: ResourceAddArgs) -> Result<()> {
    let mut resources = storage.load_resources()?;

    let mut resource = Resource::new(args.title);
    resource.url = args.url;
    resource.description = args.description;
    resource.tags = args.tag;

    let id = resources.len() + 1;
    resources.push(resource);
    storage.save_resources(&resources)?;

    println!("{} Added resource #{}", "✓".green(), id);

    Ok(())
}
