//! Storage abstraction layer for task, project, note, and resource persistence.
//!
//! | Type | Description |
//! |---|---|
//! | [`SqliteStorage`]   | Persists to a SQLite database in the OS data directory |
//! | [`JsonStorage`]     | Legacy JSON storage — kept for migration and testing |
//! | [`InMemoryStorage`] | Stores in memory — ideal for tests |

use crate::models::{Note, Project, Resource, Task};
use anyhow::Result;

/// Trait defining storage operations for tasks, projects, notes, and resources.
pub trait Storage {
    // ── tasks ─────────────────────────────────────────────────────────────────

    /// Load all tasks from storage.
    fn load(&self) -> Result<Vec<Task>>;

    /// Persist all tasks (upsert by UUID).
    fn save(&self, tasks: &[Task]) -> Result<()>;

    // ── projects ──────────────────────────────────────────────────────────────

    /// Load all projects from storage.
    fn load_projects(&self) -> Result<Vec<Project>>;

    /// Persist all projects (upsert by UUID).
    fn save_projects(&self, projects: &[Project]) -> Result<()>;

    // ── notes ─────────────────────────────────────────────────────────────────

    /// Load all notes from storage.
    fn load_notes(&self) -> Result<Vec<Note>>;

    /// Persist all notes (upsert by UUID).
    fn save_notes(&self, notes: &[Note]) -> Result<()>;

    // ── resources ─────────────────────────────────────────────────────────────

    /// Load all resources from storage.
    fn load_resources(&self) -> Result<Vec<Resource>>;

    /// Persist all resources (upsert by UUID).
    fn save_resources(&self, resources: &[Resource]) -> Result<()>;

    // ── combined ──────────────────────────────────────────────────────────────

    /// Load tasks, projects, and notes in a single call.
    fn load_all(&self) -> Result<(Vec<Task>, Vec<Project>, Vec<Note>)> {
        Ok((self.load()?, self.load_projects()?, self.load_notes()?))
    }

    /// Load everything including resources.
    #[allow(clippy::type_complexity)]
    fn load_all_with_resources(
        &self,
    ) -> Result<(Vec<Task>, Vec<Project>, Vec<Note>, Vec<Resource>)> {
        Ok((
            self.load()?,
            self.load_projects()?,
            self.load_notes()?,
            self.load_resources()?,
        ))
    }

    /// Persist tasks, projects, and notes in a single call.
    fn save_all(&self, tasks: &[Task], projects: &[Project], notes: &[Note]) -> Result<()> {
        self.save(tasks)?;
        self.save_projects(projects)?;
        self.save_notes(notes)
    }

    /// Returns a human-readable description of the storage location.
    #[allow(dead_code)]
    fn location(&self) -> String;
}

pub mod backup;
pub mod memory;
pub mod sqlite;

pub use memory::InMemoryStorage;
pub use sqlite::{SqliteStorage, get_db_path};
