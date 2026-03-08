//! Command-line interface definitions.

use clap::{Args, Parser, Subcommand};

use crate::models::{
    Difficulty, DueFilter, Priority, Recurrence, RecurrenceFilter, ResourceType, SortBy,
    StatusFilter,
};

#[derive(Parser)]
#[command(name = "rustodo")]
#[command(author = "github.com/joaofelipegalvao")]
#[command(version)]
#[command(about = "A modern, powerful task manager built with Rust", long_about = None)]
#[command(after_help = "EXAMPLES:\n    \
    # Launch interactive TUI (default)\n    \
    todo\n\n    \
    # Add a task to a project with a natural language date\n    \
    todo add \"Fix login bug\" --project \"Backend\" --priority high --due \"next friday\"\n\n    \
    # Add a project with tech stack\n    \
    todo project add \"Backend\" --difficulty hard --tech Rust,PostgreSQL\n\n    \
    # Mark a project as done\n    \
    todo project edit 1 --done\n\n    \
    # Add a note linked to a project\n    \
    todo note add \"Setup do banco de dados\" --project \"Backend\" --language Rust\n\n    \
    # Add a resource and link it to a note\n    \
    todo resource add \"sqlx docs\" --url https://docs.rs/sqlx --type docs --tag rust,db\n    \
    todo note edit 1 --add-resource 1\n\n    \
    # Configure sync and push\n    \
    todo sync init git@github.com:user/tasks.git\n    \
    todo sync push\n\n\
For more information, visit: https://github.com/joaofelipegalvao/rustodo\n")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task to your todo list
    #[command(visible_alias = "a")]
    Add(AddArgs),

    /// List and filter tasks
    #[command(visible_alias = "ls")]
    List {
        #[arg(long, value_enum, default_value_t = StatusFilter::All)]
        status: StatusFilter,
        #[arg(long, value_enum)]
        priority: Option<Priority>,
        #[arg(long, value_enum)]
        due: Option<DueFilter>,
        #[arg(long, short = 's', value_enum)]
        sort: Option<SortBy>,
        #[arg(long, short = 't', value_delimiter = ',')]
        tag: Vec<String>,
        #[arg(long, short = 'p')]
        project: Option<String>,
        #[arg(long, short = 'r', value_enum)]
        recurrence: Option<RecurrenceFilter>,
    },

    /// Mark a task as completed
    #[command(visible_alias = "complete")]
    Done {
        #[arg(value_name = "ID")]
        id: usize,
    },

    /// Mark a completed task as pending
    #[command(visible_alias = "undo")]
    Undone {
        #[arg(value_name = "ID")]
        id: usize,
    },

    /// Remove a task permanently
    #[command(visible_aliases = ["rm", "delete"])]
    Remove {
        #[arg(value_name = "ID")]
        id: usize,
        #[arg(long, short = 'y')]
        yes: bool,
    },

    /// Edit an existing task
    #[command(visible_alias = "e")]
    Edit(EditArgs),

    /// Clear all tasks
    #[command(visible_alias = "reset")]
    Clear {
        #[arg(long, short = 'y')]
        yes: bool,
    },

    /// Search for tasks by text content
    #[command(visible_alias = "find")]
    Search {
        #[arg(value_name = "QUERY")]
        query: String,
        #[arg(long, short = 't', value_delimiter = ',')]
        tag: Vec<String>,
        #[arg(long, short = 'p')]
        project: Option<String>,
        #[arg(long, value_enum, default_value_t = StatusFilter::All)]
        status: StatusFilter,
    },

    /// Show productivity statistics and activity chart.
    Stats,

    /// List all tags with counts, or show hub view for a specific tag.
    Tags {
        /// Optional tag name to show hub view (all tasks, notes, resources with this tag).
        #[arg(value_name = "TAG")]
        tag: Option<String>,
    },

    /// List all projects (shorthand for `todo project list`).
    Projects,

    /// Manage projects.
    #[command(subcommand)]
    Project(ProjectCommands),

    /// Show everything linked to a task: project, dependencies, notes, resources.
    #[command(visible_alias = "ctx")]
    Context {
        #[arg(value_name = "ID")]
        id: usize,
    },

    /// Show dependency graph for a task.
    Deps {
        #[arg(value_name = "ID")]
        id: usize,
    },

    /// Show information about data file location.
    Info,

    /// Set or change recurrence pattern for a task.
    Recur {
        #[arg(value_name = "ID")]
        id: usize,
        #[arg(value_enum)]
        pattern: Recurrence,
    },

    /// Remove recurrence pattern from a task.
    #[command(visible_alias = "norecur")]
    ClearRecur {
        #[arg(value_name = "ID")]
        id: usize,
    },

    /// Permanently remove soft-deleted tombstones.
    Purge {
        #[arg(long, default_value_t = 30)]
        days: u32,
        #[arg(long)]
        dry_run: bool,
        #[arg(long, short = 'y')]
        yes: bool,
    },

    /// Manage notes (documentation linked to projects, tasks, or resources).
    #[command(subcommand)]
    Note(NoteCommands),

    /// Manage resources (external references: links, docs, assets).
    #[command(subcommand)]
    Resource(ResourceCommands),

    /// Sync tasks with a Git repository.
    #[command(subcommand)]
    Sync(SyncCommands),
}

