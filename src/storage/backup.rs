//! Automatic rotating backup for the SQLite database.
//!
//! Uses `VACUUM INTO` to create a consistent snapshot without closing the
//! main connection. Backups are stored in `<data_dir>/backups/`.
//!
//! Rotation policy: keep the last `max_backups` files (default 10).
//! Backup is skipped if the last backup is newer than `min_interval_minutes`.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Checks whether a backup is needed and creates one if so.
///
/// Called after every write operation.
pub fn backup_if_needed(
    db_path: &Path,
    max_backups: usize,
    min_interval_minutes: u64,
) -> Result<()> {
    let backup_dir = db_path.parent().unwrap_or(Path::new(".")).join("backups");

    std::fs::create_dir_all(&backup_dir).context("Failed to create backups directory")?;

    // Check if last backup is recent enough to skip
    if let Some(last) = last_backup_time(&backup_dir) {
        let elapsed = std::time::SystemTime::now()
            .duration_since(last)
            .unwrap_or_default();
        if elapsed.as_secs() < min_interval_minutes * 60 {
            return Ok(());
        }
    }

    create_backup(db_path, &backup_dir)?;
    rotate_backups(&backup_dir, max_backups)?;

    Ok(())
}

/// Creates a manual backup regardless of interval.
pub fn create_backup(db_path: &Path, backup_dir: &Path) -> Result<PathBuf> {
    std::fs::create_dir_all(backup_dir).context("Failed to create backups directory")?;

    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let backup_path = backup_dir.join(format!("{}.db", timestamp));

    // VACUUM INTO creates a consistent snapshot — safe while DB is open
    let conn = rusqlite::Connection::open(db_path).context("Failed to open DB for backup")?;
    conn.execute(&format!("VACUUM INTO '{}'", backup_path.display()), [])
        .context("VACUUM INTO failed")?;

    Ok(backup_path)
}

/// Removes oldest backups keeping only `max_backups`.
fn rotate_backups(backup_dir: &Path, max_backups: usize) -> Result<()> {
    let mut backups: Vec<PathBuf> = std::fs::read_dir(backup_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map_or(false, |e| e == "db"))
        .collect();

    backups.sort();

    while backups.len() > max_backups {
        if let Some(oldest) = backups.first() {
            std::fs::remove_file(oldest).context("Failed to remove old backup")?;
            backups.remove(0);
        }
    }

    Ok(())
}

/// Returns the modification time of the most recent backup file.
fn last_backup_time(backup_dir: &Path) -> Option<std::time::SystemTime> {
    std::fs::read_dir(backup_dir)
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "db"))
        .filter_map(|e| e.metadata().ok()?.modified().ok())
        .max()
}
