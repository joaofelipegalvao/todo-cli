//! Handler for `todo done <ID>`.
//!
//! Marks a task as completed. If the task has pending dependencies it is
//! rejected with a [`TodoError::TaskBlocked`]
//! error. For recurring tasks a new occurrence is automatically created via
//! [`Task::create_next_recurrence`](crate::models::Task::create_next_recurrence).

use anyhow::Result;
use colored::Colorize;

use crate::error::TodoError;
use crate::storage::Storage;
use crate::utils::validation::resolve_visible_index;

/// Marks task `id` as done, creating the next recurrence if needed.
pub fn execute(storage: &impl Storage, id: usize) -> Result<()> {
    execute_inner(storage, id, false)?;
    Ok(())
}

/// TUI variant: same logic, no stdout, returns a status string.
pub fn execute_silent(storage: &impl Storage, id: usize) -> Result<String> {
    execute_inner(storage, id, true)
}

fn execute_inner(storage: &impl Storage, id: usize, silent: bool) -> Result<String> {
    let mut tasks = storage.load()?;

    let index = resolve_visible_index(&tasks, id, |t| t.is_deleted())
        .map_err(|_| anyhow::anyhow!("invalid task ID: {}", id))?;

    if tasks[index].completed {
        return Err(TodoError::TaskAlreadyInStatus {
            id,
            status: "completed".to_owned(),
        }
        .into());
    }

    let blocking = tasks[index].blocking_deps(&tasks);
    if !blocking.is_empty() {
        let vis: Vec<usize> = tasks
            .iter()
            .enumerate()
            .filter(|(_, t)| !t.is_deleted())
            .map(|(i, _)| i)
            .collect();

        let ids = blocking
            .iter()
            .filter_map(|uuid| {
                let real_pos = tasks.iter().position(|t| t.uuid == *uuid)?;
                let vis_id = vis.iter().position(|&i| i == real_pos).map(|p| p + 1)?;
                let text = tasks[real_pos].text.clone();
                Some(format!("#{} \"{}\"", vis_id, text))
            })
            .collect::<Vec<_>>()
            .join(", ");

        return Err(TodoError::TaskBlocked(id, ids).into());
    }

    tasks[index].mark_done();
    let task_uuid = tasks[index].uuid;

    if let Some(next_task) = tasks[index].create_next_recurrence(task_uuid) {
        let next_due = next_task.due_date.unwrap();

        let already_exists = tasks.iter().any(|t| {
            !t.completed
                && t.due_date == Some(next_due)
                && (t.parent_id == Some(task_uuid) || t.text == next_task.text)
        });

        if !already_exists {
            tasks.push(next_task);
            storage.save(&tasks)?;

            let next_vis_id = tasks.iter().filter(|t| !t.is_deleted()).count();

            let msg = format!(
                "Task #{} marked as done. Next recurrence: #{} (due {})",
                id,
                next_vis_id,
                next_due.format("%Y-%m-%d")
            );

            if !silent {
                let id_colored = format!("#{}", id).green();
                let next_colored = format!("#{}", next_vis_id).yellow();

                println!("Task {} marked as done.", id_colored);
                println!(
                    "Task {} created (due {})",
                    next_colored,
                    next_due.format("%Y-%m-%d")
                );
            }

            Ok(msg)
        } else {
            storage.save(&tasks)?;

            if !silent {
                let id_colored = format!("#{}", id).green();
                println!("Task {} marked as done.", id_colored);
                println!(
                    "{}",
                    "Next recurrence already exists, skipping creation.".dimmed()
                );
            }

            Ok(format!("Task #{} marked as done.", id))
        }
    } else {
        storage.save(&tasks)?;

        if !silent {
            let id_colored = format!("#{}", id).green();
            println!("Task {} marked as done.", id_colored);
        }

        Ok(format!("Task #{} marked as done.", id))
    }
}
