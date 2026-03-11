//! Terminal rendering for task lists.
//!
//! Column order (Taskwarrior-style): ID  P  S  R  Tags  Project  Due  Task
//! Fixed context columns on the left, content (Task) on the right.

use colored::Colorize;

use crate::models::{Project, Recurrence, Task};

use super::formatting::{get_due_colored, get_due_text, project_colored, project_name, truncate};

const ID_WIDTH: usize = 4;
const PRIORITY_WIDTH: usize = 1;
const STATUS_WIDTH: usize = 1;
const RECUR_WIDTH: usize = 1;

pub struct TableLayout<'a> {
    id: usize,
    priority: usize,
    status: usize,
    recur: usize,
    task: usize,
    project: usize,
    tags: usize,
    due: usize,
    show_recur: bool,
    show_project: bool,
    show_tags: bool,
    show_due: bool,
    show_notes: bool,
    show_resources: bool,
    all_tasks: &'a [Task],
    projects: &'a [Project],
    notes: &'a [crate::models::Note],
    resources: &'a [crate::models::Resource],
}

impl<'a> TableLayout<'a> {
    pub fn new(
        tasks: &[(usize, &Task)],
        all_tasks: &'a [Task],
        projects: &'a [Project],
        notes: &'a [crate::models::Note],
        resources: &'a [crate::models::Resource],
    ) -> Self {
        let (task_w, project_w, tags_w, due_w) = calculate_column_widths(tasks, projects);
        let show_recur = tasks.iter().any(|(_, t)| t.recurrence.is_some());
        let show_project = tasks.iter().any(|(_, t)| {
            t.project_id
                .and_then(|pid| projects.iter().find(|p| p.uuid == pid && !p.is_deleted()))
                .is_some()
        });
        let show_tags = tasks.iter().any(|(_, t)| !t.tags.is_empty());
        let show_due = tasks.iter().any(|(_, t)| t.due_date.is_some());
        let show_notes = tasks.iter().any(|(_, t)| {
            notes
                .iter()
                .any(|n| !n.is_deleted() && n.task_id == Some(t.uuid))
        });
        let show_resources = tasks.iter().any(|(_, t)| {
            notes.iter().any(|n| {
                !n.is_deleted()
                    && n.task_id == Some(t.uuid)
                    && n.resource_ids
                        .iter()
                        .any(|rid| resources.iter().any(|r| !r.is_deleted() && r.uuid == *rid))
            })
        });

        Self {
            id: ID_WIDTH,
            priority: PRIORITY_WIDTH,
            status: STATUS_WIDTH,
            recur: RECUR_WIDTH,
            task: task_w,
            project: project_w,
            tags: tags_w,
            due: due_w,
            show_recur,
            show_project,
            show_tags,
            show_due,
            show_notes,
            show_resources,
            all_tasks,
            projects,
            notes,
            resources,
        }
    }

    pub fn total_width(&self) -> usize {
        // ID + P + S + optional R + optional context cols + Task + optional Notes/Res
        let mut width = self.id + 2 + self.priority + 2 + self.status + 2 + self.task;
        if self.show_recur {
            width += self.recur + 2;
        }
        if self.show_tags {
            width += self.tags + 2;
        }
        if self.show_project {
            width += self.project + 2;
        }
        if self.show_due {
            width += self.due + 2;
        }
        if self.show_notes {
            width += 5 + 2;
        }
        if self.show_resources {
            width += 3 + 2;
        }
        width
    }

    pub fn display_header(&self) {
        print!("{:>id_width$}  ", "ID".dimmed(), id_width = self.id);
        print!("{:<p$}  ", "P".dimmed(), p = self.priority);
        print!("{:<s$}  ", "S".dimmed(), s = self.status);
        if self.show_recur {
            print!("{:<r$}  ", "R".dimmed(), r = self.recur);
        }
        if self.show_tags {
            print!("{:<t$}  ", "Tags".dimmed(), t = self.tags);
        }
        if self.show_project {
            print!("{:<p$}  ", "Project".dimmed(), p = self.project);
        }
        if self.show_due {
            print!("{:<d$}  ", "Due".dimmed(), d = self.due);
        }
        print!("{:<t$}", "Task".dimmed(), t = self.task);
        if self.show_notes {
            print!("  {:^5}", "Notes".dimmed());
        }
        if self.show_resources {
            print!("  {:^3}", "Res".dimmed());
        }
        println!();
    }

    pub fn display_separator(&self) {
        println!("{}", "─".repeat(self.total_width()).dimmed());
    }

