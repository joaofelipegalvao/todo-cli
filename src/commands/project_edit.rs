//! Handler for `todo project edit <ID>`.

use anyhow::Result;
use colored::Colorize;

use crate::cli::ProjectEditArgs;
use crate::date_parser;
use crate::storage::Storage;
use crate::validation::{validate_task_id, visible_indices};

pub fn execute(storage: &impl Storage, args: ProjectEditArgs) -> Result<()> {
    let (_, mut projects, _) = storage.load_all()?;

    let vis = visible_indices(&projects, |p| p.is_deleted());
    validate_task_id(args.id, vis.len())?;
    let real_index = vis[args.id - 1];

    let due = if let Some(ref due_str) = args.due {
        Some(date_parser::parse_date_not_in_past(due_str)?)
    } else {
        None
    };

    let project = &mut projects[real_index];
    let mut changes = Vec::new();

    if let Some(ref new_name) = args.name {
        if new_name.trim().is_empty() {
            return Err(anyhow::anyhow!("Project name cannot be empty"));
        }
        if &project.name != new_name {
            project.name = new_name.clone();
            changes.push(format!("name → {}", new_name.bright_white()));
        }
    }

    if args.done && !project.completed {
        project.mark_done();
        changes.push(format!("status → {}", "done".green()));
    } else if args.undone && project.completed {
        project.mark_undone();
        changes.push(format!("status → {}", "pending".yellow()));
    }

    if let Some(new_diff) = args.difficulty
        && project.difficulty != new_diff
    {
        project.difficulty = new_diff;
        changes.push(format!("difficulty → {}", new_diff.label().yellow()));
    }

    if args.clear_tech {
        if !project.tech.is_empty() {
            let old = project.tech.clone();
            project.tech.clear();
            changes.push(format!("tech cleared → was [{}]", old.join(", ").dimmed()));
        }
    } else {
        if !args.remove_tech.is_empty() {
            let remove_normalized: Vec<String> = args
                .remove_tech
                .iter()
                .map(|t| t.trim().to_lowercase())
                .collect();
            let mut removed = Vec::new();
            project.tech.retain(|t| {
                if remove_normalized.contains(&t.to_lowercase()) {
                    removed.push(t.clone());
                    false
                } else {
                    true
                }
            });
            if !removed.is_empty() {
                changes.push(format!("removed tech → [{}]", removed.join(", ").red()));
            }
        }
        if !args.add_tech.is_empty() {
            let mut added = Vec::new();
            for tech in &args.add_tech {
                let tech = tech.trim().to_string();
                if tech.is_empty() {
                    continue;
                }
                if !project
                    .tech
                    .iter()
                    .any(|t| t.to_lowercase() == tech.to_lowercase())
                {
                    project.tech.push(tech.clone());
                    added.push(tech);
                }
            }
            if !added.is_empty() {
                changes.push(format!("added tech → [{}]", added.join(", ").cyan()));
            }
        }
    }

    if args.clear_due {
        if project.due_date.is_some() {
            project.due_date = None;
            changes.push("due date → cleared".dimmed().to_string());
        }
    } else if let Some(new_due) = due
        && project.due_date != Some(new_due)
    {
        project.due_date = Some(new_due);
        changes.push(format!("due date → {}", new_due.to_string().cyan()));
    }

    if changes.is_empty() {
        println!(
            "{} No changes made (values are already set to the specified values).",
            "".blue()
        );
        return Ok(());
    }

    projects[real_index].touch();
    storage.save_projects(&projects)?;

    println!("{} Project #{} updated:", "✓".green(), args.id);
    for change in &changes {
        println!("  • {}", change);
    }

    Ok(())
}
