// src/commands/stats.rs

//! Handler for `todo stats`.

use anyhow::Result;
use chrono::{Duration, Local};
use colored::Colorize;

use crate::models::{Priority, count_by_project};
use crate::storage::Storage;

pub fn execute(storage: &impl Storage) -> Result<()> {
    let (all_tasks, projects, all_notes, all_resources) = storage.load_all_with_resources()?;

    let tasks: Vec<_> = all_tasks.into_iter().filter(|t| !t.is_deleted()).collect();
    let notes: Vec<_> = all_notes.into_iter().filter(|n| !n.is_deleted()).collect();
    let resources: Vec<_> = all_resources
        .into_iter()
        .filter(|r| !r.is_deleted())
        .collect();

    if tasks.is_empty() && notes.is_empty() && resources.is_empty() {
        println!("{}", "\nNo data found.\n".dimmed());
        return Ok(());
    }

    let today = Local::now().naive_local().date();

    // ── Task metrics ──────────────────────────────────────────────────────────
    let total = tasks.len();
    let completed = tasks.iter().filter(|t| t.completed).count();
    let pending = total - completed;
    let overdue = tasks.iter().filter(|t| t.is_overdue()).count();
    let due_soon = tasks.iter().filter(|t| t.is_due_soon(7)).count();
    let blocked = tasks
        .iter()
        .filter(|t| !t.completed && t.is_blocked(&tasks))
        .count();
    let recurring = tasks.iter().filter(|t| t.recurrence.is_some()).count();
    let with_deps = tasks.iter().filter(|t| !t.depends_on.is_empty()).count();
    let pct = percent(completed, total);

    // ── Tag metrics ───────────────────────────────────────────────────────────
    let mut tag_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for task in &tasks {
        for tag in &task.tags {
            *tag_counts.entry(tag.clone()).or_insert(0) += 1;
        }
    }
    for note in &notes {
        for tag in &note.tags {
            *tag_counts.entry(tag.clone()).or_insert(0) += 1;
        }
    }
    for resource in &resources {
        for tag in &resource.tags {
            *tag_counts.entry(tag.clone()).or_insert(0) += 1;
        }
    }
    let mut top_tags: Vec<(String, usize)> = tag_counts.into_iter().collect();
    top_tags.sort_by(|a, b| b.1.cmp(&a.1));

    // ── Urgency buckets ───────────────────────────────────────────────────────
    let urgent: Vec<_> = tasks
        .iter()
        .filter(|t| !t.completed && t.urgency_score(&tasks) >= 10.0)
        .collect();

    println!("\n{}\n", "Todo Statistics".bright_white().bold());

    // ── Overview ──────────────────────────────────────────────────────────────
    section("Overview");
    if total > 0 {
        stat_line("Tasks", &format!("{} total", total), None);
        stat_line(
            "Completed",
            &format!("{} ({}%)", completed, pct),
            Some(completion_color(pct)),
        );
        stat_line("Pending", &pending.to_string(), None);
        stat_line(
            "Overdue",
            &overdue.to_string(),
            Some(if overdue > 0 { "red" } else { "none" }),
        );
        if due_soon > 0 {
            stat_line("Due soon", &due_soon.to_string(), Some("yellow"));
        }
        let no_due = tasks
            .iter()
            .filter(|t| !t.completed && t.due_date.is_none())
            .count();
        if no_due > 0 {
            stat_line("No due date", &no_due.to_string(), None);
        }
        if blocked > 0 {
            stat_line("Blocked", &blocked.to_string(), Some("yellow"));
        }
        if recurring > 0 {
            stat_line("Recurring", &recurring.to_string(), None);
        }
        if with_deps > 0 {
            stat_line("With deps", &with_deps.to_string(), None);
        }
    }
    if !notes.is_empty() {
        let orphan_notes = notes
            .iter()
            .filter(|n| n.project_id.is_none() && n.task_id.is_none())
            .count();
        let linked_notes = notes.len() - orphan_notes;
        stat_line(
            "Notes",
            &format!(
                "{} total  ({} linked, {} orphan)",
                notes.len(),
                linked_notes,
                orphan_notes
            ),
            None,
        );
    }
    if !resources.is_empty() {
        let linked_resource_uuids: std::collections::HashSet<uuid::Uuid> = notes
            .iter()
            .flat_map(|n| n.resource_ids.iter().copied())
            .collect();
        let linked_res = resources
            .iter()
            .filter(|r| linked_resource_uuids.contains(&r.uuid))
            .count();
        let orphan_res = resources.len() - linked_res;
        stat_line(
            "Resources",
            &format!(
                "{} total  ({} linked, {} orphan)",
                resources.len(),
                linked_res,
                orphan_res
            ),
            None,
        );
    }
    println!();

    // ── By Priority ───────────────────────────────────────────────────────────
    if total > 0 {
        section("By Priority");
        for (label, priority) in &[
            ("High", Priority::High),
            ("Medium", Priority::Medium),
            ("Low", Priority::Low),
        ] {
            let t: Vec<_> = tasks.iter().filter(|t| t.priority == *priority).collect();
            if !t.is_empty() {
                let d = t.iter().filter(|t| t.completed).count();
                let p = t.len() - d;
                let bar = progress_bar(d, t.len(), 10);
                println!(
                    "  {:<8} {}  {}  ({} pending, {} done)",
                    label.bright_white(),
                    t.len().to_string().cyan(),
                    bar,
                    p,
                    d,
                );
            }
        }
        println!();
    }

    // ── By Project ────────────────────────────────────────────────────────────
    let visible_projects: Vec<_> = projects.iter().filter(|p| !p.is_deleted()).collect();
    if !visible_projects.is_empty() {
        section("By Project");
        for project in &visible_projects {
            let (total_p, done_p) = count_by_project(&tasks, project.uuid);
            let note_count = notes
                .iter()
                .filter(|n| n.project_id == Some(project.uuid))
                .count();
            let note_str = if note_count > 0 {
                format!("  {} {}", note_count.to_string().dimmed(), "notes".dimmed())
            } else {
                String::new()
            };
            if total_p == 0 {
                println!(
                    "  {:<24} {}{}",
                    project.name.bright_white(),
                    "no tasks".dimmed(),
                    note_str,
                );
            } else {
                let pct_p = percent(done_p, total_p);
                let bar = progress_bar(done_p, total_p, 10);
                let task_str = pluralize(total_p, "task");
                println!(
                    "  {:<24} {:<10}  {}  {}%{}",
                    project.name.bright_white(),
                    task_str.cyan(),
                    bar,
                    pct_p,
                    note_str,
                );
            }
        }
        let no_project = tasks.iter().filter(|t| t.project_id.is_none()).count();
        if no_project > 0 {
            println!(
                "  {:<24} {}",
                "(no project)".dimmed(),
                pluralize(no_project, "task").dimmed(),
            );
        }
        println!();
    }

    // ── Urgent tasks ──────────────────────────────────────────────────────────
    if !urgent.is_empty() {
        section("Urgent  (score >= 10)");
        let all_vis: Vec<_> = tasks.iter().collect();
        let mut urgent_sorted: Vec<_> = urgent.iter().collect();
        urgent_sorted.sort_by(|a, b| {
            b.urgency_score(&tasks)
                .partial_cmp(&a.urgency_score(&tasks))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        for task in urgent_sorted.iter().take(5) {
            let vis_id = all_vis
                .iter()
                .position(|t| t.uuid == task.uuid)
                .map(|i| i + 1)
                .unwrap_or(0);
            let score = task.urgency_score(&tasks);
            let score_str = format!("{:.1}", score).red().bold();
            println!(
                "  {}  {}  {}",
                format!("#{}", vis_id).dimmed(),
                score_str,
                task.text.bright_white(),
            );
        }
        println!();
    }

    // ── Top Tags ──────────────────────────────────────────────────────────────
    if !top_tags.is_empty() {
        section("Top Tags");
        for (tag, count) in top_tags.iter().take(8) {
            println!(
                "  {:<20}  {}",
                format!("#{}", tag).cyan(),
                count.to_string().dimmed(),
            );
        }
        println!();
    }

    // ── Completion rate ───────────────────────────────────────────────────────
    section("Completion Rate");
    let rate_7 = {
        let cutoff = today - Duration::days(7);
        let done = tasks
            .iter()
            .filter(|t| t.completed_at.map(|d| d >= cutoff).unwrap_or(false))
            .count();
        let created = tasks
            .iter()
            .filter(|t| t.created_at.date_naive() >= cutoff)
            .count();
        percent(done, created.max(1))
    };
    let rate_30 = {
        let cutoff = today - Duration::days(30);
        let done = tasks
            .iter()
            .filter(|t| t.completed_at.map(|d| d >= cutoff).unwrap_or(false))
            .count();
        let created = tasks
            .iter()
            .filter(|t| t.created_at.date_naive() >= cutoff)
            .count();
        percent(done, created.max(1))
    };
    stat_line(
        "7 days",
        &format!("{}%", rate_7),
        Some(completion_color(rate_7)),
    );
    stat_line(
        "30 days",
        &format!("{}%", rate_30),
        Some(completion_color(rate_30)),
    );
    println!();

    Ok(())
}

// ── Helpers ───────────────────────────────────────────────────────────────────

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

fn progress_bar(done: usize, total: usize, width: usize) -> String {
    let filled = if total == 0 {
        0
    } else {
        (done * width) / total
    };
    let empty = width - filled;
    let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));
    if done == total && total > 0 {
        bar.green().to_string()
    } else if filled > 0 {
        bar.yellow().to_string()
    } else {
        bar.dimmed().to_string()
    }
}

fn pluralize(count: usize, word: &str) -> String {
    if count == 1 {
        format!("{} {}", count, word)
    } else {
        format!("{} {}s", count, word)
    }
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
