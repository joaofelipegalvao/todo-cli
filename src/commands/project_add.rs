//! Handler for `todo project add`.

use anyhow::Result;
use colored::Colorize;

use crate::cli::ProjectAddArgs;
use crate::date_parser;
use crate::models::Project;
use crate::storage::Storage;

pub fn execute(storage: &impl Storage, args: ProjectAddArgs) -> Result<()> {
    let (_, mut projects, _) = storage.load_all()?;

    if projects
        .iter()
        .any(|p| p.name.to_lowercase() == args.name.to_lowercase() && !p.is_deleted())
    {
        return Err(anyhow::anyhow!("Project \"{}\" already exists", args.name));
    }

    let due = if let Some(ref due_str) = args.due {
        Some(date_parser::parse_date_not_in_past(due_str)?)
    } else {
        None
    };

    let mut project = Project::new(args.name.clone());

    if let Some(difficulty) = args.difficulty {
        project.difficulty = difficulty;
    }
    if !args.tech.is_empty() {
        project.tech = args
            .tech
            .into_iter()
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect();
    }
    if let Some(d) = due {
        project.due_date = Some(d);
    }

    // Use the visible count for the displayed ID so it matches what the user
    // sees in `project list`, regardless of how many soft-deleted projects
    // exist in storage.
    let visible_id = projects.iter().filter(|p| !p.is_deleted()).count() + 1;
    projects.push(project);
    storage.save_projects(&projects)?;

    println!(
        "{} Added project #{}: {}",
        "✓".green(),
        visible_id,
        args.name.cyan()
    );
    Ok(())
}
