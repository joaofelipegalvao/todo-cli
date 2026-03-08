//! Note — a free-form documentation entity that can optionally link to a
//! [`Project`], a [`Task`], and/or one or more [`Resource`]s.
//!
//! Notes are first-class citizens: they exist independently and are only
//! associated with other entities when the user explicitly sets `project_id`,
//! `task_id`, or `resource_ids`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Note ──────────────────────────────────────────────────────────────────────

/// A free-form documentation note.
///
/// # Relationships
/// - `project_id`   → links to a [`Project`]          (optional, one)
/// - `task_id`      → links to a [`Task`]              (optional, one)
/// - `resource_ids` → links to one or more [`Resource`]s (optional, many)
///
/// All can be set simultaneously, or none.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    /// Stable unique identifier.
    #[serde(default = "Uuid::new_v4")]
    pub uuid: Uuid,

    /// Short title to identify the note (optional).
    #[serde(default)]
    pub title: Option<String>,

    /// The main content — free-form text / documentation.
    pub body: String,

    /// Tags for filtering and categorisation.
    #[serde(default)]
    pub tags: Vec<String>,

    /// Programming language this note relates to (e.g. "Rust", "Python").
    #[serde(default)]
    pub language: Option<String>,

    /// Optional link to a Project.
    #[serde(default)]
    pub project_id: Option<Uuid>,

    /// Optional link to a Task.
    #[serde(default)]
    pub task_id: Option<Uuid>,

    /// Links to zero or more Resources.
    ///
    /// Existing notes without this field deserialise with an empty `Vec`
    /// automatically via `#[serde(default)]` — no migration required.
    #[serde(default)]
    pub resource_ids: Vec<Uuid>,

    /// Timestamp when the note was created (UTC).
    pub created_at: DateTime<Utc>,

    /// Last modification timestamp.
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,

    /// Soft-deletion timestamp — `None` means not deleted.
    #[serde(default)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Note {
    /// Create a new note with just a body. All optional fields default to `None` / empty.
    pub fn new(body: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            title: None,
            body,
            tags: Vec::new(),
            language: None,
            project_id: None,
            task_id: None,
            resource_ids: Vec::new(),
            created_at: Utc::now(),
            updated_at: Some(Utc::now()),
            deleted_at: None,
        }
    }

    /// Update the last-modified timestamp.
    pub fn touch(&mut self) {
        self.updated_at = Some(Utc::now());
    }

    /// Soft-delete the note.
    pub fn soft_delete(&mut self) {
        self.deleted_at = Some(Utc::now());
        self.touch();
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    /// Returns `true` if the note is linked to the given project UUID.
    pub fn belongs_to_project(&self, project_id: Uuid) -> bool {
        self.project_id == Some(project_id)
    }

    /// Returns `true` if the note is linked to the given task UUID.
    pub fn belongs_to_task(&self, task_id: Uuid) -> bool {
        self.task_id == Some(task_id)
    }

    /// Returns `true` if the note references the given resource UUID.
    pub fn references_resource(&self, resource_id: Uuid) -> bool {
        self.resource_ids.contains(&resource_id)
    }

    /// Attach a resource. No-op if already present.
    pub fn add_resource(&mut self, resource_id: Uuid) {
        if !self.resource_ids.contains(&resource_id) {
            self.resource_ids.push(resource_id);
        }
    }

    /// Detach a resource. No-op if not present.
    pub fn remove_resource(&mut self, resource_id: Uuid) {
        self.resource_ids.retain(|id| *id != resource_id);
    }
}
