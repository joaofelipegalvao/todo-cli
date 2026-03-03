//! Command-line interface definitions.
//!
//! This module uses [`clap`] to define the full CLI surface of `rustodo`:
//! the top-level [`Cli`] struct, the [`Commands`] enum with one variant per
//! subcommand, and the [`AddArgs`] helper struct for the `add` subcommand.
//!
//! Consumers of the library can use these types to embed `rustodo` commands
//! in their own applications.

use clap::{Args, Parser, Subcommand};

use crate::models::{DueFilter, Priority, Recurrence, RecurrenceFilter, SortBy, StatusFilter};

/// Top-level CLI entry point.
#[derive(Parser)]
#[command(name = "rustodo")]
#[command(author = "github.com/joaofelipegalvao")]
#[command(version)]
#[command(about = "A modern, powerful task manager built with Rust", long_about = None)]
#[command(after_help = "EXAMPLES:\n    \
    # Add a task to a project with a natural language date\n    \
    todo add \"Fix login bug\" --project \"Backend\" --priority high --due \"next friday\"\n\n    \
    # Add a task due in 3 days\n    \
    todo add \"Review PR\" --due \"in 3 days\"\n\n    \
    # Add a task with strict date format\n    \
    todo add \"Project deadline\" --due 2026-03-15\n\n    \
    # List all tasks in a project\n    \
    todo list --project \"Backend\"\n\n    \
    # List pending tasks in a project, sorted by due date\n    \
    todo list --project \"Backend\" --status pending --sort due\n\n    \
    # Configure sync and push\n    \
    todo sync init git@github.com:user/tasks.git\n    \
    todo sync push\n\n\
For more information, visit: https://github.com/joaofelipegalvao/rustodo
")]
pub struct Cli {
    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// All available subcommands.
#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task to your todo list
    #[command(visible_alias = "a")]
    #[command(long_about = "Add a new task to your todo list\n\n\
        Creates a new task with the specified text and optional metadata like priority,\n\
        tags, and due date. Tasks are saved immediately to todos.json.\n\n\
        Due dates accept both natural language and strict YYYY-MM-DD format:\n  \
        todo add \"Meeting\" --due tomorrow\n  \
        todo add \"Deploy\" --due \"next friday\"\n  \
        todo add \"Report\" --due \"in 3 days\"\n  \
        todo add \"Deadline\" --due 2026-03-15\n\n\
        Assign to a project:\n  \
        todo add \"Fix bug\" --project \"Backend\"\n  \
        todo add \"Write docs\" --project \"Documentation\" --tag work\n\n\
        Use --recurrence to make the task repeat automatically when completed.")]
    Add(AddArgs),

    /// List and filter tasks
    #[command(visible_alias = "ls")]
    #[command(
        long_about = "List and filter tasks with powerful filtering options\n\n\
        Examples:\n  \
        todo list --project \"Backend\"\n  \
        todo list --project \"Backend\" --status pending\n  \
        todo list --recurrence daily\n  \
        todo list --status pending --priority high --sort due"
    )]
    List {
        /// Show all, pending or done tasks.
        #[arg(long, value_enum, default_value_t = StatusFilter::All)]
        status: StatusFilter,
        /// Filter by priority level.
        #[arg(long, value_enum)]
        priority: Option<Priority>,
        /// Filter by due-date window
        #[arg(long, value_enum)]
        due: Option<DueFilter>,
        /// Sort results.
        #[arg(long, short = 's', value_enum)]
        sort: Option<SortBy>,
        /// Filter by tag name.
        #[arg(long, short = 't')]
        tag: Option<String>,
        /// Filter by project name (case-insensitive)
        #[arg(long, short = 'p')]
        project: Option<String>,
        /// Filter by recurrence pattern
        #[arg(long, short = 'r', value_enum)]
        recurrence: Option<RecurrenceFilter>,
    },

    /// Mark a task as completed
    #[command(visible_alias = "complete")]
    Done {
        /// 1-based task ID
        #[arg(value_name = "ID")]
        id: usize,
    },

    /// Mark a completed task as pending
    #[command(visible_alias = "undo")]
    Undone {
        /// 1-based task ID
        #[arg(value_name = "ID")]
        id: usize,
    },

    /// Remove a task permanently
    #[command(visible_aliases = ["rm", "delete"])]
    Remove {
        /// 1-based Task ID.
        #[arg(value_name = "ID")]
        id: usize,
        /// Skip the interactive confirmation prompt.
        #[arg(long, short = 'y')]
        yes: bool,
    },

    /// Edit an existing task
    #[command(visible_alias = "e")]
    #[command(long_about = "Edit an existing task\n\n\
        Modify task properties like text, priority, tags, or due date.\n\
        Only specify the fields you want to change.\n\n\
        Due dates accept natural language or YYYY-MM-DD:\n  \
        todo edit 3 --due tomorrow\n  \
        todo edit 3 --due \"next monday\"\n  \
        todo edit 3 --due \"in 5 days\"\n  \
        todo edit 3 --due 2026-04-01\n\n\
        Tag operations:\n  \
        todo edit 1 --add-tag urgent,critical     # Add multiple tags\n  \
        todo edit 1 --remove-tag work,team        # Remove multiple tags\n  \
        todo edit 1 --add-tag urgent --remove-tag team  # Combine operations\n\n\
        Project operations:\n  \
        todo edit 3 --project \"Backend\"   # Assign to a project\n  \
        todo edit 3 --clear-project         # Remove from project")]
    Edit(EditArgs),

    /// Clear all tasks
    #[command(visible_alias = "reset")]
    Clear {
        /// Skip the interactive confirmation prompt.
        #[arg(long, short = 'y')]
        yes: bool,
    },

    /// Search for tasks by text content
    #[command(visible_alias = "find")]
    Search {
        /// Case-insensitive substring to match against task descriptions.
        #[arg(value_name = "QUERY")]
        query: String,
        /// Narrow results to a specific tag.
        #[arg(long, short = 't')]
        tag: Option<String>,
        /// Narrow results to a specific project.
        #[arg(long, short = 'p')]
        project: Option<String>,
        /// Narrow results by completion status.
        #[arg(long, value_enum, default_value_t = StatusFilter::All)]
        status: StatusFilter,
    },

    /// Show productivity statistics and activity chart.
    Stats,

    /// List all tags with task counts.
    Tags,

    /// List all projects with task counts.
    #[command(long_about = "List all projects used across your tasks\n\n\
        Shows each project name with the count of pending and completed tasks.\n\n\
        Use 'todo list --project <NAME>' to see tasks within a specific project.")]
    Projects,

    /// Show dependency graph for a task.
    #[command(long_about = "Show the dependency graph for a task\n\n
        Displays:\n  \
        • Tasks this task depends on (with completion status)\n  \
        • Tasks that depend on this one\n  \
        • Whether the task is currently blocked\n\n\
        Examples:\n  \
        todo deps 5\n  \
        todo deps 1")]
    Deps {
        /// 1-based task ID.
        #[arg(value_name = "ID")]
        id: usize,
    },

    /// Show information about data file location.
    Info,

    /// Set or change recurrence pattern for a task.
    Recur {
        /// 1-based task ID.
        #[arg(value_name = "ID")]
        id: usize,
        /// Desired recurrence pattern.
        #[arg(value_enum)]
        pattern: Recurrence,
    },

    /// Remove recurrence pattern from a task.
    #[command(visible_alias = "norecur")]
    #[command(long_about = "Remove recurrence pattern from a task\n\n\
        Stops a task from repeating automatically. The task will remain\n\
        but won't create new occurrences when completed.\n\n\
        Aliases: norecur")]
    ClearRecur {
        /// 1-based task ID.
        #[arg(value_name = "ID")]
        id: usize,
    },

    /// Permanently remove soft-deleted tombstones.
    #[command(long_about = "Permanently remove soft-deleted tombstones\n\n\
        Deleted tasks are kept as tombstones so sync can propagate deletions\n\
        across devices. Once all devices have synced, tombstones can be purged.\n\n\
        Examples:\n  \
        todo purge                # remove tombstones older than 30 days\n  \
        todo purge --days 7       # remove tombstones older than 7 days\n  \
        todo purge --days 0       # remove all tombstones immediately\n  \
        todo purge --dry-run      # preview without removing")]
    Purge {
        /// Remove tombstones older than this many days (default: 30).
        #[arg(long, default_value_t = 30)]
        days: u32,
        /// Preview what would be removed without actually removing.
        #[arg(long)]
        dry_run: bool,
        /// Skip the confirmation prompt.
        #[arg(long, short = 'y')]
        yes: bool,
    },

    /// Sync tasks with a Git repository.
    #[command(subcommand)]
    #[command(long_about = "Sync tasks with a Git repository.\n\n\
        Subcommands:\n  \
        todo sync init <remote>  — initialize repo and configure remote\n  \
        todo sync push           — commit changes and push\n  \
        todo sync pull           — pull and merge changes\n  \
        todo sync status         — show sync state\n\n\
        First time setup:\n  \
        todo sync init git@github.com:user/tasks.git\n  \
        todo sync push")]
    Sync(SyncCommands),
}

