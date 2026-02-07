use std::{fs, process};

use anyhow::{Context, Result};
use chrono::{Local, NaiveDate};
use clap::{Args, Parser, Subcommand, ValueEnum};
use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Task ID {id} is invalid (valid range: 1-{max})")]
    InvalidTaskId { id: usize, max: usize },

    #[error("Task #{id} is already marked as {status}")]
    TaskAlreadyInStatus { id: usize, status: String },

    #[error("Tag '{0}' not found in any task")]
    TagNotFound(String),

    #[error("No tasks found matching the specified filters")]
    NoTasksFound,

    #[error("No tags found in any task")]
    NoTagsFound,

    #[error("Search returned no results for query: '{0}'")]
    NoSearchResults(String),
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("{} {}", "Error:".red().bold(), format!("{}", e).red());

        let mut source = e.source();
        while let Some(cause) = source {
            eprintln!("{} {}", "Caused by:".red(), cause);
            source = cause.source();
        }

        process::exit(1);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    text: String,
    completed: bool,
    priority: Priority,
    tags: Vec<String>,
    due_date: Option<NaiveDate>,
    created_at: NaiveDate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
enum Priority {
    /// High priority - urgent and important tasks
    High,
    /// Medium priority - default for most tasks
    Medium,
    /// Low priority - nice to have, not urgent
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum StatusFilter {
    /// Show only pending tasks
    Pending,
    /// Show only completed tasks
    Done,
    /// Show all tasks (default)
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum DueFilter {
    /// Tasks past their due date
    Overdue,
    /// Tasks due in the next 7 days
    Soon,
    /// Tasks with any due date set
    WithDue,
    /// Tasks without a due date
    NoDue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum SortBy {
    /// Sort by priority (High -> Medium -> Low)
    Priority,
    /// Sort by due date (earliest first)
    Due,
    /// Sort by creation date (oldest first)
    Created,
}

#[derive(Parser)]
#[command(name = "todo-list")]
#[command(author = "github.com/joaofelipegalvao")]
#[command(version = "1.6.0")]
#[command(about = "A modern, powerful task manager built with Rust", long_about = None)]
#[command(after_help = "EXAMPLES:\n    \
    # Add a high priority task\n    \
    todo add \"Complete Rust project\" --priority high --tag work --due 2025-02-15\n\n    \
    # List pending high priority tasks\n    \
    todo list --status pending --priority high\n\n    \
    # List overdue tasks sorted by due date\n    \
    todo list --due overdue --sort due\n\n    \
    # Search for tasks\n    \
    todo search rust\n\n    \
    # Mark task as completed\n    \
    todo done 3\n\n\
For more information, visit: https://github.com/joaofelipegalvao/todo-cli
")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task to your todo list
    #[command(visible_alias = "a")]
    #[command(long_about = "Add a new task to your todo list\n\n\
        Creates a new task with the specified text and optional metadata like priority,\n\
        tags, and due date. Tasks are saved immediately to todos.json.")]
    Add(AddArgs),

    /// List and filter tasks
    #[command(visible_alias = "ls")]
    #[command(
        long_about = "List and filter tasks with powerful filtering options\n\n\
        Display your tasks with filtering and sorting capabilities.\n\
        All filters can be combined to find exactly what you need."
    )]
    List {
        /// Filter by completion status
        #[arg(long, value_enum, default_value_t = StatusFilter::All)]
        status: StatusFilter,

        /// Filter by priority level
        #[arg(long, value_enum)]
        priority: Option<Priority>,

        /// Filter by due date
        #[arg(long, value_enum)]
        due: Option<DueFilter>,

        /// Sort results by field
        #[arg(long, short = 's', value_enum)]
        sort: Option<SortBy>,

        /// Filter by tag name
        #[arg(long, short = 't')]
        tag: Option<String>,
    },