// ── Project subcommands ───────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum ProjectCommands {
    /// Add a new project.
    Add(ProjectAddArgs),
    /// List all projects.
    List,
    /// Show full details of a project.
    Show {
        #[arg(value_name = "ID")]
        id: usize,
    },
    /// Edit an existing project.
    Edit(ProjectEditArgs),
    /// Mark a project as completed.
    Done {
        #[arg(value_name = "ID")]
        id: usize,
    },
    /// Mark a completed project as pending.
    Undone {
        #[arg(value_name = "ID")]
        id: usize,
    },
    /// Remove a project (soft delete).
    Remove {
        #[arg(value_name = "ID")]
        id: usize,
        #[arg(long, short = 'y')]
        yes: bool,
    },
    /// Clear all projects (soft delete).
    Clear {
        #[arg(long, short = 'y')]
        yes: bool,
    },
}

// ── ProjectAddArgs ────────────────────────────────────────────────────────────

#[derive(Args)]
pub struct ProjectAddArgs {
    #[arg(value_name = "NAME")]
    pub name: String,
    #[arg(long, value_enum)]
    pub difficulty: Option<Difficulty>,
    #[arg(long, value_delimiter = ',')]
    pub tech: Vec<String>,
    #[arg(long, value_name = "DATE|EXPRESSION")]
    pub due: Option<String>,
}

// ── ProjectEditArgs ───────────────────────────────────────────────────────────

#[derive(Args)]
pub struct ProjectEditArgs {
    #[arg(value_name = "ID")]
    pub id: usize,
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long, value_enum)]
    pub difficulty: Option<Difficulty>,
    /// Mark project as completed.
    #[arg(long, conflicts_with = "undone")]
    pub done: bool,
    /// Mark project as pending.
    #[arg(long, conflicts_with = "done")]
    pub undone: bool,
    #[arg(long, value_delimiter = ',', conflicts_with = "clear_tech")]
    pub add_tech: Vec<String>,
    #[arg(long, value_delimiter = ',', conflicts_with = "clear_tech")]
    pub remove_tech: Vec<String>,
    #[arg(long, conflicts_with_all = ["add_tech", "remove_tech"])]
    pub clear_tech: bool,
    #[arg(long, value_name = "DATE|EXPRESSION", conflicts_with = "clear_due")]
    pub due: Option<String>,
    #[arg(long, conflicts_with = "due")]
    pub clear_due: bool,
}

// ── Note subcommands ──────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum NoteCommands {
    /// Add a new note.
    Add(NoteAddArgs),
    /// List notes with optional filters.
    List(NoteListArgs),
    /// Show the full content of a note.
    Show {
        #[arg(value_name = "ID")]
        id: usize,
    },
    /// Edit an existing note.
    Edit(NoteEditArgs),
    /// Remove a note (soft delete).
    Remove {
        #[arg(value_name = "ID")]
        id: usize,
        #[arg(long, short = 'y')]
        yes: bool,
    },
    /// Clear all notes (soft delete).
    Clear {
        #[arg(long, short = 'y')]
        yes: bool,
    },
}

