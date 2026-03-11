//! Handler for `todo next`.
//!
//! Shows the top N pending tasks sorted by urgency score, excluding tasks
//! blocked by unresolved dependencies — only "ready to work" tasks.
//!
//! Inspired by Taskwarrior's `task next` command.

use anyhow::Result;
use colored::Colorize;

use crate::models::Task;
use crate::render::next_table::display_next;
use crate::storage::Storage;

const DEFAULT_LIMIT: usize = 5;

pub fn execute(storage: &impl Storage, limit: Option<usize>) -> Result<()> {
    let (all_tasks, projects, _, _) = storage.load_all_with_resources()?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT);

    let pending: Vec<&Task> = all_tasks
        .iter()
        .filter(|t| !t.is_deleted() && !t.completed)
        .collect();

    let blocked_count = pending.iter().filter(|t| t.is_blocked(&all_tasks)).count();

    let ready_count = pending.len() - blocked_count;

    let mut ready: Vec<&Task> = pending
        .into_iter()
        .filter(|t| !t.is_blocked(&all_tasks))
        .collect();

    if ready.is_empty() {
        if blocked_count > 0 {
            println!(
                "\n{}\n",
                format!(
                    "All {} pending tasks are blocked by dependencies.",
                    blocked_count
                )
                .yellow()
            );
        } else {
            println!("\n{}\n", "No pending tasks found.".dimmed());
        }
        return Ok(());
    }

    ready.sort_by(|a, b| {
        b.urgency_score(&all_tasks)
            .partial_cmp(&a.urgency_score(&all_tasks))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Resolve display IDs (1-based position among non-deleted tasks)
    let shown: Vec<(&Task, usize)> = ready
        .into_iter()
        .take(limit)
        .map(|t| {
            let idx = all_tasks
                .iter()
                .filter(|x| !x.is_deleted())
                .position(|x| x.uuid == t.uuid)
                .map(|i| i + 1)
                .unwrap_or(0);
            (t, idx)
        })
        .collect();

    display_next(&shown, &all_tasks, &projects, ready_count, blocked_count);

    Ok(())
}
