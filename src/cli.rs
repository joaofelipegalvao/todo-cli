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
#[command(override_usage = "todo [COMMAND]")]
#[command(after_help = "\
COMMANDS:
  Task Management:
    add (a), list (ls), done, undone, edit (e), remove (rm), clear, recur, clear-recur

  Viewing & Planning:
    next (n), calendar (cal), stats, search (find), context (ctx), deps, tags

  Organization:
    project, note, resource

  System:
    sync, info, purge, holidays

Run 'todo <COMMAND> --help' for more information on a command.
")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    // ── Task Management ───────────────────────────────────────────────────────
    /// Add a new task to your todo list
    #[command(visible_alias = "a", hide = true)]
    Add(AddArgs),

    /// List and filter tasks
    #[command(visible_alias = "ls", hide = true)]
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
    #[command(visible_alias = "complete", hide = true)]
    Done {
        #[arg(value_name = "ID")]
        id: usize,
    },

    /// Mark a completed task as pending
    #[command(visible_alias = "undo", hide = true)]
    Undone {
        #[arg(value_name = "ID")]
        id: usize,
    },

    /// Edit an existing task
    #[command(visible_alias = "e", hide = true)]
    Edit(EditArgs),

    /// Remove a task permanently
    #[command(visible_aliases = ["rm", "delete"], hide = true)]
    Remove {
        #[arg(value_name = "ID")]
        id: usize,
        #[arg(long, short = 'y')]
        yes: bool,
    },

    /// Clear all tasks
    #[command(visible_alias = "reset", hide = true)]
    Clear {
        #[arg(long, short = 'y')]
        yes: bool,
    },

    /// Set or change recurrence pattern for a task
    #[command(hide = true)]
    Recur {
        #[arg(value_name = "ID")]
        id: usize,
        #[arg(value_enum)]
        pattern: Recurrence,
    },

    /// Remove recurrence pattern from a task
    #[command(visible_alias = "norecur", hide = true)]
    ClearRecur {
        #[arg(value_name = "ID")]
        id: usize,
    },

    // ── Viewing & Planning ────────────────────────────────────────────────────
    /// Show the most urgent pending tasks ready to work on
    #[command(visible_alias = "n", hide = true)]
    Next {
        /// How many tasks to show (default: 5).
        #[arg(long, short = 'n', default_value_t = 5)]
        limit: usize,
    },

    /// Show a monthly calendar with due dates for tasks and projects
    #[command(visible_alias = "cal", hide = true)]
    Calendar {
        /// Month (1-12). Defaults to current month.
        #[arg(value_name = "MONTH")]
        month: Option<u32>,
        /// Year (e.g. 2026). Defaults to current year.
        #[arg(value_name = "YEAR")]
        year: Option<i32>,
    },

    /// Show productivity statistics and activity chart
    #[command(hide = true)]
    Stats,

    /// Search for tasks by text content
    #[command(visible_alias = "find", hide = true)]
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

    /// Show everything linked to a task: project, dependencies, notes, resources
    #[command(visible_alias = "ctx", hide = true)]
    Context {
        #[arg(value_name = "ID")]
        id: usize,
    },

    /// Show dependency graph for a task
    #[command(hide = true)]
    Deps {
        #[arg(value_name = "ID")]
        id: usize,
    },

    /// List all tags with counts, or show hub view for a specific tag
    #[command(hide = true)]
    Tags {
        /// Optional tag name to show hub view (all tasks, notes, resources with this tag).
        #[arg(value_name = "TAG")]
        tag: Option<String>,
    },

    // ── Organization ─────────────────────────────────────────────────────────
    /// Manage projects
    #[command(subcommand, hide = true)]
    Project(ProjectCommands),

    /// Manage notes (documentation linked to projects, tasks, or resources)
    #[command(subcommand, hide = true)]
    Note(NoteCommands),

    /// Manage resources (external references: links, docs, assets)
    #[command(subcommand, hide = true)]
    Resource(ResourceCommands),

    // ── System ────────────────────────────────────────────────────────────────
    /// Sync tasks with a Git repository
    #[command(subcommand, hide = true)]
    Sync(SyncCommands),

    /// Show information about data file location
    #[command(hide = true)]
    Info,

    /// Permanently remove soft-deleted tombstones
    #[command(hide = true)]
    Purge {
        #[arg(long, default_value_t = 30)]
        days: u32,
        #[arg(long)]
        dry_run: bool,
        #[arg(long, short = 'y')]
        yes: bool,
    },

    /// Manage holiday data from holidata.net
    #[command(subcommand, hide = true)]
    Holidays(HolidaysCommands),
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
    /// Preview note body with markdown rendering (requires bat).
    Preview {
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
    /// Note body (short text). Mutually exclusive with --editor and --file.
    #[arg(value_name = "BODY", conflicts_with_all = ["editor", "file"])]
    pub body: Option<String>,
    /// Open $EDITOR to write the note body. Mutually exclusive with <BODY> and --file.
    #[arg(long, conflicts_with_all = ["file"])]
    pub editor: bool,
    /// Read note body from a markdown file. Mutually exclusive with <BODY> and --editor.
    #[arg(long, value_name = "PATH", conflicts_with_all = ["editor"])]
    pub file: Option<std::path::PathBuf>,
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
    /// Edit body inline. Mutually exclusive with --editor.
    #[arg(long, conflicts_with = "editor")]
    pub body: Option<String>,
    /// Open $EDITOR to edit the note body. Mutually exclusive with --body.
    #[arg(long, conflicts_with = "body")]
    pub editor: bool,
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

// ── Holidays subcommands ──────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum HolidaysCommands {
    /// Download or refresh holiday data from holidata.net for the configured locale.
    Refresh,
}
