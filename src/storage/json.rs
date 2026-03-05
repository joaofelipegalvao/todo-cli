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
//! Old format (v1) — bare array — is detected and migrated automatically
//! on first load. The migrated file is immediately written back in v2 format.
//!
//! ## Migration: project name → project_id
//!
//! Tasks that have a legacy `"project": "string"` field but no `project_id`
//! are automatically migrated on first load:
//! - If a Project with that name (case-insensitive) already exists, the task
//!   is linked to it via `project_id`.
//! - Otherwise a new Project is created from the name and linked.
//!   The migrated file is immediately written back.
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

use anyhow::{Context, Result};
use directories::ProjectDirs;
use fs4::fs_std::FileExt;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
};
use uuid::Uuid;

use super::Storage;
use crate::models::{Note, Project, Resource, Task};

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
    /// Resources are zero-cost to add: files without this key deserialise with
    /// an empty Vec via `#[serde(default)]` — no explicit migration needed.
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
    ///
    /// Handles the following migration cases automatically:
    ///   1. File does not exist → empty envelope
    ///   2. File is a v1 bare array → migrate to v2, write back
    ///   3. Tasks have legacy `project: String` → resolve/create Project, fill `project_id`
    ///   4. Tasks, projects, notes, or resources have nil UUIDs → assign new UUIDs
    ///   5. Files without `resources` key → deserialise with empty Vec (zero migration cost)
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

        // v1: bare array — no projects, notes, or resources yet
        if trimmed.starts_with('[') {
            let tasks: Vec<Task> = serde_json::from_str(trimmed)
                .context("Failed to parse v1 todos.json (bare task array)")?;
            let mut envelope = StorageFile {
                tasks,
                projects: vec![],
                notes: vec![],
                resources: vec![],
            };
            Self::migrate_project_names(&mut envelope);
            self.write_file(&envelope)?;
            return Ok(envelope);
        }

        // v2: object
        let mut envelope: StorageFile =
            serde_json::from_str(trimmed).context("Failed to parse todos.json")?;

        let mut modified = false;

        // UUID migration for tasks
        for task in &mut envelope.tasks {
            if task.uuid.is_nil() {
                task.uuid = Uuid::new_v4();
                modified = true;
            }
        }

        // UUID migration for projects
        for project in &mut envelope.projects {
            if project.uuid.is_nil() {
                project.uuid = Uuid::new_v4();
                modified = true;
            }
        }

        // UUID migration for notes
        for note in &mut envelope.notes {
            if note.uuid.is_nil() {
                note.uuid = Uuid::new_v4();
                modified = true;
            }
        }

        // UUID migration for resources
        for resource in &mut envelope.resources {
            if resource.uuid.is_nil() {
                resource.uuid = Uuid::new_v4();
                modified = true;
            }
        }

        // Project name → project_id migration
        if Self::migrate_project_names(&mut envelope) {
            modified = true;
        }

        if modified {
            self.write_file(&envelope)?;
        }

        Ok(envelope)
    }

    /// Migrates tasks with `project_name_legacy` to `project_id`.
    ///
    /// Returns `true` if any migration was performed.
    fn migrate_project_names(envelope: &mut StorageFile) -> bool {
        let needs_migration: Vec<(usize, String)> = envelope
            .tasks
            .iter()
            .enumerate()
            .filter_map(|(i, t)| {
                if t.project_id.is_none() {
                    t.project_name_legacy.clone().map(|name| (i, name))
                } else {
                    None
                }
            })
            .collect();

        if needs_migration.is_empty() {
            return false;
        }

        for (task_idx, project_name) in needs_migration {
            let project_uuid = if let Some(existing) = envelope
                .projects
                .iter()
                .find(|p| p.name.to_lowercase() == project_name.to_lowercase())
            {
                existing.uuid
            } else {
                let new_project = Project::new(project_name);
                let uuid = new_project.uuid;
                envelope.projects.push(new_project);
                uuid
            };

            envelope.tasks[task_idx].project_id = Some(project_uuid);
        }

        true
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
    let proj_dirs =
        ProjectDirs::from("", "", "rustodo").context("Could not determine data directory")?;
    let data_dir = proj_dirs.data_dir();
    fs::create_dir_all(data_dir).context(format!(
        "Failed to create data directory: {}",
        data_dir.display()
    ))?;
    Ok(data_dir.join("todos.json"))
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
                        "created_at": "2025-01-01"
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
    fn test_v1_migration() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("todos.json");
        fs::write(&path, r#"[{"text":"Old","completed":false,"priority":"medium","tags":[],"created_at":"2025-01-01","depends_on":[]}]"#).unwrap();

        let storage = JsonStorage::with_path(path.clone());
        let tasks = storage.load().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].text, "Old");

        let content = fs::read_to_string(&path).unwrap();
        assert!(
            content.trim().starts_with('{'),
            "file must be migrated to v2 object format"
        );

        let projects = storage.load_projects().unwrap();
        assert!(projects.is_empty());
        let notes = storage.load_notes().unwrap();
        assert!(notes.is_empty());
        let resources = storage.load_resources().unwrap();
        assert!(resources.is_empty());
    }

    #[test]
    fn test_project_name_migration() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("todos.json");
        fs::write(&path, r#"{
            "tasks": [
                {"text":"Task A","completed":false,"priority":"medium","tags":[],"created_at":"2025-01-01","depends_on":[],"project":"Backend"},
                {"text":"Task B","completed":false,"priority":"medium","tags":[],"created_at":"2025-01-01","depends_on":[],"project":"Backend"},
                {"text":"Task C","completed":false,"priority":"medium","tags":[],"created_at":"2025-01-01","depends_on":[],"project":"Frontend"}
            ],
            "projects": []
        }"#).unwrap();

        let storage = JsonStorage::with_path(path.clone());
        let tasks = storage.load().unwrap();
        let projects = storage.load_projects().unwrap();

        assert_eq!(projects.len(), 2);
        assert!(tasks[0].project_id.is_some());
        assert_eq!(tasks[0].project_id, tasks[1].project_id);
        assert_ne!(tasks[0].project_id, tasks[2].project_id);

        let backend = projects.iter().find(|p| p.name == "Backend").unwrap();
        assert_eq!(tasks[0].project_id, Some(backend.uuid));
    }

    #[test]
    fn test_empty_file_returns_defaults() {
        let (storage, _tmp) = make_storage();
        assert!(storage.load().unwrap().is_empty());
        assert!(storage.load_projects().unwrap().is_empty());
        assert!(storage.load_notes().unwrap().is_empty());
        assert!(storage.load_resources().unwrap().is_empty());
    }

    #[test]
    fn test_uuid_migration() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("todos.json");
        fs::write(&path, r#"{"tasks":[{"text":"No UUID","completed":false,"priority":"medium","tags":[],"created_at":"2025-01-01","depends_on":[]}],"projects":[]}"#).unwrap();

        let storage = JsonStorage::with_path(path);
        let tasks = storage.load().unwrap();
        assert!(!tasks[0].uuid.is_nil());
    }
}