    pub fn display_task(&self, number: usize, task: &Task) {
        let blocked = !task.completed && task.is_blocked(self.all_tasks);

        let status_letter = if blocked {
            "B".red()
        } else if task.completed {
            "D".green()
        } else {
            "P".yellow()
        };

        let letter = task.priority.letter();
        let task_text = truncate(&task.text, self.task);

        let name = project_name(task.project_id, self.projects);
        let project_str = truncate(name, self.project);

        let tags_str = if task.tags.is_empty() {
            "—".to_string()
        } else {
            truncate(&task.tags.join(", "), self.tags)
        };
        let due_text = get_due_text(task);
        let due_colored = get_due_colored(task, &due_text);

        let recur_indicator = match task.recurrence {
            Some(Recurrence::Daily) => "D".cyan(),
            Some(Recurrence::Weekly) => "W".cyan(),
            Some(Recurrence::Monthly) => "M".cyan(),
            None => " ".normal(),
        };

        let (text_colored, tags_colored, proj_colored) = if task.completed {
            (task_text.green(), tags_str.dimmed(), project_str.dimmed())
        } else if blocked {
            (
                task_text.truecolor(150, 150, 150),
                tags_str.dimmed(),
                project_str.dimmed(),
            )
        } else {
            let tags_c = if task.tags.is_empty() {
                tags_str.dimmed()
            } else {
                tags_str.cyan()
            };
            (
                task_text.bright_white(),
                tags_c,
                project_colored(&project_str),
            )
        };

        print!(
            "{:>id_width$}  ",
            format!("#{}", number).dimmed(),
            id_width = self.id
        );
        print!("{:<p$}  ", letter, p = self.priority);
        print!("{:<s$}  ", status_letter, s = self.status);
        if self.show_recur {
            print!("{:<r$}  ", recur_indicator, r = self.recur);
        }
        if self.show_tags {
            print!("{:<t$}  ", tags_colored, t = self.tags);
        }
        if self.show_project {
            print!("{:<p$}  ", proj_colored, p = self.project);
        }
        if self.show_due {
            print!("{:<d$}  ", due_colored, d = self.due);
        }
        print!("{:<t$}", text_colored, t = self.task);
        if self.show_notes {
            let count = self
                .notes
                .iter()
                .filter(|n| !n.is_deleted() && n.task_id == Some(task.uuid))
                .count();
            let notes_str = if count > 0 {
                format!("{:^5}", count).dimmed().to_string()
            } else {
                format!("{:^5}", "—").dimmed().to_string()
            };
            print!("  {}", notes_str);
        }
        if self.show_resources {
            let count = self
                .notes
                .iter()
                .filter(|n| !n.is_deleted() && n.task_id == Some(task.uuid))
                .flat_map(|n| n.resource_ids.iter())
                .filter(|rid| {
                    self.resources
                        .iter()
                        .any(|r| !r.is_deleted() && r.uuid == **rid)
                })
                .count();
            let res_str = if count > 0 {
                format!("{:^3}", count).dimmed().to_string()
            } else {
                format!("{:^3}", "—").dimmed().to_string()
            };
            print!("  {}", res_str);
        }
        println!();
    }
}

fn calculate_column_widths(
    tasks: &[(usize, &Task)],
    projects: &[Project],
) -> (usize, usize, usize, usize) {
    let mut max_task = 10;
    let mut max_project = 7;
    let mut max_tags = 4;
    let mut max_due = 3;

    for (_, task) in tasks {
        max_task = max_task.max(task.text.len());

        if let Some(pid) = task.project_id
            && let Some(p) = projects.iter().find(|p| p.uuid == pid && !p.is_deleted())
        {
            max_project = max_project.max(p.name.len());
        }

        if !task.tags.is_empty() {
            max_tags = max_tags.max(task.tags.join(", ").len());
        }
        let due_text = get_due_text(task);
        if !due_text.is_empty() {
            max_due = max_due.max(due_text.len());
        }
    }

    (
        max_task.min(40),
        max_project.min(24),
        max_tags.min(20),
        max_due.min(20),
    )
}

/// Renders a labeled task list table to stdout.
pub fn display_lists(
    tasks: &[(usize, &Task)],
    title: &str,
    all_tasks: &[Task],
    projects: &[Project],
    notes: &[crate::models::Note],
    resources: &[crate::models::Resource],
) {
    println!("\n{}:\n", title);

    let layout = TableLayout::new(tasks, all_tasks, projects, notes, resources);
    layout.display_header();
    layout.display_separator();

    let mut completed = 0;
    let total = tasks.len();

    for (number, task) in tasks {
        layout.display_task(*number, task);
        if task.completed {
            completed += 1;
        }
    }

    layout.display_separator();

    let percentage = if total > 0 {
        (completed as f32 / total as f32 * 100.0) as u32
    } else {
        0
    };
    let stats = format!("{} of {} completed ({}%)", completed, total, percentage);

    if percentage == 100 {
        println!("{}", stats.green().bold());
    } else if percentage >= 50 {
        println!("{}", stats.yellow());
    } else {
        println!("{}", stats.red());
    }

    println!();
}