/// Subcommands for `todo sync`.
#[derive(Subcommand)]
pub enum SyncCommands {
    /// Initialize Git repo in the data directory and configure remote.
    #[command(long_about = "Initialize sync with a Git remote.\n\n\
        Creates a Git repository in the rustodo data directory,\n\
        adds the remote, and makes an initial commit.\n\n\
        Example:\n  \
        todo sync init git@github.com:user/tasks.git")]
    Init {
        /// Git remote URL (SSH or HTTPS).
        ///
        /// Examples:
        ///   git@github.com:user/tasks.git
        ///   <https://github.com/user/tasks.git>
        #[arg(value_name = "REMOTE")]
        remote: String,
    },

    /// Commit todos.json and push to remote.
    #[command(
        long_about = "Commit any pending changes to todos.json and push to the remote.\n\n\
        The commit message summarises the current task counts.\n\
        No-ops if todos.json has not changed since the last commit.\n\n\
        Requires: todo sync init <remote> to be run first."
    )]
    Push,

    /// Pull from remote and merge changes.
    #[command(long_about = "Pull latest changes from the remote.\n\n\
        Uses git pull --rebase to keep history linear.\n\
        Semantic UUID-based merge is planned for Phase 2.\n\n\
        Requires: todo sync init <remote> to be run first.")]
    Pull,

    /// Show sync state (branch, last commit, dirty status).
    Status,
}

