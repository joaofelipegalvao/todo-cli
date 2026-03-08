// src/storage/json.rs

//! JSON file-based storage implementation.
//!
//! ## File format
//!
//! New format (v2):
//! ```json
//! {
//!   "tasks":     [ ... ],
//!   "projects":  [ ... ],
//!   "notes":     [ ... ],
//!   "resources": [ ... ]
//! }
//! ```
//!
//! ## Platform paths
//!
//! | Platform | Path |
//! |----------|------|
//! | Linux    | `~/.local/share/rustodo/todos.json` |
//! | macOS    | `~/Library/Application Support/rustodo/todos.json` |
//! | Windows  | `%APPDATA%\rustodo\todos.json` |
//!
//! Writes are atomic (write to `.tmp`, then `rename`) and protected by an
//! exclusive file lock (`.lock`) to guard against concurrent access.

use super::Storage;
use crate::models::{Note, Project, Resource, Task};
use anyhow::{Context, Result};
use directories::ProjectDirs;
use fs4::fs_std::FileExt;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
};

// ── on-disk envelope ──────────────────────────────────────────────────────────

/// The v2 JSON envelope stored on disk.
#[derive(Serialize, Deserialize, Default)]
struct StorageFile {
    #[serde(default)]
    tasks: Vec<Task>,
    #[serde(default)]
    projects: Vec<Project>,
    #[serde(default)]
    notes: Vec<Note>,
    #[serde(default)]
    resources: Vec<Resource>,
}

// ── JsonStorage ───────────────────────────────────────────────────────────────

pub struct JsonStorage {
    file_path: PathBuf,
}

impl JsonStorage {
    pub fn new() -> Result<Self> {
        let file_path = get_data_file_path()?;
        Ok(Self { file_path })
    }

    #[cfg(test)]
    pub fn with_path(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    // ── internal helpers ──────────────────────────────────────────────────────

    /// Read the file and return the parsed envelope.
    fn read_file(&self) -> Result<StorageFile> {
        let content = match fs::read_to_string(&self.file_path) {
            Ok(s) => s,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(StorageFile::default());
            }
            Err(e) => {
                return Err(e).context(format!("Failed to read {}", self.file_path.display()));
            }
        };

        let trimmed = content.trim();
        let envelope: StorageFile =
            serde_json::from_str(trimmed).context("Failed to parse todos.json")?;

        Ok(envelope)
    }

    /// Write the envelope atomically with file locking.
    fn write_file(&self, envelope: &StorageFile) -> Result<()> {
        let json =
            serde_json::to_string_pretty(envelope).context("Failed to serialize storage file")?;

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
        fs::write(&tmp_path, &json)
            .context(format!("Failed to write to {}", tmp_path.display()))?;
        fs::rename(&tmp_path, &self.file_path).context(format!(
            "Failed to rename {} to {}",
            tmp_path.display(),
            self.file_path.display()
        ))?;

        Ok(())
    }
}

// ── Storage impl ──────────────────────────────────────────────────────────────

impl Storage for JsonStorage {
    fn load(&self) -> Result<Vec<Task>> {
        Ok(self.read_file()?.tasks)
    }

    fn save(&self, tasks: &[Task]) -> Result<()> {
        let mut envelope = self.read_file()?;
        envelope.tasks = tasks.to_vec();
        self.write_file(&envelope)
    }

    fn load_projects(&self) -> Result<Vec<Project>> {
        Ok(self.read_file()?.projects)
    }

    fn save_projects(&self, projects: &[Project]) -> Result<()> {
        let mut envelope = self.read_file()?;
        envelope.projects = projects.to_vec();
        self.write_file(&envelope)
    }

    fn load_notes(&self) -> Result<Vec<Note>> {
        Ok(self.read_file()?.notes)
    }

    fn save_notes(&self, notes: &[Note]) -> Result<()> {
        let mut envelope = self.read_file()?;
        envelope.notes = notes.to_vec();
        self.write_file(&envelope)
    }

    fn load_resources(&self) -> Result<Vec<Resource>> {
        Ok(self.read_file()?.resources)
    }

    fn save_resources(&self, resources: &[Resource]) -> Result<()> {
        let mut envelope = self.read_file()?;
        envelope.resources = resources.to_vec();
        self.write_file(&envelope)
    }

