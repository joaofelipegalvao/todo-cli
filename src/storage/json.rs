// src/storage/json.rs

//! JSON file-based storage implementation.
//!
//! Tasks are stored as a pretty-printed JSON array at a platform-specific
//! path resolved by [`get_data_file_path`]:
//!
//! | Platform | Path |
//! |----------|------|
//! | Linux | `~/.local/share/rustodo/todos.json` |
//! | macOS | `~/Library/Application Support/rustodo/todos.json` |
//! | Windows | `%APPDATA%\rustodo\todos.json` |
//!
//! The directory is created automatically on first use.
//!
//!  Writes are atomic (via a `.tmp` file and `rename`) and protected against
//! concurrent access via an exclusive file lock (`.lock`).

use anyhow::{Context, Result};
use directories::ProjectDirs;
use fs4::fs_std::FileExt;
use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
};

use super::Storage;
use crate::models::Task;
use uuid::Uuid;

/// JSON file-based storage implementation
pub struct JsonStorage {
    file_path: PathBuf,
}

impl JsonStorage {
    /// Creates a new [`JsonStorage`] pointing to the default OS data directory.
    ///
    /// The data directory is created automatically if it does not exist.
    ///
    /// # Errors
    ///
    /// Returns an error if the OS data directory cannot be determined or
    /// if the directory cannot be created.
    pub fn new() -> Result<Self> {
        let file_path = get_data_file_path()?;
        Ok(Self { file_path })
    }

    /// Creates a [`JsonStorage`] at an arbitrary path.
    ///
    /// Intended for use in tests where a [`tempfile::TempDir`] provides an
    /// isolated, automatically-cleaned directory.
    #[cfg(test)]
    pub fn with_path(file_path: PathBuf) -> Self {
        Self { file_path }
    }
}

impl Storage for JsonStorage {
    /// Loads tasks from the JSON file.
    ///
    /// Automatically migrates tasks without UUIDs by generating and saving them.
    ///
    /// Returns an empty list if the file does not exist yet.
    fn load(&self) -> Result<Vec<Task>> {
        // ── 1. Load from disk ────────────────────────────────────────────
        let mut tasks: Vec<Task> = match fs::read_to_string(&self.file_path) {
            Ok(content) => serde_json::from_str(&content)
                .context("Failed to parse todos.json - file may be corrupted")?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(e) => {
                return Err(e).context(format!(
                    "Failed to read todos.json from: {}",
                    self.file_path.display()
                ));
            }
        };

        let mut modified = false;
        for task in &mut tasks {
            if task.uuid.is_nil() {
                task.uuid = Uuid::new_v4();
                modified = true;
            }
        }

        if modified {
            self.save(&tasks)?;
        }

        Ok(tasks)
    }

    /// Persists tasks atomically using a write-rename strategy.
    ///
    /// Acquires an exclusive lock on `todos.json.lock` before writing,
    /// ensuring concurrent processes do not corrupt the file.
    /// The lock is released automatically when the function returns.
    fn save(&self, tasks: &[Task]) -> Result<()> {
        let json =
            serde_json::to_string_pretty(tasks).context("Failed to serialize tasks to JSON")?;

        let lock_path = self.file_path.with_added_extension("lock");
        let lock_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(&lock_path)
            .context("Failed to create lock file")?;

        lock_file
            .lock_exclusive()
            .context("Failed to acquire lock")?;

        let tmp_path = self.file_path.with_added_extension("tmp");

        fs::write(&tmp_path, &json).context(format!(
            "Failed to write to {} - check file permissions",
            tmp_path.display()
        ))?;

        fs::rename(&tmp_path, &self.file_path).context(format!(
            "Failed to rename {} to {}",
            tmp_path.display(),
            self.file_path.display()
        ))?;

        Ok(())
    }

    fn location(&self) -> String {
        self.file_path.display().to_string()
    }
}

