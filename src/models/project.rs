use anyhow::Result;
use chrono::{DateTime, Local, NaiveDate, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Difficulty ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    #[default]
    Medium,
    Hard,
}

impl Difficulty {
    pub fn label(self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Difficulty::Easy => Difficulty::Medium,
            Difficulty::Medium => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Easy,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Difficulty::Easy => Difficulty::Hard,
            Difficulty::Medium => Difficulty::Easy,
            Difficulty::Hard => Difficulty::Medium,
        }
    }
}

// ── Project ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    #[serde(default = "Uuid::new_v4")]
    pub uuid: Uuid,
    pub name: String,
    /// Whether the project has been completed.
    #[serde(default)]
    pub completed: bool,
    #[serde(default)]
    pub difficulty: Difficulty,
    #[serde(default)]
    pub tech: Vec<String>,
    #[serde(default)]
    pub due_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub completed_at: Option<NaiveDate>,
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Project {
    pub fn new(name: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            completed: false,
            difficulty: Difficulty::Medium,
            tech: Vec::new(),
            due_date: None,
            created_at: Utc::now(),
            completed_at: None,
            updated_at: Some(Utc::now()),
            deleted_at: None,
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = Some(Utc::now());
    }

    pub fn mark_done(&mut self) {
        self.completed = true;
        self.completed_at = Some(Local::now().naive_local().date());
        self.touch();
    }

    pub fn mark_undone(&mut self) {
        self.completed = false;
        self.completed_at = None;
        self.touch();
    }

    pub fn soft_delete(&mut self) {
        self.deleted_at = Some(Utc::now());
        self.touch();
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(due) = self.due_date {
            let today = Local::now().naive_local().date();
            due < today && !self.completed
        } else {
            false
        }
    }

    /// Finds a project by name (case-insensitive) or creates a new one.
    ///
    /// This is the single shared implementation used by every command handler
    /// that accepts `--project <NAME>` — `task add`, `task edit`, `note add`,
    /// `note edit`. All of them resolve a user-supplied string to a UUID via
    /// this function before writing to storage.
    ///
    /// # Behaviour
    ///
    /// - If a non-deleted project with `name` already exists → returns its UUID.
    /// - Otherwise → creates a new `Project`, appends it to `projects`, calls
    ///   `storage.save_projects`, and returns the new UUID.
    ///
    /// # Example
    ///
    /// ```
    /// # use rustodo::models::Project;
    /// # use rustodo::storage::{InMemoryStorage, Storage};
    /// let storage = InMemoryStorage::default();
    /// let projects = storage.load_projects().unwrap();
    ///
    /// // First call: "Backend" does not exist — creates it.
    /// let uuid1 = Project::resolve_or_create(&storage, &projects, "Backend").unwrap();
    ///
    /// // Second call with the same name: finds the existing one.
    /// let projects = storage.load_projects().unwrap();
    /// let uuid2 = Project::resolve_or_create(&storage, &projects, "backend").unwrap();
    ///
    /// assert_eq!(uuid1, uuid2);
    /// ```
    pub fn resolve_or_create(
        storage: &impl crate::storage::Storage,
        projects: &[Project],
        name: &str,
    ) -> Result<Uuid> {
        // Case-insensitive lookup among non-deleted projects
        if let Some(existing) = projects
            .iter()
            .find(|p| p.name.to_lowercase() == name.to_lowercase() && !p.is_deleted())
        {
            return Ok(existing.uuid);
        }

        // Project does not exist yet — create and persist it
        let new_project = Project::new(name.to_string());
        let uuid = new_project.uuid;
        let mut all = projects.to_vec();
        all.push(new_project);
        storage.save_projects(&all)?;

        Ok(uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{InMemoryStorage, Storage};

    #[test]
    fn test_resolve_creates_new_project() {
        let storage = InMemoryStorage::default();
        let projects = storage.load_projects().unwrap();

        let uuid = Project::resolve_or_create(&storage, &projects, "Backend").unwrap();

        let saved = storage.load_projects().unwrap();
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0].uuid, uuid);
        assert_eq!(saved[0].name, "Backend");
    }

    #[test]
    fn test_resolve_finds_existing_project() {
        let storage = InMemoryStorage::default();
        let project = Project::new("Backend".into());
        let expected_uuid = project.uuid;
        storage.save_projects(&[project]).unwrap();

        let projects = storage.load_projects().unwrap();
        let uuid = Project::resolve_or_create(&storage, &projects, "Backend").unwrap();

        assert_eq!(uuid, expected_uuid);
        // No new project was created
        assert_eq!(storage.load_projects().unwrap().len(), 1);
    }

    #[test]
    fn test_resolve_is_case_insensitive() {
        let storage = InMemoryStorage::default();
        let project = Project::new("Backend".into());
        let expected_uuid = project.uuid;
        storage.save_projects(&[project]).unwrap();

        let projects = storage.load_projects().unwrap();
        let uuid = Project::resolve_or_create(&storage, &projects, "backend").unwrap();

        assert_eq!(uuid, expected_uuid);
        assert_eq!(storage.load_projects().unwrap().len(), 1);
    }

    #[test]
    fn test_resolve_ignores_deleted_projects() {
        let storage = InMemoryStorage::default();
        let mut project = Project::new("Backend".into());
        project.soft_delete();
        storage.save_projects(&[project]).unwrap();

        let projects = storage.load_projects().unwrap();
        let uuid = Project::resolve_or_create(&storage, &projects, "Backend").unwrap();

        // Should have created a NEW project, not reused the deleted one
        let saved = storage.load_projects().unwrap();
        assert_eq!(saved.len(), 2);
        // The returned UUID belongs to the new (non-deleted) project
        let new_proj = saved.iter().find(|p| p.uuid == uuid).unwrap();
        assert!(!new_proj.is_deleted());
    }

    #[test]
    fn test_resolve_deduplicates_across_calls() {
        let storage = InMemoryStorage::default();

        let p1 = storage.load_projects().unwrap();
        let uuid1 = Project::resolve_or_create(&storage, &p1, "Work").unwrap();

        let p2 = storage.load_projects().unwrap();
        let uuid2 = Project::resolve_or_create(&storage, &p2, "WORK").unwrap();

        assert_eq!(uuid1, uuid2);
        assert_eq!(storage.load_projects().unwrap().len(), 1);
    }
}
