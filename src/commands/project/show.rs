//! Handler for `todo project show`.

use anyhow::Result;
use colored::Colorize;

use crate::models::count_by_project;
use crate::render::formatting::truncate;
use crate::storage::Storage;

/// Returns a single-line preview of a note — title if set, otherwise first non-empty line.
fn note_preview(note: &crate::models::Note) -> String {
    if let Some(ref title) = note.title {
        return title.clone();
    }
    note.body
        .lines()
        .find(|l| !l.trim().is_empty())
        .map(|l| l.trim_start_matches('#').trim().to_string())
        .unwrap_or_default()
}

pub fn execute(storage: &impl Storage, id: usize) -> Result<()> {
    let (tasks, projects, notes) = storage.load_all()?;

    let mut visible_projects: Vec<_> = projects.iter().filter(|p| !p.is_deleted()).collect();
    visible_projects.sort_by(|a, b| a.name.cmp(&b.name));

    let project = visible_projects
        .get(id.saturating_sub(1))
        .ok_or_else(|| anyhow::anyhow!("Project #{} not found", id))?;

    let (total, done) = count_by_project(&tasks, project.uuid);

    let all_visible: Vec<_> = tasks.iter().filter(|t| !t.is_deleted()).cloned().collect();

    let visible_tasks: Vec<_> = all_visible
        .iter()
        .filter(|t| t.project_id == Some(project.uuid))
        .collect();

    let blocked = visible_tasks
        .iter()
        .filter(|t| !t.completed && t.is_blocked(&all_visible))
        .count();
    let pending = total - done - blocked;

    let visible_notes: Vec<_> = notes.iter().filter(|n| !n.is_deleted()).collect();
    let project_notes: Vec<(usize, _)> = visible_notes
        .iter()
        .enumerate()
        .filter(|(_, n)| n.project_id == Some(project.uuid))
        .map(|(i, n)| (i + 1, n))
        .collect();

    let status_label = if project.completed {
        "done".green()
    } else {
        "pending".yellow()
    };

    // ── Header ────────────────────────────────────────────────────────────────
    println!();
    println!(
        "  {}",
        format!("Project #{}: {}", id, project.name).bold().cyan()
    );
    println!("  {}", "─".repeat(50).dimmed());

    // ── Details ───────────────────────────────────────────────────────────────
    println!("  {}  {}", "Status".dimmed(), status_label);
    println!(
        "  {}  {}",
        "Difficulty".dimmed(),
        project.difficulty.label()
    );

    if !project.tech.is_empty() {
        println!(
            "  {}  {}",
            "Tech".dimmed(),
            project.tech.join(", ").yellow()
        );
    }

    if let Some(due) = project.due_date {
        let overdue = if project.is_overdue() {
            format!("  {}", "overdue".red())
        } else {
            String::new()
        };
        println!("  {}  {}{}", "Due".dimmed(), due, overdue);
    }

    if let Some(completed_at) = project.completed_at {
        println!("  {}  {}", "Completed".dimmed(), completed_at);
    }

    // ── Tasks ─────────────────────────────────────────────────────────────────
    println!();

    let mut summary_parts = Vec::new();
    if pending > 0 {
        summary_parts.push(format!("{} pending", pending.to_string().yellow()));
    }
    if done > 0 {
        summary_parts.push(format!("{} done", done.to_string().green()));
    }
    if blocked > 0 {
        summary_parts.push(format!("{} blocked", blocked.to_string().red()));
    }
    if summary_parts.is_empty() {
        summary_parts.push("no tasks".dimmed().to_string());
    }

    println!("  {}  {}", "Tasks".dimmed(), summary_parts.join("  "));

    if !visible_tasks.is_empty() {
        let all_vis_refs: Vec<_> = tasks.iter().filter(|t| !t.is_deleted()).collect();

        for task in &visible_tasks {
            let vis_id = all_vis_refs
                .iter()
                .position(|t| t.uuid == task.uuid)
                .map(|i| i + 1)
                .unwrap_or(0);

            let is_blocked = !task.completed && task.is_blocked(&all_visible);

            let status = if task.completed {
                "D".green()
            } else if is_blocked {
                "B".red()
            } else {
                "P".yellow()
            };

            let text = truncate(&task.text, 40);
            let text_colored = if task.completed {
                text.dimmed()
            } else if is_blocked {
                text.truecolor(150, 150, 150)
            } else {
                text.bright_white()
            };

            println!(
                "    {}  {}  {}",
                format!("#{}", vis_id).dimmed(),
                status,
                text_colored
            );
        }
    }

    // ── Notes ─────────────────────────────────────────────────────────────────
    if !project_notes.is_empty() {
        println!();
        println!(
            "  {}  {}",
            "Notes".dimmed(),
            format!(
                "{}{}",
                project_notes.len(),
                if project_notes.len() == 1 {
                    " note"
                } else {
                    " notes"
                }
            )
            .dimmed()
        );
        for (note_id, note) in &project_notes {
            let preview = note_preview(note);
            println!(
                "    {}  {}",
                format!("#{}", note_id).dimmed(),
                preview.bright_white()
            );
        }
    }

    println!();
    println!(
        "  {}  {}",
        "Created".dimmed(),
        project.created_at.format("%Y-%m-%d")
    );
    println!();

    Ok(())
}
