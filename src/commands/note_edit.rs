//! Handler for `todo note edit <ID>`.
//!
//! Applies partial updates to an existing note. Only fields explicitly
//! provided are changed; everything else is preserved.

use anyhow::Result;
use colored::Colorize;

use crate::cli::NoteEditArgs;
use crate::models::Project;
use crate::storage::Storage;

pub fn execute(storage: &impl Storage, args: NoteEditArgs) -> Result<()> {
    let (tasks, projects, mut notes, resources) = storage.load_all_with_resources()?;

    let visible: Vec<usize> = notes
        .iter()
        .enumerate()
        .filter(|(_, n)| !n.is_deleted())
        .map(|(i, _)| i)
        .collect();

    let real_index = visible
        .get(args.id.saturating_sub(1))
        .copied()
        .ok_or_else(|| anyhow::anyhow!("Note #{} not found", args.id))?;

    let note = &mut notes[real_index];
    let mut changes = Vec::new();

    // ── body ──────────────────────────────────────────────────────────────────
    if let Some(new_body) = args.body {
        if new_body.trim().is_empty() {
            return Err(anyhow::anyhow!("Note body cannot be empty"));
        }
        if note.body != new_body {
            note.body = new_body.clone();
            changes.push(format!("body → {}", new_body.bright_white()));
        }
    }

    // ── title ─────────────────────────────────────────────────────────────────
    if args.clear_title {
        if note.title.is_some() {
            note.title = None;
            changes.push("title → cleared".dimmed().to_string());
        }
    } else if let Some(new_title) = args.title {
        if note.title.as_deref() != Some(new_title.as_str()) {
            note.title = Some(new_title.clone());
            changes.push(format!("title → {}", new_title.bright_white()));
        }
    }

    // ── language ──────────────────────────────────────────────────────────────
    if args.clear_language {
        if note.language.is_some() {
            note.language = None;
            changes.push("language → cleared".dimmed().to_string());
        }
    } else if let Some(new_lang) = args.language {
        if note.language.as_deref() != Some(new_lang.as_str()) {
            note.language = Some(new_lang.clone());
            changes.push(format!("language → {}", new_lang.yellow()));
        }
    }

    // ── tags ──────────────────────────────────────────────────────────────────
    if args.clear_tags {
        if !note.tags.is_empty() {
            let old = note.tags.clone();
            note.tags.clear();
            changes.push(format!("tags cleared → was [{}]", old.join(", ").dimmed()));
        }
    } else {
        if !args.remove_tag.is_empty() {
            let mut removed = Vec::new();
            note.tags.retain(|t| {
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
                if !note.tags.contains(tag) {
                    note.tags.push(tag.clone());
                    added.push(tag.clone());
                }
            }
            if !added.is_empty() {
                changes.push(format!("added tags → [{}]", added.join(", ").cyan()));
            }
        }
    }

    // ── project ───────────────────────────────────────────────────────────────
    if args.clear_project {
        if note.project_id.is_some() {
            note.project_id = None;
            changes.push("project → cleared".dimmed().to_string());
        }
    } else if let Some(ref proj_name) = args.project {
        let new_uuid = Project::resolve_or_create(storage, &projects, proj_name)?;
        if note.project_id != Some(new_uuid) {
            note.project_id = Some(new_uuid);
            changes.push(format!("project → {}", proj_name.cyan()));
        }
    }

    // ── task link ─────────────────────────────────────────────────────────────
    if args.clear_task {
        if note.task_id.is_some() {
            note.task_id = None;
            changes.push("task → cleared".dimmed().to_string());
        }
    } else if let Some(task_num) = args.task {
        let task = tasks
            .get(task_num.saturating_sub(1))
            .ok_or_else(|| anyhow::anyhow!("Task #{} not found", task_num))?;
        if note.task_id != Some(task.uuid) {
            note.task_id = Some(task.uuid);
            changes.push(format!("task → #{} {}", task_num, task.text.cyan()));
        }
    }

    // ── resource links ────────────────────────────────────────────────────────
    if args.clear_resources {
        if !note.resource_ids.is_empty() {
            note.resource_ids.clear();
            changes.push("resources → cleared".dimmed().to_string());
        }
    } else {
        if !args.remove_resource.is_empty() {
            let mut removed = Vec::new();
            for res_num in &args.remove_resource {
                let visible_resources: Vec<_> =
                    resources.iter().filter(|r| !r.is_deleted()).collect();
                let resource = visible_resources
                    .get(res_num.saturating_sub(1))
                    .ok_or_else(|| anyhow::anyhow!("Resource #{} not found", res_num))?;
                if note.resource_ids.contains(&resource.uuid) {
                    note.remove_resource(resource.uuid);
                    removed.push(resource.title.clone());
                }
            }
            if !removed.is_empty() {
                changes.push(format!(
                    "removed resources → [{}]",
                    removed.join(", ").red()
                ));
            }
        }
        if !args.add_resource.is_empty() {
            let mut added = Vec::new();
            for res_num in &args.add_resource {
                let visible_resources: Vec<_> =
                    resources.iter().filter(|r| !r.is_deleted()).collect();
                let resource = visible_resources
                    .get(res_num.saturating_sub(1))
                    .ok_or_else(|| anyhow::anyhow!("Resource #{} not found", res_num))?;
                if !note.resource_ids.contains(&resource.uuid) {
                    note.add_resource(resource.uuid);
                    added.push(resource.title.clone());
                }
            }
            if !added.is_empty() {
                changes.push(format!("added resources → [{}]", added.join(", ").cyan()));
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

    notes[real_index].touch();
    storage.save_notes(&notes)?;

    println!("{} Note #{} updated:", "✓".green(), args.id);
    for change in &changes {
        println!("  • {}", change);
    }

    Ok(())
}
