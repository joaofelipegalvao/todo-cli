/*
A modern, powerful task manager built with Rust.
*/

use std::process;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

use rustodo::cli::{Cli, Commands, SyncCommands};
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
    match cli.command {
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