    fn location(&self) -> String {
        self.file_path.display().to_string()
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

pub fn get_data_file_path() -> Result<PathBuf> {
    let data_dir = if let Ok(dir) = std::env::var("RUSTODO_DATA_DIR") {
        PathBuf::from(dir)
    } else {
        let proj_dirs =
            ProjectDirs::from("", "", "rustodo").context("Could not determine data directory")?;
        proj_dirs.data_dir().to_path_buf()
    };
    fs::create_dir_all(&data_dir).context(format!(
        "Failed to create data directory: {}",
        data_dir.display()
    ))?;
    Ok(data_dir.join("todos.json"))
}

/// Parses a `todos.json` string into a list of tasks.
pub fn parse_tasks_from_str(json: &str) -> Result<Vec<Task>> {
    let trimmed = json.trim();
    #[derive(serde::Deserialize)]
    struct Envelope {
        #[serde(default)]
        tasks: Vec<Task>,
    }
    let envelope: Envelope =
        serde_json::from_str(trimmed).context("Failed to parse v2 envelope")?;
    Ok(envelope.tasks)
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Priority;
    use tempfile::TempDir;

    fn make_storage() -> (JsonStorage, TempDir) {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("todos.json");
        (JsonStorage::with_path(path), temp)
    }

    #[test]
    fn test_save_and_load_tasks() {
        let (storage, _tmp) = make_storage();
        let tasks = vec![Task::new(
            "Buy milk".into(),
            Priority::Medium,
            vec![],
            None,
            None,
            None,
        )];
        storage.save(&tasks).unwrap();
        let loaded = storage.load().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].text, "Buy milk");
    }

