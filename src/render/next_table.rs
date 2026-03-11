//! Terminal rendering for `todo next`.
//!
//! Column order (Taskwarrior-style): ID  Age  P  Tags  Project  Due  Task  Urg
//! Fixed context columns on the left, content (Task) and score (Urg) on the right.

use chrono::Utc;
use colored::Colorize;

use crate::models::{Project, Task};

use super::formatting::{get_due_colored, get_due_text, project_colored, project_name, truncate};

const ID_WIDTH: usize = 4;
const AGE_WIDTH: usize = 5;
const PRIORITY_WIDTH: usize = 1;
const SCORE_WIDTH: usize = 5;

pub struct NextTableLayout {
    task: usize,
    tags: usize,
    project: usize,
    due: usize,
    show_tags: bool,
    show_project: bool,
    show_due: bool,
}

impl NextTableLayout {
    pub fn new(tasks: &[&Task], projects: &[Project]) -> Self {
        let mut max_task = 10usize;
        let mut max_tags = 4usize;
        let mut max_project = 7usize;
        let mut max_due = 3usize;

        for task in tasks {
            max_task = max_task.max(task.text.len());

            if !task.tags.is_empty() {
                max_tags = max_tags.max(task.tags.join(", ").len());
            }

            if let Some(pid) = task.project_id
                && let Some(p) = projects.iter().find(|p| p.uuid == pid && !p.is_deleted())
            {
                max_project = max_project.max(p.name.len());
            }

            let due_text = get_due_text(task);
            if !due_text.is_empty() {
                max_due = max_due.max(due_text.len());
            }
        }

        let show_tags = tasks.iter().any(|t| !t.tags.is_empty());
        let show_project = tasks.iter().any(|t| {
            t.project_id
                .and_then(|pid| projects.iter().find(|p| p.uuid == pid && !p.is_deleted()))
                .is_some()
        });
        let show_due = tasks.iter().any(|t| t.due_date.is_some());

        Self {
            task: max_task.min(40),
            tags: max_tags.min(20),
            project: max_project.min(24),
            due: max_due.min(20),
            show_tags,
            show_project,
            show_due,
        }
    }

    pub fn total_width(&self) -> usize {
        // ID(4) + 2 + Age(5) + 2 + P(1) + 2 + optional cols + 2 + Task + 2 + Urg(5)
        let mut width =
            ID_WIDTH + 2 + AGE_WIDTH + 2 + PRIORITY_WIDTH + 2 + self.task + 2 + SCORE_WIDTH;
        if self.show_tags {
            width += 2 + self.tags;
        }
        if self.show_project {
            width += 2 + self.project;
        }
        if self.show_due {
            width += 2 + self.due;
        }
        width
    }

    pub fn display_header(&self) {
        print!("{:>id$}  ", "ID".dimmed(), id = ID_WIDTH);
        print!("{:<age$}  ", "Age".dimmed(), age = AGE_WIDTH);
        print!("{:<p$}  ", "P".dimmed(), p = PRIORITY_WIDTH);
        if self.show_tags {
            print!("{:<t$}  ", "Tags".dimmed(), t = self.tags);
        }
        if self.show_project {
            print!("{:<p$}  ", "Project".dimmed(), p = self.project);
        }
        if self.show_due {
            print!("{:<d$}  ", "Due".dimmed(), d = self.due);
        }
        print!("{:<t$}  ", "Task".dimmed(), t = self.task);
        print!("{:>score$}", "Urg".dimmed(), score = SCORE_WIDTH);
        println!();
    }

    pub fn display_separator(&self) {
        println!("{}", "─".repeat(self.total_width()).dimmed());
    }

    pub fn display_row(&self, idx: usize, task: &Task, all_tasks: &[Task], projects: &[Project]) {
        let score = task.urgency_score(all_tasks);
        let score_str = format!("{:.1}", score);
        let score_colored = if score >= 10.0 {
            score_str.red()
        } else if score >= 6.0 {
            score_str.yellow()
        } else {
            score_str.normal()
        };

        let age_secs = (Utc::now() - task.created_at).num_seconds();
        let age_str = if age_secs < 3600 {
            format!("{}m", age_secs / 60)
        } else if age_secs < 86400 {
            format!("{}h", age_secs / 3600)
        } else if age_secs < 7 * 86400 {
            format!("{}d", age_secs / 86400)
        } else if age_secs < 30 * 86400 {
            format!("{}w", age_secs / (7 * 86400))
        } else if age_secs < 365 * 86400 {
            format!("{}mo", age_secs / (30 * 86400))
        } else {
            format!("{}y", age_secs / (365 * 86400))
        };

        let tags_str = if task.tags.is_empty() {
            "—".to_string()
        } else {
            truncate(&task.tags.join(", "), self.tags)
        };
        let tags_colored = if task.tags.is_empty() {
            tags_str.dimmed()
        } else {
            tags_str.cyan()
        };

        let name = project_name(task.project_id, projects);
        let proj_str = truncate(name, self.project);
        let proj_colored = project_colored(&proj_str);

        let due_text = get_due_text(task);
        let due_colored = get_due_colored(task, &due_text);

        print!("{:>id$}  ", format!("#{}", idx).dimmed(), id = ID_WIDTH);
        print!("{:<age$}  ", age_str.dimmed(), age = AGE_WIDTH);
        print!("{:<p$}  ", task.priority.letter(), p = PRIORITY_WIDTH);
        if self.show_tags {
            print!("{:<t$}  ", tags_colored, t = self.tags);
        }
        if self.show_project {
            print!("{:<p$}  ", proj_colored, p = self.project);
        }
        if self.show_due {
            print!("{:<d$}  ", due_colored, d = self.due);
        }
        print!(
            "{:<t$}  ",
            truncate(&task.text, self.task).bright_white(),
            t = self.task
        );
        print!("{:>score$}", score_colored, score = SCORE_WIDTH);
        println!();
    }
}

/// Renders the next/urgency table to stdout.
pub fn display_next(
    tasks: &[(&Task, usize)],
    all_tasks: &[Task],
    projects: &[Project],
    ready_count: usize,
    blocked_count: usize,
) {
    println!("\nNext tasks  (by urgency):\n");

    let task_refs: Vec<&Task> = tasks.iter().map(|(t, _)| *t).collect();
    let layout = NextTableLayout::new(&task_refs, projects);

    layout.display_header();
    layout.display_separator();

    for (task, idx) in tasks {
        layout.display_row(*idx, task, all_tasks, projects);
    }

    layout.display_separator();

    let footer = if blocked_count > 0 {
        format!(
            "Showing {} of {} ready · {} blocked by deps",
            tasks.len(),
            ready_count,
            blocked_count,
        )
    } else {
        format!("Showing {} of {} ready", tasks.len(), ready_count)
    };
    println!("{}\n", footer.dimmed());
}
