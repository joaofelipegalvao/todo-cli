//! Handler for `todo info`.
//!
//! Prints the path to the active data file, whether it exists, and its size
//! on disk. Useful for locating the file for backups or debugging.

use std::fs;

use anyhow::Result;
use colored::Colorize;

use crate::storage::get_data_file_path;

pub fn execute() -> Result<()> {
    let path = get_data_file_path()?;
    let exists = path.exists();

    println!("\n{} Todo-List Information\n", "".blue().bold());
    println!("{} {}", "Data file:".dimmed(), path.display());

    if exists {
        println!("{} {}", "Status:".dimmed(), "exists ✓".green());
        let metadata = fs::metadata(&path)?;
        let size = metadata.len();
        println!("{} {} bytes", "Size:".dimmed(), size);
    } else {
        println!("{} {}", "Status:".dimmed(), "not created yet".blue());
    }

    println!();
    Ok(())
}
