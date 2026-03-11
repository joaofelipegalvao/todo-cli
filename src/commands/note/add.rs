//! Handler for `todo note add`.

use anyhow::Result;
use colored::Colorize;

use crate::cli::NoteAddArgs;
use crate::models::{Note, Project};
use crate::services::tag_service::collect_all_tag_names;
use crate::storage::Storage;
use crate::utils::tag_normalizer::normalize_tags;

pub fn execute(storage: &impl Storage, args: NoteAddArgs) -> Result<()> {
    let (tasks, projects, mut notes) = storage.load_all()?;
    let resources = storage.load_resources()?;

    // Resolve project name → UUID (creates the project if it doesn't exist yet)
    let project_id = if let Some(ref name) = args.project {
        Some(Project::resolve_or_create(storage, &projects, name)?)
    } else {
        None
    };

    // Resolve task display-id → UUID
    let task_id = if let Some(task_num) = args.task {
        let task = tasks
            .get(task_num.saturating_sub(1))
            .ok_or_else(|| anyhow::anyhow!("Task #{} not found", task_num))?;
        Some(task.uuid)
    } else {
        None
    };

    let existing_tags = collect_all_tag_names(&tasks, &notes, &resources);
    let (normalized_tags, normalization_messages) = normalize_tags(args.tag, &existing_tags);

    let mut note = Note::new(args.body);
    note.title = args.title;
    note.tags = normalized_tags;
    note.language = args.language;
    note.project_id = project_id;
    note.task_id = task_id;

    let id = notes.len() + 1;
    notes.push(note);
    storage.save_notes(&notes)?;

    for msg in &normalization_messages {
        println!("  {} Tag normalized: {}", "~".yellow(), msg.yellow());
    }
    println!("{} Added note #{}", "✓".green(), id);

    Ok(())
}
