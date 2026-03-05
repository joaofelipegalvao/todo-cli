/*
A modern, powerful task manager built with Rust.
*/

use std::process;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

use rustodo::cli::{Cli, Commands, NoteCommands, ProjectCommands, ResourceCommands, SyncCommands};
use rustodo::commands;
use rustodo::commands::sync::SyncCommand;
use rustodo::storage::{JsonStorage, Storage};

fn main() {
    let cli = Cli::parse();

    let storage = match JsonStorage::new() {
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
}

fn run(cli: Cli, storage: &impl Storage) -> Result<()> {
    let Some(command) = cli.command else {
        return rustodo::tui::run(storage);
    };

    match command {
        Commands::Add(args) => commands::add::execute(storage, args),

        Commands::List {
            status,
            priority,
            due,
            sort,
            tag,
            project,
            recurrence: recur,
        } => commands::list::execute(storage, status, priority, due, sort, tag, project, recur),

        Commands::Done { id } => commands::done::execute(storage, id),

        Commands::Undone { id } => commands::undone::execute(storage, id),

        Commands::Remove { id, yes } => commands::remove::execute(storage, id, yes),

        Commands::Edit(args) => commands::edit::execute(storage, args),

        Commands::Clear { yes } => commands::clear::execute(storage, yes),

        Commands::Search {
            query,
            tag,
            project,
            status,
        } => commands::search::execute(storage, query, tag, project, status),

        Commands::Stats => commands::stats::execute(storage),

        Commands::Tags => commands::tags::execute(storage),

        Commands::Projects => commands::projects::execute(storage),

        Commands::Project(sub) => match sub {
            ProjectCommands::Add(args) => commands::project_add::execute(storage, args),
            ProjectCommands::List => commands::projects::execute(storage),
            ProjectCommands::Show { id } => commands::projects::execute_show(storage, id),
            ProjectCommands::Edit(args) => commands::project_edit::execute(storage, args),
            ProjectCommands::Remove { id, yes } => {
                commands::project_remove::execute(storage, id, yes)
            }
        },

        Commands::Note(sub) => match sub {
            NoteCommands::Add(args) => commands::note_add::execute(storage, args),
            NoteCommands::List(args) => commands::note_list::execute(storage, args),
            NoteCommands::Show { id } => commands::note_show::execute(storage, id),
            NoteCommands::Edit(args) => commands::note_edit::execute(storage, args),
            NoteCommands::Remove { id, yes } => commands::note_remove::execute(storage, id, yes),
        },

        Commands::Resource(sub) => match sub {
            ResourceCommands::Add(args) => commands::resource_add::execute(storage, args),
            ResourceCommands::List(args) => commands::resource_list::execute(storage, args),
            ResourceCommands::Show { id } => commands::resource_show::execute(storage, id),
            ResourceCommands::Edit(args) => commands::resource_edit::execute(storage, args),
            ResourceCommands::Remove { id, yes } => {
                commands::resource_remove::execute(storage, id, yes)
            }
        },

        Commands::Deps { id } => commands::deps::execute(storage, id),

        Commands::Info => commands::info::execute(),

        Commands::Recur { id, pattern } => commands::recur::execute(storage, id, pattern),

        Commands::ClearRecur { id } => commands::clear_recur::execute(storage, id),

        Commands::Purge { days, dry_run, yes } => {
            commands::purge::execute(storage, days, dry_run, yes)
        }

        Commands::Sync(sub) => {
            let cmd = match sub {
                SyncCommands::Init { remote } => SyncCommand::Init { remote },
                SyncCommands::Push => SyncCommand::Push,
                SyncCommands::Pull => SyncCommand::Pull,
                SyncCommands::Status => SyncCommand::Status,
            };
            commands::sync::execute(storage, cmd)
        }
    }
}