    /// Mark a task as completed
    #[command(visible_alias = "complete")]
    #[command(long_about = "Mark a task as completed\n\n\
        Marks the specified task as done. The task will be shown with a ✓ symbol\n\
        and appear in green when listing tasks.")]
    Done {
        /// Task ID number (from 'list' command)
        #[arg(value_name = "ID")]
        id: usize,
    },

    /// Mark a completed task as pending
    #[command(visible_alias = "undo")]
    #[command(long_about = "Mark a completed task as pending\n\n\
        Reverts a task back to pending status. Useful if you accidentally marked\n\
        a task as done or need to redo it.")]
    Undone {
        /// Task ID number
        #[arg(value_name = "ID")]
        id: usize,
    },

    /// Remove a task permanently
    #[command(visible_aliases = ["rm", "delete"])]
    #[command(long_about = "Remove a task permanently from your list\n\n\
        WARNING: This action cannot be undone. The task will be permanently deleted\n\
        from your todos.json file.")]
    Remove {
        /// Task ID number
        #[arg(value_name = "ID")]
        id: usize,
    },

    /// Clear all tasks
    #[command(visible_alias = "reset")]
    #[command(long_about = "Clear all tasks (removes todos.json file)\n\n\
        WARNING: This will permanently delete ALL tasks. This action cannot be undone.\n\
        You will lose all your tasks, tags, and metadata.")]
    Clear,

    /// Search for tasks by text content
    #[command(visible_alias = "find")]
    #[command(long_about = "Search for tasks by text content\n\n\
        Performs a case-insensitive search through all task descriptions.\n\
        Returns all tasks that contain the search term.")]
    Search {
        /// The text to search for in task descriptions
        #[arg(value_name = "QUERY")]
        query: String,

        /// Filter results by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
    },

    /// List all tags
    #[command(long_about = "List all tags used across your tasks\n\n\
        Shows a summary of all tags you've created, along with the count\n\
        of tasks associated with each tag.")]
    Tags,
}

#[derive(Args)]
struct AddArgs {
    /// Task description
    #[arg(value_name = "DESCRIPTION")]
    text: String,

    /// Task priority level
    #[arg(long, value_enum, default_value_t = Priority::Medium)]
    priority: Priority,

    /// Add tags (can be repeated: -t work -t urgent)
    #[arg(long, short = 't', value_name = "TAG")]
    tag: Vec<String>,

    /// Due date in format YYYY-MM-DD (example: --due 2025-12-31)
    #[arg(long, value_name = "DATE", value_parser = clap::value_parser!(NaiveDate))]
    due: Option<NaiveDate>,
}

impl Task {
    fn new(
        text: String,
        priority: Priority,
        tags: Vec<String>,
        due_date: Option<NaiveDate>,
    ) -> Self {
        Task {
            text,
            completed: false,
            priority,
            tags,
            due_date,
            created_at: Local::now().naive_local().date(),
        }
    }

    fn mark_done(&mut self) {
        self.completed = true;
    }

    fn mark_undone(&mut self) {
        self.completed = false;
    }

    fn is_overdue(&self) -> bool {
        if let Some(due) = self.due_date {
            let today = Local::now().naive_local().date();
            due < today && !self.completed
        } else {
            false
        }
    }

    fn is_due_soon(&self, days: i64) -> bool {
        if let Some(due) = self.due_date {
            let today = Local::now().naive_local().date();
            let days_until = (due - today).num_days();
            days_until >= 0 && days_until <= days && !self.completed
        } else {
            false
        }
    }

    fn matches_status(&self, status: StatusFilter) -> bool {
        match status {
            StatusFilter::Pending => !self.completed,
            StatusFilter::Done => self.completed,
            StatusFilter::All => true,
        }
    }

    fn matches_due_filter(&self, filter: DueFilter) -> bool {
        match filter {
            DueFilter::Overdue => self.is_overdue(),
            DueFilter::Soon => self.is_due_soon(7),
            DueFilter::WithDue => self.due_date.is_some(),
            DueFilter::NoDue => self.due_date.is_none(),
        }
    }
}

