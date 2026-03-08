//! Resource — an independent external reference (link, doc, asset) that can be
//! attached to one or more [`Note`]s.
//!
//! Resources are first-class citizens: they exist independently of any other
//! entity. The association with a Note is stored on the Note side via
//! `resource_ids: Vec<Uuid>`.

use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── ResourceType ──────────────────────────────────────────────────────────────

/// The kind of external reference a [`Resource`] represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ResourceType {
    /// Official documentation (docs.rs, MDN, etc.)
    Docs,
    /// Blog post or tutorial.
    Article,
    /// Video content (YouTube, etc.)
    Video,
    /// Source code repository (GitHub, GitLab, etc.)
    Repo,
    /// A Rust crate on crates.io.
    Crate,
    /// A book or long-form reference.
    Book,
    /// An RFC or formal specification.
    Spec,
    /// A development tool (Docker, Postman, etc.)
    Tool,
}

impl ResourceType {
    pub fn label(self) -> &'static str {
        match self {
            ResourceType::Docs => "docs",
            ResourceType::Article => "article",
            ResourceType::Video => "video",
            ResourceType::Repo => "repo",
            ResourceType::Crate => "crate",
            ResourceType::Book => "book",
            ResourceType::Spec => "spec",
            ResourceType::Tool => "tool",
        }
    }
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

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

    /// The kind of resource (docs, article, repo, etc.)
    #[serde(default)]
    pub resource_type: Option<ResourceType>,

    /// The external URL or file path this resource points to.
    #[serde(default)]
    pub url: Option<String>,

    /// Optional description or notes about this resource.
    #[serde(default)]
    pub description: Option<String>,

    /// Tags for filtering and categorisation.
    #[serde(default)]
    pub tags: Vec<String>,

    /// Timestamp when the resource was created (UTC).
    pub created_at: DateTime<Utc>,

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
            resource_type: None,
            url: None,
            description: None,
            tags: Vec::new(),
            created_at: Utc::now(),
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