/// Arguments for the `add` subcommand.
#[derive(Args)]
pub struct AddArgs {
    /// Task description text.
    #[arg(value_name = "DESCRIPTION")]
    pub text: String,
    /// Priority level (default: medium).
    #[arg(long, value_enum, default_value_t = Priority::Medium)]
    pub priority: Priority,
    /// Tags to attach (comma-separated or repeat flag).
    #[arg(long, short = 't', value_name = "TAG", value_delimiter = ',')]
    pub tag: Vec<String>,
    /// Project to assign the task to.
    #[arg(long, short = 'p', value_name = "PROJECT")]
    pub project: Option<String>,
    /// Due date — accepts natural language or YYYY-MM-DD.
    ///
    /// Examples: `tomorrow`, `"next friday"`, `"in 3 days"`, `2026-03-15`
    #[arg(long, value_name = "DATE|EXPRESSION")]
    pub due: Option<String>,
    /// Recurrence pattern. Requires `--due` to be set.
    #[arg(long, value_enum)]
    pub recurrence: Option<Recurrence>,
    /// Task IDs this task depends on (must be completed first).
    #[arg(long, value_name = "ID")]
    pub depends_on: Vec<usize>,
}

/// Arguments for the `edit` subcommand.
#[derive(Args)]
pub struct EditArgs {
    /// 1-based task ID.
    #[arg(value_name = "ID")]
    pub id: usize,
    /// New task description.
    #[arg(long)]
    pub text: Option<String>,
    /// New priority level.
    #[arg(long, value_enum)]
    pub priority: Option<Priority>,
    /// Tags to add (comma-separated or repeat flag).
    #[arg(long, value_delimiter = ',')]
    pub add_tag: Vec<String>,
    /// Tags to remove (comma-separated or repeat flag).
    #[arg(long, value_delimiter = ',')]
    pub remove_tag: Vec<String>,
    /// Assign task to a project.
    #[arg(long, short = 'p', conflicts_with = "clear_project")]
    pub project: Option<String>,
    /// Remove task from its current project.
    #[arg(long, conflicts_with = "project")]
    pub clear_project: bool,
    /// New due date - accepts natural language or YYYY-MM-DD.
    #[arg(long, value_name = "DATE|EXPRESSION")]
    pub due: Option<String>,
    /// Remove the due date.
    #[arg(long, conflicts_with = "due")]
    pub clear_due: bool,
    /// Remove all tags.
    #[arg(long, conflicts_with_all = ["add_tag", "remove_tag"])]
    pub clear_tags: bool,
    /// Task IDs to add as dependencies.
    #[arg(long, value_name = "ID", conflicts_with = "clear_deps")]
    pub add_dep: Vec<usize>,
    /// Task IDs to remove from dependencies.
    #[arg(long, value_name = "ID", conflicts_with = "clear_deps")]
    pub remove_dep: Vec<usize>,
    /// Remove all dependencies from this task.
    #[arg(long, conflicts_with_all = ["add_dep", "remove_dep"])]
    pub clear_deps: bool,
}
