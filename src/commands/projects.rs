//! Handler for `todo projects` / `todo project list` / `todo project show`.

use anyhow::Result;
use colored::Colorize;

use crate::error::TodoError;
use crate::models::count_by_project;
use crate::storage::Storage;

pub fn execute(storage: &impl Storage) -> Result<()> {
    let (tasks, projects, notes) = storage.load_all()?;

    let mut visible: Vec<_> = projects.iter().filter(|p| !p.is_deleted()).collect();

    if visible.is_empty() {
        return Err(TodoError::NoProjectsFound.into());
    }

    visible.sort_by(|a, b| a.name.cmp(&b.name));

    println!("\nProjects:\n");
    for (i, project) in visible.iter().enumerate() {
        let id = i + 1;
        let (total, done) = count_by_project(&tasks, project.uuid);
        let pending = total - done;
        let note_count = notes
            .iter()
            .filter(|n| n.project_id == Some(project.uuid) && !n.is_deleted())
            .count();

        let status_label = if project.completed {
            "done".green()
        } else {
            "pending".yellow()
        };

        let tech_label = if project.tech.is_empty() {
            String::new()
        } else {
            format!("  [{}]", project.tech.join(", ").yellow())
        };

        let notes_label = if note_count > 0 {
            format!("  {} note(s)", note_count)
        } else {
            String::new()
        };

        let overdue = if project.is_overdue() {
            format!("  {}", "overdue".red())
        } else {
            String::new()
        };

        println!(
            "  {}  {} [{}] [{}]{}  ({} pending, {} done){}{}",
            format!("#{}", id).dimmed(),
            project.name.cyan(),
            status_label,
            project.difficulty.label().dimmed(),
            tech_label,
            pending,
            done,
            notes_label,
            overdue,
        );
    }

    println!();
    Ok(())
}

pub fn execute_show(storage: &impl Storage, id: usize) -> Result<()> {
    let (tasks, projects, notes) = storage.load_all()?;

    let mut visible: Vec<_> = projects.iter().filter(|p| !p.is_deleted()).collect();
    visible.sort_by(|a, b| a.name.cmp(&b.name));

    let project = visible
        .get(id.saturating_sub(1))
        .ok_or_else(|| anyhow::anyhow!("Project #{} not found", id))?;

    let (total, done) = count_by_project(&tasks, project.uuid);
    let pending = total - done;
    let project_notes: Vec<_> = notes
        .iter()
        .filter(|n| n.project_id == Some(project.uuid) && !n.is_deleted())
        .collect();

    let status_label = if project.completed {
        "done".green()
    } else {
        "pending".yellow()
    };

    println!();
    println!("  {}", project.name.bold().cyan());
    println!("  {}", "─".repeat(50).dimmed());
    println!("  {}  {}", "Status:".dimmed(), status_label);
    println!(
        "  {}  {}",
        "Difficulty:".dimmed(),
        project.difficulty.label()
    );

    if !project.tech.is_empty() {
        println!(
            "  {}  {}",
            "Tech:".dimmed(),
            project.tech.join(", ").yellow()
        );
    }

    if let Some(due) = project.due_date {
        let overdue = if project.is_overdue() {
            format!(" {}", "overdue".red())
        } else {
            String::new()
        };
        println!("  {}  {}{}", "Due:".dimmed(), due, overdue);
    }

    if let Some(completed_at) = project.completed_at {
        println!("  {}  {}", "Completed:".dimmed(), completed_at);
    }

    println!(
        "  {}  {} pending, {} done",
        "Tasks:".dimmed(),
        pending,
        done
    );

    if !project_notes.is_empty() {
        println!("  {}  {} note(s)", "Notes:".dimmed(), project_notes.len());
        for (i, note) in project_notes.iter().enumerate() {
            let label = note.title.as_deref().unwrap_or_else(|| {
                let b = note.body.as_str();
                if b.len() > 50 { &b[..50] } else { b }
            });
            println!("    {}  {}", format!("#{}", i + 1).dimmed(), label);
        }
    }

    println!("  {}  {}", "Created:".dimmed(), project.created_at);
    println!();

    Ok(())
}