// ── NoteAddArgs ───────────────────────────────────────────────────────────────

#[derive(Args)]
pub struct NoteAddArgs {
    #[arg(value_name = "BODY")]
    pub body: String,
    #[arg(long)]
    pub title: Option<String>,
    #[arg(long, short = 't', value_delimiter = ',')]
    pub tag: Vec<String>,
    #[arg(long, short = 'l')]
    pub language: Option<String>,
    #[arg(long, short = 'p')]
    pub project: Option<String>,
    #[arg(long)]
    pub task: Option<usize>,
}

// ── NoteListArgs ──────────────────────────────────────────────────────────────

#[derive(Args)]
pub struct NoteListArgs {
    #[arg(long, short = 'p')]
    pub project: Option<String>,
    #[arg(long, short = 't')]
    pub tag: Option<String>,
    #[arg(long, short = 'l')]
    pub language: Option<String>,
}

// ── NoteEditArgs ──────────────────────────────────────────────────────────────

#[derive(Args)]
pub struct NoteEditArgs {
    #[arg(value_name = "ID")]
    pub id: usize,
    #[arg(long)]
    pub body: Option<String>,
    #[arg(long, conflicts_with = "clear_title")]
    pub title: Option<String>,
    #[arg(long, conflicts_with = "title")]
    pub clear_title: bool,
    #[arg(long, short = 'l', conflicts_with = "clear_language")]
    pub language: Option<String>,
    #[arg(long, conflicts_with = "language")]
    pub clear_language: bool,
    #[arg(long, value_delimiter = ',', conflicts_with = "clear_tags")]
    pub add_tag: Vec<String>,
    #[arg(long, value_delimiter = ',', conflicts_with = "clear_tags")]
    pub remove_tag: Vec<String>,
    #[arg(long, conflicts_with_all = ["add_tag", "remove_tag"])]
    pub clear_tags: bool,
    #[arg(long, short = 'p', conflicts_with = "clear_project")]
    pub project: Option<String>,
    #[arg(long, conflicts_with = "project")]
    pub clear_project: bool,
    #[arg(long, conflicts_with = "clear_task")]
    pub task: Option<usize>,
    #[arg(long, conflicts_with = "task")]
    pub clear_task: bool,
    /// Link one or more resources to this note by display ID.
    #[arg(long, value_name = "ID", conflicts_with = "clear_resources")]
    pub add_resource: Vec<usize>,
    /// Unlink one or more resources from this note by display ID.
    #[arg(long, value_name = "ID", conflicts_with = "clear_resources")]
    pub remove_resource: Vec<usize>,
    /// Remove all resource links from this note.
    #[arg(long, conflicts_with_all = ["add_resource", "remove_resource"])]
    pub clear_resources: bool,
}

// ── Resource subcommands ──────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum ResourceCommands {
    /// Add a new resource.
    Add(ResourceAddArgs),
    /// List all resources.
    List(ResourceListArgs),
    /// Show full details of a resource.
    Show {
        #[arg(value_name = "ID")]
        id: usize,
    },
    /// Edit an existing resource.
    Edit(ResourceEditArgs),
    /// Remove a resource (soft delete).
    Remove {
        #[arg(value_name = "ID")]
        id: usize,
        #[arg(long, short = 'y')]
        yes: bool,
    },
    /// Clear all resources (soft delete).
    Clear {
        #[arg(long, short = 'y')]
        yes: bool,
    },
}

// ── ResourceAddArgs ───────────────────────────────────────────────────────────

#[derive(Args)]
pub struct ResourceAddArgs {
    #[arg(value_name = "TITLE")]
    pub title: String,
    #[arg(long, value_enum)]
    pub r#type: Option<ResourceType>,
    #[arg(long, short = 'u')]
    pub url: Option<String>,
    #[arg(long, short = 'd')]
    pub description: Option<String>,
    #[arg(long, short = 't', value_delimiter = ',')]
    pub tag: Vec<String>,
}

