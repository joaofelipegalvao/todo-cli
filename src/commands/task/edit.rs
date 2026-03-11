//! Handler for `todo edit <ID>`.

use anyhow::Result;
use colored::Colorize;
use uuid::Uuid;

use crate::cli::EditArgs;
use crate::error::TodoError;
use crate::models::{Project, detect_cycle};
use crate::storage::Storage;
use crate::utils::date_parser;
use crate::utils::validation::{self, validate_task_id, visible_indices};

pub fn execute(storage: &impl Storage, args: EditArgs) -> Result<()> {
    execute_inner(storage, args, false)?;
    Ok(())
}

/// TUI variant: same logic, no stdout, returns a summary string.
pub fn execute_silent(storage: &impl Storage, args: EditArgs) -> Result<String> {
    execute_inner(storage, args, true)
}

fn execute_inner(storage: &impl Storage, args: EditArgs, silent: bool) -> Result<String> {
    let due = if let Some(ref due_str) = args.due {
        Some(date_parser::parse_date(due_str)?)
    } else {
        None
    };

    let mut tasks = storage.load()?;
    let vis = visible_indices(&tasks, |t| t.is_deleted());
    validate_task_id(args.id, vis.len())?;
    let real_index = vis[args.id - 1];

    let add_dep_uuids: Vec<Uuid> = args
        .add_dep
        .iter()
        .map(|&id| validation::resolve_uuid_visible(id, &tasks))
        .collect::<Result<_, _>>()
        .map_err(anyhow::Error::from)?;

    let remove_dep_uuids: Vec<Uuid> = args
        .remove_dep
        .iter()
        .map(|&id| validation::resolve_uuid_visible(id, &tasks))
        .collect::<Result<_, _>>()
        .map_err(anyhow::Error::from)?;

    let current_deps_display = tasks[real_index]
        .depends_on
        .iter()
        .filter_map(|uuid| {
            let real_pos = tasks.iter().position(|t| t.uuid == *uuid)?;
            let vis_id = vis.iter().position(|&i| i == real_pos).map(|p| p + 1)?;
            Some(format!("#{}", vis_id))
        })
        .collect::<Vec<_>>()
        .join(", ");

    let mut changes = Vec::new();

    for (dep_id, &dep_uuid) in args.add_dep.iter().zip(add_dep_uuids.iter()) {
        if *dep_id == args.id {
            return Err(TodoError::SelfDependency { task_id: args.id }.into());
        }
        validate_task_id(*dep_id, vis.len())?;
        detect_cycle(&tasks, tasks[real_index].uuid, dep_uuid)
            .map_err(TodoError::DependencyCycle)?;
        if tasks[real_index].depends_on.contains(&dep_uuid) {
            return Err(TodoError::DuplicateDependency {
                task_id: args.id,
                dep_id: *dep_id,
            }
            .into());
        }
    }
    for (dep_id, dep_uuid) in args.remove_dep.iter().zip(remove_dep_uuids.iter()) {
        if !tasks[real_index].depends_on.contains(dep_uuid) {
            return Err(TodoError::DependencyNotFound {
                task_id: args.id,
                dep_id: *dep_id,
            }
            .into());
        }
    }

    let task = &mut tasks[real_index];

    if let Some(new_text) = args.text {
        if new_text.trim().is_empty() {
            return Err(anyhow::anyhow!("Task text cannot be empty"));
        }
        if task.text != new_text {
            task.text = new_text.clone();
            changes.push(format!("text → {}", new_text.bright_white()));
        }
    }

    if let Some(new_priority) = args.priority
        && task.priority != new_priority
    {
        task.priority = new_priority;
        changes.push(format!("priority → {}", new_priority.letter()));
    }

    // ── project ───────────────────────────────────────────────────────────────
    if args.clear_project {
        if task.project_id.is_some() {
            task.project_id = None;
            changes.push("project → cleared".dimmed().to_string());
        }
    } else if let Some(ref new_project_name) = args.project {
        validation::validate_project_name(new_project_name)?;
        let projects = storage.load_projects()?;
        let new_uuid = Project::resolve_or_create(storage, &projects, new_project_name)?;
        if task.project_id != Some(new_uuid) {
            task.project_id = Some(new_uuid);
            changes.push(format!("project → {}", new_project_name.cyan()));
        }
    }

    // ── tags ──────────────────────────────────────────────────────────────────
    if args.clear_tags {
        if !task.tags.is_empty() {
            let old_tags = task.tags.clone();
            task.tags.clear();
            changes.push(format!(
                "tags cleared → was [{}]",
                old_tags.join(", ").dimmed()
            ));
        }
    } else {
        if !args.remove_tag.is_empty() {
            let before_len = task.tags.len();
            let mut removed = Vec::new();
            task.tags.retain(|t| {
                if args.remove_tag.contains(t) {
                    removed.push(t.clone());
                    false
                } else {
                    true
                }
            });
            if !removed.is_empty() {
                changes.push(format!("removed tags → [{}]", removed.join(", ").red()));
            } else if before_len > 0 {
                return Err(anyhow::anyhow!(
                    "None of the specified tags [{}] exist in task #{}",
                    args.remove_tag.join(", "),
                    args.id
                ));
            }
        }

        if !args.add_tag.is_empty() {
            validation::validate_tags(&args.add_tag)?;
            let mut added = Vec::new();
            for new_tag in &args.add_tag {
                if !task.tags.contains(new_tag) {
                    task.tags.push(new_tag.clone());
                    added.push(new_tag.clone());
                }
            }
            if !added.is_empty() {
                changes.push(format!("added tags → [{}]", added.join(", ").cyan()));
            }
        }
    }

    // ── due date ──────────────────────────────────────────────────────────────
    if args.clear_due {
        if task.due_date.is_some() {
            task.due_date = None;
            changes.push("due date → cleared".dimmed().to_string());
        }
    } else if let Some(new_due) = due
        && task.due_date != Some(new_due)
    {
        task.due_date = Some(new_due);
        changes.push(format!("due date → {}", new_due.to_string().cyan()));
    }

    // ── dependencies ──────────────────────────────────────────────────────────
    if args.clear_deps {
        if !task.depends_on.is_empty() {
            task.depends_on.clear();
            changes.push(format!(
                "dependencies cleared → was [{}]",
                current_deps_display.dimmed()
            ));
        }
    } else {
        if !args.remove_dep.is_empty() {
            task.depends_on.retain(|d| !remove_dep_uuids.contains(d));
            let removed = args
                .remove_dep
                .iter()
                .map(|id| format!("#{}", id))
                .collect::<Vec<_>>()
                .join(", ");
            changes.push(format!("removed deps → [{}]", removed.red()));
        }
        if !args.add_dep.is_empty() {
            for dep_uuid in &add_dep_uuids {
                task.depends_on.push(*dep_uuid);
            }
            let added = args
                .add_dep
                .iter()
                .map(|id| format!("#{}", id))
                .collect::<Vec<_>>()
                .join(", ");
            changes.push(format!("added deps → [{}]", added.cyan()));
        }
    }

    if changes.is_empty() {
        if !silent {
            println!(
                "{} No changes made (values are already set to the specified values).",
                "".blue()
            );
        }
        return Ok("No changes made.".to_string());
    }

    task.touch();
    storage.save(&tasks)?;

    if !silent {
        println!("{} Task #{} updated:", "✓".green(), args.id);
        for change in &changes {
            println!("  • {}", change);
        }
    }

    Ok(format!("Task #{} updated.", args.id))
}
