/*
A modern, powerful task manager built with Rust.
*/

use std::process;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

use rustodo::cli::{
    Cli, Commands, HolidaysCommands, NoteCommands, ProjectCommands, ResourceCommands,
};
use rustodo::commands;
use rustodo::storage::{SqliteStorage, Storage, backup, get_db_path};

fn main() {
    let cli = Cli::parse();

    let db_path = match get_db_path() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{} Failed to resolve database path: {}", "✗".red(), e);
            process::exit(1);
        }
    };

    let storage = match SqliteStorage::new() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} Failed to initialize storage: {}", "✗".red(), e);
            process::exit(1);
        }
    };

    if let Err(e) = run(cli, &storage) {
        eprintln!("{} {}", "✗".red(), e);

        let mut source = e.source();
        while let Some(cause) = source {
            eprintln!("  {} {}", "↳".red(), cause);
            source = cause.source();
        }

        process::exit(1);
    }

    // Backup after successful write operations (best-effort)
    let _ = backup::backup_if_needed(&db_path, 10, 60);
}

fn run(cli: Cli, storage: &impl Storage) -> Result<()> {
    // Ensure config.toml is created on first run
    let _ = rustodo::config::Config::load();

    let Some(command) = cli.command else {
        return rustodo::tui::run(storage);
    };

    match command {
        Commands::Add(args) => commands::task::add::execute(storage, args),

        Commands::List {
            status,
            priority,
            due,
            sort,
            tag,
            project,
            recurrence: recur,
        } => {
            commands::task::list::execute(storage, status, priority, due, sort, tag, project, recur)
        }

        Commands::Done { id } => commands::task::done::execute(storage, id),

        Commands::Undone { id } => commands::task::undone::execute(storage, id),

        Commands::Remove { id, yes } => commands::task::remove::execute(storage, id, yes),

        Commands::Edit(args) => commands::task::edit::execute(storage, args),

        Commands::Clear { yes } => commands::task::clear::execute(storage, yes),

        Commands::Search {
            query,
            tag,
            project,
            status,
        } => commands::search::execute(storage, query, tag, project, status),

        Commands::Stats => commands::stats::execute(storage),

        Commands::Calendar { month, year } => commands::calendar::execute(storage, month, year),

        Commands::Next { limit } => commands::next::execute(storage, Some(limit)),

        Commands::Tags { tag } => commands::tags::execute(storage, tag),

        Commands::Project(sub) => match sub {
            ProjectCommands::Add(args) => commands::project::add::execute(storage, args),
            ProjectCommands::List => commands::project::list::execute(storage),
            ProjectCommands::Show { id } => commands::project::show::execute(storage, id),
            ProjectCommands::Edit(args) => commands::project::edit::execute(storage, args),
            ProjectCommands::Done { id } => commands::project::done::execute(storage, id),
            ProjectCommands::Undone { id } => commands::project::undone::execute(storage, id),
            ProjectCommands::Remove { id, yes } => {
                commands::project::remove::execute(storage, id, yes)
            }
            ProjectCommands::Clear { yes } => commands::project::clear::execute(storage, yes),
        },

        Commands::Note(sub) => match sub {
            NoteCommands::Add(args) => commands::note::add::execute(storage, args),
            NoteCommands::List(args) => commands::note::list::execute(storage, args),
            NoteCommands::Show { id } => commands::note::show::execute(storage, id),
            NoteCommands::Preview { id } => commands::note::preview::execute(storage, id),
            NoteCommands::Edit(args) => commands::note::edit::execute(storage, args),
            NoteCommands::Remove { id, yes } => commands::note::remove::execute(storage, id, yes),
            NoteCommands::Clear { yes } => commands::note::clear::execute(storage, yes),
        },

        Commands::Resource(sub) => match sub {
            ResourceCommands::Add(args) => commands::resource::add::execute(storage, args),
            ResourceCommands::List(args) => commands::resource::list::execute(storage, args),
            ResourceCommands::Show { id } => commands::resource::show::execute(storage, id),
            ResourceCommands::Edit(args) => commands::resource::edit::execute(storage, args),
            ResourceCommands::Remove { id, yes } => {
                commands::resource::remove::execute(storage, id, yes)
            }
            ResourceCommands::Clear { yes } => commands::resource::clear::execute(storage, yes),
        },

        Commands::Context { id } => commands::context::execute(storage, id),

        Commands::Deps { id } => commands::task::deps::execute(storage, id),

        Commands::Info => commands::task::info::execute(),

        Commands::Recur { id, pattern } => commands::task::recur::execute(storage, id, pattern),

        Commands::ClearRecur { id } => commands::task::clear_recur::execute(storage, id),

        Commands::Purge { days, dry_run, yes } => {
            commands::purge::execute(storage, days, dry_run, yes)
        }

        Commands::Holidays(sub) => match sub {
            HolidaysCommands::Refresh => commands::holidays_cmd::execute_refresh(),
        },
    }
}