impl Priority {
    fn order(&self) -> u8 {
        match self {
            Priority::High => 0,
            Priority::Medium => 1,
            Priority::Low => 2,
        }
    }

    fn letter(&self) -> ColoredString {
        match self {
            Priority::High => "H".red(),
            Priority::Medium => "M".yellow(),
            Priority::Low => "L".green(),
        }
    }
}

fn load_tasks() -> Result<Vec<Task>> {
    match fs::read_to_string("todos.json") {
        Ok(content) => serde_json::from_str(&content)
            .context("Failed to parse todos.json - file may be corrupted"),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e).context(format!(
            "Failed to read todos.json from current directory: {}",
            std::env::current_dir().unwrap_or_default().display()
        )),
    }
}

fn save_tasks(tasks: &[Task]) -> Result<()> {
    let json = serde_json::to_string_pretty(tasks).context("Failed to serialize tasks to JSON")?;

    fs::write("todos.json", json)
        .context("Failed to write to todos.json - check file permissions")?;

    Ok(())
}

fn validate_task_id(id: usize, max: usize) -> Result<(), TodoError> {
    if id == 0 || id > max {
        return Err(TodoError::InvalidTaskId { id, max });
    }
    Ok(())
}

fn calculate_column_widths(tasks: &[(usize, &Task)]) -> (usize, usize, usize) {
    let mut max_task_len = 10;
    let mut max_tags_len = 4;
    let mut max_due_len = 3;

    for (_, task) in tasks {
        max_task_len = max_task_len.max(task.text.len());

        if !task.tags.is_empty() {
            let tags_str = task.tags.join(", ");
            max_tags_len = max_tags_len.max(tags_str.len());
        }

        let due_text = get_due_text(task);
        if !due_text.is_empty() {
            max_due_len = max_due_len.max(due_text.len());
        }
    }

    max_task_len = max_task_len.min(40);
    max_tags_len = max_tags_len.min(20);
    max_due_len = max_due_len.min(20);

    (max_task_len, max_tags_len, max_due_len)
}

fn get_due_text(task: &Task) -> String {
    let Some(due) = task.due_date else {
        return String::new();
    };

    let today = Local::now().naive_local().date();
    let days = (due - today).num_days();

    match days {
        d if d < 0 => {
            let abs_d = d.abs();
            format!("late {} day{}", abs_d, if abs_d == 1 { "" } else { "s" })
        }
        0 => "due today".to_string(),
        d => format!("in {} day{}", d, if d == 1 { "" } else { "s" }),
    }
}

fn get_due_colored(task: &Task, text: &str) -> ColoredString {
    if text.is_empty() {
        return "".normal();
    }

    if task.completed {
        return text.dimmed();
    }

    if let Some(due) = task.due_date {
        let today = Local::now().naive_local().date();
        let days_until = (due - today).num_days();

        if days_until < 0 {
            text.red().bold()
        } else if days_until == 0 {
            text.yellow().bold()
        } else if days_until <= 7 {
            text.yellow()
        } else {
            text.cyan()
        }
    } else {
        text.normal()
    }
}

