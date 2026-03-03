//! Handler for `todo projects`.
//!
//! Collects all unique project names across the task list and prints each one
//! with its pending and done task counts.

use anyhow::Result;
use colored::Colorize;

use crate::error::TodoError;
use crate::models::count_by_project;
use crate::storage::Storage;

pub fn execute(storage: &impl Storage) -> Result<()> {
    let tasks = storage.load()?;

    let mut projects: Vec<String> = tasks
        .iter()
        .filter(|t| !t.is_deleted())
        .filter_map(|t| t.project.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    if projects.is_empty() {
        return Err(TodoError::NoProjectsFound.into());
    }

    projects.sort();

    println!("\nProjects:\n");
    for project in &projects {
        let (total, done) = count_by_project(&tasks, project);
        let pending = total - done;
        println!("  {} ({} pending, {} done)", project.cyan(), pending, done,);
    }

    println!();
    Ok(())
}
