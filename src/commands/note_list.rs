//! Handler for `todo note list`.

use anyhow::Result;
use colored::Colorize;

use crate::cli::NoteListArgs;
use crate::render::display_notes;
use crate::storage::Storage;
use crate::utils::tag_normalizer::has_tag;

pub fn execute(storage: &impl Storage, args: NoteListArgs) -> Result<()> {
    let (_, projects, notes) = storage.load_all()?;
    let resources = storage.load_resources()?;

    let mut visible: Vec<_> = notes.iter().filter(|n| !n.is_deleted()).collect();

    if let Some(ref proj_name) = args.project {
        let proj_uuid = projects
            .iter()
            .find(|p| p.name.to_lowercase() == proj_name.to_lowercase() && !p.is_deleted())
            .map(|p| p.uuid);
        visible.retain(|n| proj_uuid.is_some() && n.project_id == proj_uuid);
    }

    if let Some(ref tag) = args.tag {
        visible.retain(|n| has_tag(&n.tags, tag));
    }

    if let Some(ref lang) = args.language {
        visible.retain(|n| {
            n.language
                .as_deref()
                .map(|l| l.to_lowercase() == lang.to_lowercase())
                .unwrap_or(false)
        });
    }

    if visible.is_empty() {
        println!("{}", "No notes found.".dimmed());
        return Ok(());
    }

    display_notes(&visible, &projects, &resources);
    Ok(())
}