/// Returns the path to the todos.json file (re-exported for compatibility)
pub fn get_data_file_path() -> Result<PathBuf> {
    // Allow overriding the data directory via environment variable
    if let Ok(dir) = std::env::var("RUSTODO_DATA_DIR") {
        let data_dir = PathBuf::from(dir);
        fs::create_dir_all(&data_dir).context(format!(
            "Failed to create data directory: {}",
            data_dir.display()
        ))?;
        return Ok(data_dir.join("todos.json"));
    }

    let project_dirs =
        ProjectDirs::from("", "", "rustodo").context("Failed to determine project directories")?;

    let data_dir = project_dirs.data_dir();
    fs::create_dir_all(data_dir).context(format!(
        "Failed to create data directory: {}",
        data_dir.display()
    ))?;

    let mut path = data_dir.to_path_buf();
    path.push("todos.json");

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Priority;
    use tempfile::TempDir;

    #[test]
    fn test_json_storage_save_and_load() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("test.json");

        let storage = JsonStorage::with_path(path.clone());

        let tasks = vec![crate::models::Task::new(
            "Test task".to_string(),
            Priority::Medium,
            vec![],
            None,
            None,
            None,
        )];
        storage.save(&tasks).unwrap();

        assert!(path.exists());

        let loaded = storage.load().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].text, "Test task");
        assert!(!loaded[0].uuid.is_nil(), "UUID should be generated");
    }

    #[test]
    fn test_json_storage_empty_file() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("empty.json");

        let storage = JsonStorage::with_path(path);

        let tasks = storage.load().unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn test_uuid_migration_on_old_json() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("todos.json");

        // Simulate old JSON without UUIDs
        let old_json = r#"[
            {
                "text": "Old task",
                "completed": false,
                "priority": "medium",
                "tags": [],
                "created_at": "2025-01-01",
                "depends_on": []
            }
        ]"#;
        fs::write(&path, old_json).unwrap();

        let storage = JsonStorage::with_path(path.clone());

        // First load should migrate
        let tasks = storage.load().unwrap();
        assert_eq!(tasks.len(), 1);
        assert!(!tasks[0].uuid.is_nil(), "UUID should be generated");

        let first_uuid = tasks[0].uuid;

        // Second load should return SAME UUID (stability test)
        let tasks2 = storage.load().unwrap();
        assert_eq!(
            tasks2[0].uuid, first_uuid,
            "UUID must be stable across loads"
        );
    }

    #[test]
    fn test_uuid_stability_across_multiple_loads() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("todos.json");
        let storage = JsonStorage::with_path(path);

        // Create task
        let tasks = vec![Task::new(
            "Stable task".to_string(),
            Priority::Medium,
            vec![],
            None,
            None,
            None,
        )];
        storage.save(&tasks).unwrap();

        // Load 3 times — UUID must be identical
        let uuid1 = storage.load().unwrap()[0].uuid;
        let uuid2 = storage.load().unwrap()[0].uuid;
        let uuid3 = storage.load().unwrap()[0].uuid;

        assert_eq!(uuid1, uuid2);
        assert_eq!(uuid2, uuid3);
    }

    #[test]
    fn test_atomic_write_no_tmp_file_after_save() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("todos.json");
        let storage = JsonStorage::with_path(path.clone());

        let tasks = vec![Task::new(
            "Test task".to_string(),
            Priority::Medium,
            vec![],
            None,
            None,
            None,
        )];

        storage.save(&tasks).unwrap();

        let tmp_path = path.with_added_extension("tmp");
        assert!(
            !tmp_path.exists(),
            "The .tmp file must not remain after save."
        );

        let loaded = storage.load().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].text, "Test task");
    }

    #[test]
    fn test_atomic_write_preserves_original_on_corrupt_tmp() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("todos.json");
        let storage = JsonStorage::with_path(path.clone());

        let original = vec![Task::new(
            "Original".to_string(),
            Priority::Medium,
            vec![],
            None,
            None,
            None,
        )];
        storage.save(&original).unwrap();

        let tmp_path = path.with_added_extension("tmp");
        fs::write(&tmp_path, "corrupted JSON {{{").unwrap();

        let new_tasks = vec![Task::new(
            "Updated".to_string(),
            Priority::High,
            vec![],
            None,
            None,
            None,
        )];
        storage.save(&new_tasks).unwrap();

        assert!(!tmp_path.exists());

        let loaded = storage.load().unwrap();
        assert_eq!(loaded[0].text, "Updated");
    }

    #[test]
    fn test_lock_file_released_after_save() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("todos.json");
        let storage = JsonStorage::with_path(path.clone());

        let tasks = vec![Task::new(
            "Test task".to_string(),
            Priority::Medium,
            vec![],
            None,
            None,
            None,
        )];

        storage.save(&tasks).unwrap();

        let lock_path = path.with_added_extension("lock");
        let lock_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(&lock_path)
            .unwrap();

        assert!(lock_file.try_lock_exclusive().is_ok());
    }
}
