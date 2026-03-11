//! Handler for `todo stats history`.
//!
//! Shows a monthly history chart of tasks created, completed, and deleted —
//! inspired by Taskwarrior's `ghistory` command.

use anyhow::Result;
use chrono::{Datelike, Duration, Local};
use colored::Colorize;

use crate::storage::Storage;

pub fn execute(storage: &impl Storage, months: usize) -> Result<()> {
    let (all_tasks, _, _, _) = storage.load_all_with_resources()?;

    if all_tasks.is_empty() {
        println!("{}", "\nNo data found.\n".dimmed());
        return Ok(());
    }

    let today = Local::now().naive_local().date();

    // Build list of (year, month) for the last `months` months
    let mut periods: Vec<(i32, u32)> = Vec::new();
    let mut cursor = today;
    for _ in 0..months {
        periods.push((cursor.year(), cursor.month()));
        // Go back one month
        cursor = cursor
            .with_day(1)
            .unwrap()
            .checked_sub_signed(Duration::days(1))
            .unwrap();
    }
    periods.reverse();

    // Count per month
    struct MonthStats {
        label: String,
        added: usize,
        completed: usize,
        deleted: usize,
    }

    let mut rows: Vec<MonthStats> = periods
        .iter()
        .map(|(year, month)| {
            let added = all_tasks
                .iter()
                .filter(|t| {
                    let d = t.created_at.date_naive();
                    d.year() == *year && d.month() == *month
                })
                .count();

            let completed = all_tasks
                .iter()
                .filter(|t| {
                    t.completed_at
                        .map(|d| d.year() == *year && d.month() == *month)
                        .unwrap_or(false)
                })
                .count();

            let deleted = all_tasks
                .iter()
                .filter(|t| {
                    t.deleted_at
                        .map(|d| d.year() == *year && d.month() == *month)
                        .unwrap_or(false)
                })
                .count();

            let label = format!("{} {:04}", month_abbr(*month), year);

            MonthStats {
                label,
                added,
                completed,
                deleted,
            }
        })
        .collect();

    // Remove leading empty months
    let first_non_empty = rows
        .iter()
        .position(|r| r.added > 0 || r.completed > 0 || r.deleted > 0);
    if let Some(idx) = first_non_empty {
        rows = rows.into_iter().skip(idx).collect();
    }

    if rows.is_empty() {
        println!("{}", "\nNo activity found.\n".dimmed());
        return Ok(());
    }

    let max_count = rows
        .iter()
        .map(|r| r.added.max(r.completed).max(r.deleted))
        .max()
        .unwrap_or(1)
        .max(1);

    let bar_width = 24usize;

    println!("\n{}\n", "Monthly History".bright_white().bold());
    println!(
        "  {:<10}  {:<bar_width$}  {}",
        "Month".dimmed(),
        "Added / Completed / Deleted".dimmed(),
        "Count".dimmed(),
        bar_width = bar_width + 2,
    );
    println!("{}", "─".repeat(bar_width + 32).dimmed());

    for row in &rows {
        // Composite bar: added (green), completed (yellow), deleted (red)
        let filled_a = (row.added * bar_width) / max_count;
        let filled_c = (row.completed * bar_width) / max_count;
        let filled_d = (row.deleted * bar_width) / max_count;
        let total_filled = (filled_a + filled_c + filled_d).min(bar_width);
        let empty = bar_width.saturating_sub(total_filled);

        let bar = format!(
            "{}{}{}{}",
            "█".repeat(filled_a).green(),
            "█".repeat(filled_c).yellow(),
            "█".repeat(filled_d).red(),
            "░".repeat(empty).dimmed(),
        );

        let detail = format!(
            "+{}  ✓{}  -{}",
            row.added.to_string().green(),
            row.completed.to_string().yellow(),
            row.deleted.to_string().red(),
        );

        println!("  {:<10}  {}  {}", row.label.dimmed(), bar, detail);
    }

    println!("{}", "─".repeat(bar_width + 32).dimmed());
    println!(
        "\n  {}  {}  {}  {}  {}  {}  {}\n",
        "█".green(),
        "Added".dimmed(),
        "█".yellow(),
        "Completed".dimmed(),
        "█".red(),
        "Deleted".dimmed(),
        format!("(last {} months)", months).dimmed(),
    );

    Ok(())
}

fn month_abbr(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "???",
    }
}
