//! Handler for `todo note list`.

use anyhow::Result;
use colored::Colorize;

use crate::cli::NoteListArgs;
use crate::storage::Storage;

pub fn execute(storage: &impl Storage, args: NoteListArgs) -> Result<()> {
    let (_, projects, notes) = storage.load_all()?;

    let mut visible: Vec<_> = notes.iter().filter(|n| !n.is_deleted()).collect();

    // Filter by project name
    if let Some(ref proj_name) = args.project {
        let proj_uuid = projects
            .iter()
            .find(|p| p.name.to_lowercase() == proj_name.to_lowercase() && !p.is_deleted())
            .map(|p| p.uuid);

        visible.retain(|n| proj_uuid.is_some() && n.project_id == proj_uuid);
    }

    // Filter by tag
    if let Some(ref tag) = args.tag {
        visible.retain(|n| {
            n.tags
                .iter()
                .any(|t| t.to_lowercase() == tag.to_lowercase())
        });
    }

    // Filter by language
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

    println!("\nNotes:\n");
    for (i, note) in visible.iter().enumerate() {
        let id = i + 1;

        // Title or first 60 chars of body as preview
        let preview = note.title.as_deref().unwrap_or_else(|| {
            let b = note.body.as_str();
            if b.len() > 60 { &b[..60] } else { b }
        });

        // Project name lookup
        let proj_label = note
            .project_id
            .and_then(|pid| projects.iter().find(|p| p.uuid == pid))
            .map(|p| format!(" [{}]", p.name.cyan()))
            .unwrap_or_default();

        // Language label
        let lang_label = note
            .language
            .as_deref()
            .map(|l| format!(" ({})", l.yellow()))
            .unwrap_or_default();

        // Tags
        let tags_label = if note.tags.is_empty() {
            String::new()
        } else {
            format!(" #{}", note.tags.join(" #").dimmed())
        };

        println!(
            "  {}  {}{}{}{}",
            format!("#{}", id).dimmed(),
            preview.bold(),
            proj_label,
            lang_label,
            tags_label,
        );
    }

    println!();
    Ok(())
}