fn display_task_tabular(number: usize, task: &Task, task_width: usize, tags_width: usize) {
    let number_str = format!("{:>3}", number);
    let letter = task.priority.letter();
    let checkbox = if task.completed {
        "".green()
    } else {
        "".bright_white()
    };

    let task_text = if task.text.len() > task_width {
        format!("{}...", &task.text[..task_width - 3])
    } else {
        task.text.to_owned()
    };

    let tags_str = if task.tags.is_empty() {
        String::new()
    } else {
        let joined = task.tags.join(", ");
        if joined.len() > tags_width {
            format!("{}...", &joined[..tags_width - 3])
        } else {
            joined
        }
    };

    let due_text = get_due_text(task);
    let due_colored = get_due_colored(task, &due_text);

    if task.completed {
        print!("{:>4} ", number_str.dimmed());
        print!(" {} ", letter);
        print!(" {} ", checkbox);
        print!("{:<width$}", task_text.green(), width = task_width);
        print!("  {:<width$}", tags_str.dimmed(), width = tags_width);
        println!("  {}", due_colored);
    } else {
        print!("{:>4} ", number_str.dimmed());
        print!(" {} ", letter);
        print!(" {} ", checkbox);
        print!("{:<width$}", task_text.bright_white(), width = task_width);
        print!("  {:<width$}", tags_str.cyan(), width = tags_width);
        println!("  {}", due_colored);
    }
}

