//! Handler for `todo note preview <ID>`.

use anyhow::Result;
use colored::Colorize;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::storage::Storage;
use crate::utils::validation::resolve_visible;

pub fn execute(storage: &impl Storage, id: usize) -> Result<()> {
    let (_, _, notes, _) = storage.load_all_with_resources()?;

    let note = resolve_visible(&notes, id, |n| n.is_deleted())
        .map_err(|_| anyhow::anyhow!("Note #{} not found", id))?;

    // ── Check note format ─────────────────────────────────────────────────────
    if !note.is_markdown() {
        eprintln!("{} Note #{} is plain text, not markdown.", "✗".red(), id);
        eprintln!("  Use {} to view it.", "todo note show".cyan());
        return Ok(());
    }

    // ── Check glow is available ───────────────────────────────────────────────
    if Command::new("glow").arg("--version").output().is_err() {
        eprintln!("{} {} is not installed.", "✗".red(), "glow".yellow());
        eprintln!(
            "  Install it from: {}",
            "https://github.com/charmbracelet/glow".cyan()
        );
        return Ok(());
    }

    // ── Pipe note body into glow via stdin ────────────────────────────────────
    let mut child = Command::new("glow")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(note.body.as_bytes())?;
    }

    child.wait()?;

    Ok(())
}
