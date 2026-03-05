// src/commands/note_add.rs

//! Handler for `todo note add`.

use anyhow::Result;
use colored::Colorize;

use crate::cli::NoteAddArgs;
use crate::models::{Note, Project};
use crate::storage::Storage;

pub fn execute(storage: &impl Storage, args: NoteAddArgs) -> Result<()> {
    let (tasks, projects, mut notes) = storage.load_all()?;

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

    let mut note = Note::new(args.body);
    note.title = args.title;
    note.tags = args.tag;
    note.language = args.language;
    note.project_id = project_id;
    note.task_id = task_id;

    let id = notes.len() + 1;
    notes.push(note);
    storage.save_notes(&notes)?;

    println!("{} Added note #{}", "✓".green(), id);

    Ok(())
}
