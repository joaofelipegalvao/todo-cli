//! Handler for `todo note add`.

use anyhow::{Result, anyhow};
use colored::Colorize;

use crate::cli::NoteAddArgs;
use crate::models::{Note, Project};
use crate::services::tag_service::collect_all_tag_names;
use crate::storage::Storage;
use crate::utils::tag_normalizer::normalize_tags;
use crate::utils::validation::resolve_visible;

pub fn execute(storage: &impl Storage, args: NoteAddArgs) -> Result<()> {
    let (tasks, projects, mut notes) = storage.load_all()?;
    let resources = storage.load_resources()?;

    // ── Resolve body from input source ────────────────────────────────────────
    let (body, is_markdown) = match (args.body, args.editor, args.file) {
        (Some(text), false, None) => (text, false),

        (None, true, None) => {
            let content = edit::edit_with_builder("", edit::Builder::new().suffix(".md"))?;
            let trimmed = content.trim().to_string();
            if trimmed.is_empty() {
                return Err(anyhow!("Aborted: note body is empty."));
            }
            (trimmed, true)
        }

        (None, false, Some(path)) => {
            let content = std::fs::read_to_string(&path)
                .map_err(|e| anyhow!("Failed to read file {}: {}", path.display(), e))?;
            (content, true)
        }

        (None, false, None) => {
            return Err(anyhow!(
                "Provide a note body: <BODY>, --editor, or --file <PATH>"
            ));
        }

        _ => {
            return Err(anyhow!(
                "Only one input source allowed: <BODY>, --editor, or --file <PATH>"
            ));
        }
    };

    // ── Resolve project name → UUID ───────────────────────────────────────────
    let project_id = if let Some(ref name) = args.project {
        Some(Project::resolve_or_create(storage, &projects, name)?)
    } else {
        None
    };

    // ── Resolve task display-id → UUID ────────────────────────────────────────
    let task_id = if let Some(task_num) = args.task {
        let task = resolve_visible(&tasks, task_num, |t| t.is_deleted())
            .map_err(|_| anyhow!("Task #{} not found", task_num))?;
        Some(task.uuid)
    } else {
        None
    };

    let existing_tags = collect_all_tag_names(&tasks, &notes, &resources);
    let (normalized_tags, normalization_messages) = normalize_tags(args.tag, &existing_tags);

    let mut note = if is_markdown {
        Note::new_markdown(body)
    } else {
        Note::new(body)
    };
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
