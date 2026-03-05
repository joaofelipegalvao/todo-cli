//! Resource — an independent external reference (link, doc, asset) that can be
//! attached to one or more [`Note`]s.
//!
//! Resources are first-class citizens: they exist independently of any other
//! entity. The association with a Note is stored on the Note side via
//! `resource_ids: Vec<Uuid>`.

use chrono::{DateTime, Local, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Resource ──────────────────────────────────────────────────────────────────

/// An independent external reference — a URL, documentation link, or asset.
///
/// # Relationships
///
/// Resources have no direct links to other entities. Association is done from
/// the [`Note`] side via `Note::resource_ids`.
///
/// ```text
/// Note.resource_ids  ──────────────→  Resource.uuid
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    /// Stable unique identifier.
    #[serde(default = "Uuid::new_v4")]
    pub uuid: Uuid,

    /// Human-readable title (e.g. "sqlx docs", "RFC 7231").
    pub title: String,

    /// The external URL or file path this resource points to.
    #[serde(default)]
    pub url: Option<String>,

    /// Optional description or notes about this resource.
    #[serde(default)]
    pub description: Option<String>,

    /// Tags for filtering and categorisation.
    #[serde(default)]
    pub tags: Vec<String>,

    /// When the resource was created.
    pub created_at: NaiveDate,

    /// Last modification timestamp.
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,

    /// Soft-deletion timestamp — `None` means not deleted.
    #[serde(default)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Resource {
    /// Create a new resource with just a title. All optional fields default to `None`.
    pub fn new(title: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            title,
            url: None,
            description: None,
            tags: Vec::new(),
            created_at: Local::now().naive_local().date(),
            updated_at: Some(Utc::now()),
            deleted_at: None,
        }
    }

    /// Update the last-modified timestamp.
    pub fn touch(&mut self) {
        self.updated_at = Some(Utc::now());
    }

    /// Soft-delete this resource.
    pub fn soft_delete(&mut self) {
        self.deleted_at = Some(Utc::now());
        self.touch();
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}
