//! Yes/no confirmation prompt for destructive operations.
//!
//! Used by [`commands::task_remove`] and [`commands::task_clear`] before
//! irreversible actions.
//!
//! [`commands::task_remove`]: crate::commands::task_remove
//! [`commands::task_clear`]: crate::commands::task_clear

use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};

/// Prompts the user for confirmation.
///
/// # Arguments
///
/// * `message` - The confirmation message to display
///
/// # Returns
///
/// `true` if the user confirms (y/Y/yes), `false` otherwise
pub fn confirm(message: &str) -> Result<bool> {
    print!("{} ", message.yellow());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let response = input.trim().to_lowercase();
    Ok(matches!(response.as_str(), "y" | "yes"))
}