    #[test]
    fn test_save_and_load_projects() {
        let (storage, _tmp) = make_storage();
        let projects = vec![Project::new("Backend".into())];
        storage.save_projects(&projects).unwrap();
        let loaded = storage.load_projects().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].name, "Backend");
    }

    #[test]
    fn test_save_and_load_notes() {
        let (storage, _tmp) = make_storage();
        let mut note = Note::new("Documentação do projeto".into());
        note.title = Some("Intro".into());
        note.language = Some("Rust".into());
        storage.save_notes(&[note]).unwrap();
        let loaded = storage.load_notes().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].body, "Documentação do projeto");
        assert_eq!(loaded[0].language.as_deref(), Some("Rust"));
    }

    #[test]
    fn test_save_and_load_resources() {
        let (storage, _tmp) = make_storage();
        let mut resource = Resource::new("sqlx docs".into());
        resource.url = Some("https://docs.rs/sqlx".into());
        resource.tags = vec!["rust".into(), "db".into()];
        storage.save_resources(&[resource]).unwrap();
        let loaded = storage.load_resources().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].title, "sqlx docs");
        assert_eq!(loaded[0].url.as_deref(), Some("https://docs.rs/sqlx"));
        assert_eq!(loaded[0].tags, vec!["rust", "db"]);
    }

    #[test]
    fn test_saving_resources_preserves_everything_else() {
        let (storage, _tmp) = make_storage();
        storage
            .save(&[Task::new(
                "T".into(),
                Priority::Medium,
                vec![],
                None,
                None,
                None,
            )])
            .unwrap();
        storage.save_projects(&[Project::new("P".into())]).unwrap();
        storage.save_notes(&[Note::new("N".into())]).unwrap();
        storage
            .save_resources(&[Resource::new("R".into())])
            .unwrap();

        assert_eq!(storage.load().unwrap().len(), 1);
        assert_eq!(storage.load_projects().unwrap().len(), 1);
        assert_eq!(storage.load_notes().unwrap().len(), 1);
        assert_eq!(storage.load_resources().unwrap().len(), 1);
    }

    #[test]
    fn test_note_links_to_resources() {
        let (storage, _tmp) = make_storage();
        let r1 = Resource::new("sqlx docs".into());
        let r2 = Resource::new("tokio docs".into());
        let (r1_uuid, r2_uuid) = (r1.uuid, r2.uuid);

        let mut note = Note::new("Async DB setup".into());
        note.add_resource(r1_uuid);
        note.add_resource(r2_uuid);

        storage.save_resources(&[r1, r2]).unwrap();
        storage.save_notes(&[note]).unwrap();

        let notes = storage.load_notes().unwrap();
        assert!(notes[0].references_resource(r1_uuid));
        assert!(notes[0].references_resource(r2_uuid));
        assert_eq!(notes[0].resource_ids.len(), 2);
    }

    #[test]
    fn test_existing_notes_without_resource_ids_deserialise_cleanly() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("todos.json");
        // Write a note without resource_ids — simulates a pre-Resource note
        fs::write(
            &path,
            r#"{
                "tasks": [],
                "projects": [],
                "notes": [
                    {
                        "uuid": "550e8400-e29b-41d4-a716-446655440000",
                        "body": "Old note without resource_ids",
                        "created_at": "2025-01-01T00:00:00Z"
                    }
                ]
            }"#,
        )
        .unwrap();

        let storage = JsonStorage::with_path(path);
        let notes = storage.load_notes().unwrap();
        assert_eq!(notes.len(), 1);
        // resource_ids must default to empty — no panic, no migration needed
        assert!(notes[0].resource_ids.is_empty());
    }

    #[test]
    fn test_saving_notes_preserves_tasks_and_projects() {
        let (storage, _tmp) = make_storage();
        storage
            .save(&[Task::new(
                "T".into(),
                Priority::Medium,
                vec![],
                None,
                None,
                None,
            )])
            .unwrap();
        storage.save_projects(&[Project::new("P".into())]).unwrap();
        storage.save_notes(&[Note::new("N".into())]).unwrap();

        assert_eq!(storage.load().unwrap().len(), 1);
        assert_eq!(storage.load_projects().unwrap().len(), 1);
        assert_eq!(storage.load_notes().unwrap().len(), 1);
    }

    #[test]
    fn test_note_linked_to_project_persists() {
        let (storage, _tmp) = make_storage();
        let project = Project::new("MeuProjeto".into());
        let p_uuid = project.uuid;

        let mut note = Note::new("Nota do projeto".into());
        note.project_id = Some(p_uuid);

        storage.save_projects(&[project]).unwrap();
        storage.save_notes(&[note]).unwrap();

        let notes = storage.load_notes().unwrap();
        assert_eq!(notes[0].project_id, Some(p_uuid));
    }

    #[test]
    fn test_tasks_and_projects_coexist() {
        let (storage, _tmp) = make_storage();
        let project = Project::new("P1".into());
        let tasks = vec![Task::new(
            "A".into(),
            Priority::Low,
            vec![],
            Some(project.uuid),
            None,
            None,
        )];
        let projects = vec![project];
        storage.save(&tasks).unwrap();
        storage.save_projects(&projects).unwrap();

        let t = storage.load().unwrap();
        let p = storage.load_projects().unwrap();
        assert_eq!(t.len(), 1);
        assert_eq!(p.len(), 1);
        assert_eq!(t[0].project_id, Some(p[0].uuid));
    }

    #[test]
    fn test_saving_tasks_preserves_projects() {
        let (storage, _tmp) = make_storage();
        storage
            .save_projects(&[Project::new("Keep me".into())])
            .unwrap();
        storage
            .save(&[Task::new(
                "T".into(),
                Priority::Medium,
                vec![],
                None,
                None,
                None,
            )])
            .unwrap();
        let projects = storage.load_projects().unwrap();
        assert_eq!(projects.len(), 1, "projects must survive a task-only save");
    }

    #[test]
    fn test_saving_projects_preserves_tasks() {
        let (storage, _tmp) = make_storage();
        storage
            .save(&[Task::new(
                "Keep me".into(),
                Priority::Medium,
                vec![],
                None,
                None,
                None,
            )])
            .unwrap();
        storage.save_projects(&[Project::new("P".into())]).unwrap();
        let tasks = storage.load().unwrap();
        assert_eq!(tasks.len(), 1, "tasks must survive a project-only save");
    }

    #[test]
    fn test_empty_file_returns_defaults() {
        let (storage, _tmp) = make_storage();
        assert!(storage.load().unwrap().is_empty());
        assert!(storage.load_projects().unwrap().is_empty());
        assert!(storage.load_notes().unwrap().is_empty());
        assert!(storage.load_resources().unwrap().is_empty());
    }
}
