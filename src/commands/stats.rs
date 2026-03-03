//! Handler for `todo stats`.
//!
//! Computes and prints:
//! - Overview counts (total, completed, pending, overdue, blocked)
//! - Breakdown by priority
//! - Breakdown by project
//! - A bar chart of completions over the last 7 days

use anyhow::Result;
use chrono::{Duration, Local};
use colored::Colorize;

use crate::models::count_by_project;
use crate::storage::Storage;

pub fn execute(storage: &impl Storage) -> Result<()> {
    let tasks = storage.load()?;

    // Work only with non-deleted tasks for all stats
    let tasks: Vec<_> = tasks.into_iter().filter(|t| !t.is_deleted()).collect();

    if tasks.is_empty() {
        println!("{}", "\nNo tasks found.\n".dimmed());
        return Ok(());
    }

    let today = Local::now().naive_local().date();

    let total = tasks.len();
    let completed = tasks.iter().filter(|t| t.completed).count();
    let pending = total - completed;
    let overdue = tasks.iter().filter(|t| t.is_overdue()).count();
    let due_soon = tasks.iter().filter(|t| t.is_due_soon(7)).count();
    let blocked = tasks
        .iter()
        .filter(|t| !t.completed && t.is_blocked(&tasks))
        .count();
    let pct = percent(completed, total);

    // === Header ===
    println!("\n{}\n", "Todo Statistics".bright_white().bold());

    // === Overview ===
    section("Overview");
    stat_line("Total tasks", &total.to_string(), None);
    stat_line(
        "Completed",
        &format!("{} ({}%)", completed, pct),
        Some(completion_color(pct)),
    );
    stat_line("Pending", &pending.to_string(), None);
    if overdue > 0 {
        stat_line("Overdue", &overdue.to_string(), Some("red"));
    }
    if due_soon > 0 {
        stat_line("Due soon", &due_soon.to_string(), Some("yellow"));
    }
    if blocked > 0 {
        stat_line("Blocked", &blocked.to_string(), Some("yellow"));
    }
    println!();

    // === By Priority ===
    section("By Priority");
    for (label, variant) in &[("High", "high"), ("Medium", "medium"), ("Low", "low")] {
        let t: Vec<_> = tasks
            .iter()
            .filter(|t| format!("{:?}", t.priority).to_lowercase() == *variant)
            .collect();
        if !t.is_empty() {
            let d = t.iter().filter(|t| t.completed).count();
            let p = t.len() - d;
            println!(
                "  {:<8} {}  ({} pending, {} done)",
                label.bright_white(),
                t.len().to_string().cyan(),
                p,
                d,
            );
        }
    }
    println!();

    // === By Project ===
    let mut projects: Vec<String> = tasks
        .iter()
        .filter_map(|t| t.project.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    projects.sort();

    if !projects.is_empty() {
        section("By Project");
        for project in &projects {
            let (total_p, done_p) = count_by_project(&tasks, project);
            let pct_p = percent(done_p, total_p);
            println!(
                "  {:<24} {}  ({}% done)",
                project.bright_white(),
                format!("{} tasks", total_p).cyan(),
                pct_p,
            );
        }
        let no_project = tasks.iter().filter(|t| t.project.is_none()).count();
        if no_project > 0 {
            println!(
                "  {:<24} {}",
                "(no project)".dimmed(),
                format!("{} tasks", no_project).dimmed(),
            );
        }
        println!();
    }

    // === Activity — last 7 days ===
    section("Activity — last 7 days");

    let max_bar = 10usize;

    let counts: Vec<(String, usize)> = (0..7)
        .rev()
        .map(|i| {
            let date = today - Duration::days(i);
            let count = tasks
                .iter()
                .filter(|t| t.completed_at == Some(date))
                .count();
            (date.format("%b %d").to_string(), count)
        })
        .collect();

    let max_count = counts.iter().map(|(_, c)| *c).max().unwrap_or(1).max(1);

    for (label, count) in &counts {
        let filled = (count * max_bar) / max_count;
        let empty = max_bar - filled;
        let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));
        let bar_colored = if *count == 0 {
            bar.dimmed()
        } else {
            bar.green()
        };
        let count_str = if *count == 0 {
            "  0 completed".dimmed().to_string()
        } else {
            format!("  {} completed", count).normal().to_string()
        };
        println!("  {}  {}  {}", label.dimmed(), bar_colored, count_str);
    }

    println!();
    Ok(())
}

fn section(title: &str) {
    println!("{}\n", title.bright_white().underline());
}

fn stat_line(label: &str, value: &str, color: Option<&str>) {
    let val = match color {
        Some("red") => value.red().to_string(),
        Some("yellow") => value.yellow().to_string(),
        Some("green") => value.green().to_string(),
        _ => value.cyan().to_string(),
    };
    println!("  {:<16} {}", label.dimmed(), val);
}

fn percent(part: usize, total: usize) -> usize {
    if total == 0 { 0 } else { (part * 100) / total }
}

fn completion_color(pct: usize) -> &'static str {
    if pct == 100 {
        "green"
    } else if pct >= 50 {
        "yellow"
    } else {
        "red"
    }
}
