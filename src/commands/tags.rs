//! Handler for `todo tags`.
//!
//! Collects all unique tags across the task list and prints each one with
//! the number of tasks it appears on.

use anyhow::Result;
use colored::Colorize;

use crate::error::TodoError;
use crate::storage::Storage;

pub fn execute(storage: &impl Storage) -> Result<()> {
    let tasks = storage.load()?;

    // Collect all unique tags from non-deleted tasks
    let mut all_tags: Vec<String> = Vec::new();
    for task in tasks.iter().filter(|t| !t.is_deleted()) {
        for tag in &task.tags {
            if !all_tags.contains(tag) {
                all_tags.push(tag.to_owned());
            }
        }
    }

    if all_tags.is_empty() {
        return Err(TodoError::NoTagsFound.into());
    }

    all_tags.sort();

    println!("\nTags:\n");
    for tag in &all_tags {
        let count = tasks
            .iter()
            .filter(|t| !t.is_deleted() && t.tags.contains(tag))
            .count();
        println!(
            "  {} ({} task{})",
            tag.cyan(),
            count,
            if count == 1 { "" } else { "s" }
        );
    }

    println!();
    Ok(())
}
