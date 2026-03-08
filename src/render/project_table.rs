//! Terminal rendering for project lists.

use chrono::Local;
use colored::Colorize;

use crate::models::{Difficulty, Note, Project, Task, count_by_project};
use crate::render::formatting::{due_relative_text, truncate};

pub struct ProjectTableLayout {
    pub name_w: usize,
    pub tech_w: usize,
    pub show_tech: bool,
    pub show_notes: bool,
    pub show_tasks: bool,
    pub show_due: bool,
    pub total_w: usize,
}

impl ProjectTableLayout {
    pub fn new(projects: &[&Project], tasks: &[Task], notes: &[Note]) -> Self {
        let name_w = projects
            .iter()
            .map(|p| p.name.len())
            .max()
            .unwrap_or(7)
            .clamp(7, 32);

        let tech_w = projects
            .iter()
            .map(|p| {
                if p.tech.is_empty() {
                    0
                } else {
                    p.tech.join(", ").len()
                }
            })
            .max()
            .unwrap_or(0)
            .clamp(0, 24);

        let show_tech = projects.iter().any(|p| !p.tech.is_empty());
        let show_notes = projects.iter().any(|p| {
            notes
                .iter()
                .any(|n| !n.is_deleted() && n.project_id == Some(p.uuid))
        });
        let show_tasks = projects.iter().any(|p| {
            tasks
                .iter()
                .any(|t| !t.is_deleted() && t.project_id == Some(p.uuid))
        });
        let show_due = projects.iter().any(|p| p.due_date.is_some());

        // ID(4) + S(1) + D(1) + name + gaps
        let mut total_w = 4 + 2 + 1 + 2 + 1 + 2 + name_w;
        if show_tech {
            total_w += tech_w + 2;
        }
        if show_notes {
            total_w += 5 + 2;
        }
        if show_tasks {
            total_w += 5 + 2;
        }
        if show_due {
            total_w += 10 + 2;
        }

        Self {
            name_w,
            tech_w,
            show_tech,
            show_notes,
            show_tasks,
            show_due,
            total_w,
        }
    }

    pub fn display_header(&self) {
        print!("{:>4}  ", "ID".dimmed());
        print!("{:<1}  ", "S".dimmed());
        print!("{:<1}  ", "D".dimmed());
        print!("{:<name_w$}  ", "Project".dimmed(), name_w = self.name_w);
        if self.show_tech {
            print!("  {:<tech_w$}", "Tech".dimmed(), tech_w = self.tech_w);
        }
        if self.show_notes {
            print!("  {:^5}", "Notes".dimmed());
        }
        if self.show_tasks {
            print!("  {:^5}", "Tasks".dimmed());
        }
        if self.show_due {
            print!("  {:<10}", "Due".dimmed());
        }
        println!();
        println!("{}", "─".repeat(self.total_w).dimmed());
    }

    pub fn display_row(&self, id: usize, project: &Project, tasks: &[Task], notes: &[Note]) {
        let (total, done) = count_by_project(tasks, project.uuid);

        let note_count = notes
            .iter()
            .filter(|n| n.project_id == Some(project.uuid) && !n.is_deleted())
            .count();

        let status_letter = if project.completed {
            "D".green()
        } else {
            "P".yellow()
        };

        let diff_letter = match project.difficulty {
            Difficulty::Easy => "E".green(),
            Difficulty::Medium => "M".yellow(),
            Difficulty::Hard => "H".red(),
        };

        let name_padded = format!(
            "{:<name_w$}",
            truncate(&project.name, self.name_w),
            name_w = self.name_w
        );
        let name_colored = if project.completed {
            name_padded.dimmed()
        } else {
            name_padded.magenta()
        };

        let tasks_str = if total == 0 {
            format!("{:^5}", "—").dimmed().to_string()
        } else {
            let ratio = done as f32 / total as f32;
            let text = format!("{:^5}", format!("{}/{}", done, total));
            if ratio == 1.0 {
                text.green().to_string()
            } else if ratio >= 0.5 {
                text.yellow().to_string()
            } else {
                text.dimmed().to_string()
            }
        };

        let due_str = project
            .due_date
            .map(|d| {
                let text = format!("{:<10}", due_relative_text(d));
                if project.is_overdue() {
                    text.red().to_string()
                } else {
                    let today = Local::now().naive_local().date();
                    let days = (d - today).num_days();
                    if days == 0 {
                        text.yellow().bold().to_string()
                    } else if days <= 7 {
                        text.yellow().to_string()
                    } else {
                        text.cyan().to_string()
                    }
                }
            })
            .unwrap_or_else(|| format!("{:<10}", "—").dimmed().to_string());

        print!("{:>4}  ", format!("#{}", id).dimmed());
        print!("{:<1}  ", status_letter);
        print!("{:<1}  ", diff_letter);
        print!("{}  ", name_colored);

        if self.show_tech {
            let tech_str = if project.tech.is_empty() {
                format!("{:<tech_w$}", "—", tech_w = self.tech_w)
                    .dimmed()
                    .to_string()
            } else {
                format!(
                    "{:<tech_w$}",
                    truncate(&project.tech.join(", "), self.tech_w),
                    tech_w = self.tech_w
                )
                .yellow()
                .to_string()
            };
            print!("  {}", tech_str);
        }
        if self.show_notes {
            let notes_str = if note_count > 0 {
                format!("{:^5}", note_count).dimmed().to_string()
            } else {
                format!("{:^5}", "—").dimmed().to_string()
            };
            print!("  {}", notes_str);
        }
        if self.show_tasks {
            print!("  {}", tasks_str);
        }
        if self.show_due {
            print!("  {}", due_str);
        }
        println!();
    }

    pub fn display_separator(&self) {
        println!("{}", "─".repeat(self.total_w).dimmed());
    }
}

pub fn display_projects(projects: &[&Project], tasks: &[Task], notes: &[Note]) {
    println!("\nProjects:\n");
    let layout = ProjectTableLayout::new(projects, tasks, notes);
    layout.display_header();
    for (i, project) in projects.iter().enumerate() {
        layout.display_row(i + 1, project, tasks, notes);
    }
    layout.display_separator();
    println!();
}