fn display_lists(tasks: &[(usize, &Task)], title: &str) {
    println!("\n{}:\n", title);

    let (task_width, tags_width, due_width) = calculate_column_widths(tasks);

    print!("{:>4} ", "ID".dimmed());
    print!(" {} ", "P".dimmed());
    print!(" {} ", "S".dimmed());
    print!("{:<width$}", "Task".dimmed(), width = task_width);
    print!("  {:<width$}", "Tags".dimmed(), width = tags_width);
    println!("  {}", "Due".dimmed());

    let total_width = task_width + tags_width + due_width + 19;

    println!("{}", "─".repeat(total_width).dimmed());

    let mut completed = 0;
    let total = tasks.len();

    for (number, task) in tasks {
        display_task_tabular(*number, task, task_width, tags_width);

        if task.completed {
            completed += 1;
        }
    }

    println!("\n{}", "─".repeat(total_width).dimmed());

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

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Add(args) => {
            let task = Task::new(args.text, args.priority, args.tag, args.due);
            let mut tasks = load_tasks()?;
            tasks.push(task);
            save_tasks(&tasks)?;
            println!("{}", "✓ Task added".green())
        }

        Commands::List {
            status,
            priority,
            due,
            sort,
            tag,
        } => {
            let all_tasks = load_tasks()?;

            let mut indexed_tasks: Vec<(usize, &Task)> = all_tasks
                .iter()
                .enumerate()
                .map(|(i, task)| (i + 1, task))
                .collect();

            indexed_tasks.retain(|(_, t)| t.matches_status(status));

            if let Some(pri) = priority {
                indexed_tasks.retain(|(_, t)| t.priority == pri);
            }

            if let Some(due_filter) = due {
                indexed_tasks.retain(|(_, t)| t.matches_due_filter(due_filter));
            }

            if let Some(tag_name) = &tag {
                let count_before = indexed_tasks.len();
                indexed_tasks.retain(|(_, t)| t.tags.contains(tag_name));

                if indexed_tasks.is_empty() && count_before > 0 {
                    return Err(TodoError::TagNotFound(tag_name.to_owned()).into());
                }
            }

            if indexed_tasks.is_empty() {
                return Err(TodoError::NoTasksFound.into());
            }

            if let Some(sort_by) = sort {
                match sort_by {
                    SortBy::Priority => {
                        indexed_tasks
                            .sort_by(|(_, a), (_, b)| a.priority.order().cmp(&b.priority.order()));
                    }
                    SortBy::Due => {
                        indexed_tasks.sort_by(|(_, a), (_, b)| match (a.due_date, b.due_date) {
                            (Some(date_a), Some(date_b)) => date_a.cmp(&date_b),
                            (Some(_), None) => std::cmp::Ordering::Less,
                            (None, Some(_)) => std::cmp::Ordering::Greater,
                            (None, None) => std::cmp::Ordering::Equal,
                        });
                    }
                    SortBy::Created => {
                        indexed_tasks.sort_by(|(_, a), (_, b)| a.created_at.cmp(&b.created_at));
                    }
                }
            }

            let title = match (status, priority, due) {
                (StatusFilter::Pending, Some(Priority::High), None) => {
                    "High priority pending tasks"
                }
                (StatusFilter::Pending, Some(Priority::Medium), None) => {
                    "Medium priority pending tasks"
                }
                (StatusFilter::Pending, Some(Priority::Low), None) => "Low priority pending tasks",
                (StatusFilter::Pending, None, Some(DueFilter::Overdue)) => "Pending overdue tasks",
                (StatusFilter::Pending, None, Some(DueFilter::Soon)) => "Pending tasks due soon",
                (StatusFilter::Pending, None, None) => "Pending tasks",
                (StatusFilter::Done, _, _) => "Completed tasks",
                (StatusFilter::All, Some(Priority::High), _) => "High priority tasks",
                (StatusFilter::All, Some(Priority::Medium), _) => "Medium priority tasks",
                (StatusFilter::All, Some(Priority::Low), _) => "Low priority tasks",
                (StatusFilter::All, None, Some(DueFilter::Overdue)) => "Overdue tasks",
                (StatusFilter::All, None, Some(DueFilter::Soon)) => "Tasks due soon",
                (StatusFilter::All, None, Some(DueFilter::WithDue)) => "Tasks with due date",
                (StatusFilter::All, None, Some(DueFilter::NoDue)) => "Tasks without due date",
                _ => "Tasks",
            };

            display_lists(&indexed_tasks, title);
        }

        Commands::Done { id } => {
            let mut tasks = load_tasks()?;
            validate_task_id(id, tasks.len())?;
            let index = id - 1;

            if tasks[index].completed {
                return Err(TodoError::TaskAlreadyInStatus {
                    id,
                    status: "completed".to_owned(),
                }
                .into());
            }

            tasks[index].mark_done();
            save_tasks(&tasks)?;
            println!("{}", "✓ Task marked as completed".green());
        }

        Commands::Undone { id } => {
            let mut tasks = load_tasks()?;
            validate_task_id(id, tasks.len())?;
            let index = id - 1;

            if !tasks[index].completed {
                return Err(TodoError::TaskAlreadyInStatus {
                    id,
                    status: "pending".to_owned(),
                }
                .into());
            }

            tasks[index].mark_undone();
            save_tasks(&tasks)?;
            println!("{}", "✓ Task unmarked".yellow());
        }

        Commands::Remove { id } => {
            let mut tasks = load_tasks()?;
            validate_task_id(id, tasks.len())?;

            let index = id - 1;
            let removed_task = tasks.remove(index);
            save_tasks(&tasks)?;
            println!("{} {}", "✓ Task removed:".red(), removed_task.text.dimmed());
        }

        Commands::Clear => {
            if fs::metadata("todos.json").is_ok() {
                fs::remove_file("todos.json")?;
                println!("{}", "✓ All tasks have been removed".red().bold());
            } else {
                println!("No tasks to remove");
            }
        }

        Commands::Search { query, tag } => {
            let tasks = load_tasks()?;

            let mut results: Vec<(usize, &Task)> = tasks
                .iter()
                .enumerate()
                .filter(|(_, task)| task.text.to_lowercase().contains(&query.to_lowercase()))
                .map(|(i, task)| (i + 1, task))
                .collect();

            if let Some(tag_name) = &tag {
                results.retain(|(_, task)| task.tags.contains(tag_name));
            }

            if results.is_empty() {
                return Err(TodoError::NoSearchResults(query).into());
            } else {
                display_lists(&results, &format!("Search results for \"{}\"", query));
            }
        }

        Commands::Tags => {
            let tasks = load_tasks()?;

            let mut all_tags: Vec<String> = Vec::new();
            for task in &tasks {
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
                let count = tasks.iter().filter(|t| t.tags.contains(tag)).count();
                println!(
                    "  {} ({} task{})",
                    tag.cyan(),
                    count,
                    if count == 1 { "" } else { "s" }
                );
            }

            println!()
        }
    }

    Ok(())
}