// ── ResourceListArgs ──────────────────────────────────────────────────────────

#[derive(Args)]
pub struct ResourceListArgs {
    #[arg(long, short = 't')]
    pub tag: Option<String>,
    #[arg(long, value_enum)]
    pub r#type: Option<ResourceType>,
}

// ── ResourceEditArgs ──────────────────────────────────────────────────────────

#[derive(Args)]
pub struct ResourceEditArgs {
    #[arg(value_name = "ID")]
    pub id: usize,
    #[arg(long)]
    pub title: Option<String>,
    #[arg(long, value_enum, conflicts_with = "clear_type")]
    pub r#type: Option<ResourceType>,
    #[arg(long, conflicts_with = "type")]
    pub clear_type: bool,
    #[arg(long, short = 'u', conflicts_with = "clear_url")]
    pub url: Option<String>,
    #[arg(long, conflicts_with = "url")]
    pub clear_url: bool,
    #[arg(long, short = 'd', conflicts_with = "clear_description")]
    pub description: Option<String>,
    #[arg(long, conflicts_with = "description")]
    pub clear_description: bool,
    #[arg(long, value_delimiter = ',', conflicts_with = "clear_tags")]
    pub add_tag: Vec<String>,
    #[arg(long, value_delimiter = ',', conflicts_with = "clear_tags")]
    pub remove_tag: Vec<String>,
    #[arg(long, conflicts_with_all = ["add_tag", "remove_tag"])]
    pub clear_tags: bool,
}

// ── Sync subcommands ──────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum SyncCommands {
    Init {
        #[arg(value_name = "REMOTE")]
        remote: String,
    },
    Push,
    Pull,
    Status,
}

// ── AddArgs ───────────────────────────────────────────────────────────────────

#[derive(Args)]
pub struct AddArgs {
    #[arg(value_name = "DESCRIPTION")]
    pub text: String,
    #[arg(long, value_enum, default_value_t = Priority::Medium)]
    pub priority: Priority,
    #[arg(long, short = 't', value_name = "TAG", value_delimiter = ',')]
    pub tag: Vec<String>,
    #[arg(long, short = 'p', value_name = "PROJECT")]
    pub project: Option<String>,
    #[arg(long, value_name = "DATE|EXPRESSION")]
    pub due: Option<String>,
    #[arg(long, value_enum)]
    pub recurrence: Option<Recurrence>,
    #[arg(long, value_name = "ID")]
    pub depends_on: Vec<usize>,
}

// ── EditArgs ──────────────────────────────────────────────────────────────────

#[derive(Args)]
pub struct EditArgs {
    #[arg(value_name = "ID")]
    pub id: usize,
    #[arg(long)]
    pub text: Option<String>,
    #[arg(long, value_enum)]
    pub priority: Option<Priority>,
    #[arg(long, value_delimiter = ',')]
    pub add_tag: Vec<String>,
    #[arg(long, value_delimiter = ',')]
    pub remove_tag: Vec<String>,
    #[arg(long, short = 'p', conflicts_with = "clear_project")]
    pub project: Option<String>,
    #[arg(long, conflicts_with = "project")]
    pub clear_project: bool,
    #[arg(long, value_name = "DATE|EXPRESSION")]
    pub due: Option<String>,
    #[arg(long, conflicts_with = "due")]
    pub clear_due: bool,
    #[arg(long, conflicts_with_all = ["add_tag", "remove_tag"])]
    pub clear_tags: bool,
    #[arg(long, value_name = "ID", conflicts_with = "clear_deps")]
    pub add_dep: Vec<usize>,
    #[arg(long, value_name = "ID", conflicts_with = "clear_deps")]
    pub remove_dep: Vec<usize>,
    #[arg(long, conflicts_with_all = ["add_dep", "remove_dep"])]
    pub clear_deps: bool,
}
