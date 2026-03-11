//! Handler for `todo search <QUERY>`.
//!
//! Case-insensitive substring search over tasks, notes, projects, and resources.
//! Searches across: task text, note title/body, project name/tech, resource title/url/description.
//!
//! Filter behaviour:
//! - `--project` filters tasks, notes, and projects (resources have no project_id).
//! - `--tag`     filters tasks, notes, and resources (projects have no tags — hidden when --tag is passed).
//! - `--status`  filters tasks only.

use anyhow::Result;
use colored::Colorize;
use uuid::Uuid;

use crate::error::TodoError;
use crate::models::StatusFilter;
use crate::render::display_lists;
use crate::render::note_table::display_notes;
use crate::render::project_table::display_projects;
use crate::render::resource_table::display_resources;
use crate::storage::Storage;

pub fn execute(
    storage: &impl Storage,
    query: String,
    tags: Vec<String>,
    project: Option<String>,
    status: StatusFilter,
) -> Result<()> {
    let (all_tasks, projects, all_notes, all_resources) = storage.load_all_with_resources()?;

    let q = query.to_lowercase();

    // ── Resolve project UUID ───────────────────────────────────────────────────
    // If --project was supplied but not found, bail out immediately.
    let proj_uuid: Option<Uuid> = if let Some(ref project_name) = project {
        let uuid = projects
            .iter()
            .find(|p| p.name.to_lowercase() == project_name.to_lowercase() && !p.is_deleted())
            .map(|p| p.uuid);

        if uuid.is_none() {
            return Err(TodoError::ProjectNotFound(project_name.clone()).into());
        }

        uuid
    } else {
        None
    };

    // ── Shared filter helpers ──────────────────────────────────────────────────
    let matches_tags =
        |item_tags: &Vec<String>| tags.is_empty() || tags.iter().all(|tag| item_tags.contains(tag));

    let matches_proj =
        |item_proj: Option<Uuid>| proj_uuid.is_none_or(|uuid| item_proj == Some(uuid));

    // ── Tasks ─────────────────────────────────────────────────────────────────
    let visible_tasks: Vec<_> = all_tasks
        .iter()
        .filter(|t| !t.is_deleted())
        .cloned()
        .collect();

    let task_results: Vec<(usize, &_)> = visible_tasks
        .iter()
        .enumerate()
        .filter(|(_, t)| t.text.to_lowercase().contains(&q))
        .filter(|(_, t)| t.matches_status(status))
        .filter(|(_, t)| matches_tags(&t.tags))
        .filter(|(_, t)| proj_uuid.is_none_or(|uuid| t.project_id == Some(uuid)))
        .map(|(i, t)| (i + 1, t))
        .collect();

    // ── Notes ─────────────────────────────────────────────────────────────────
    let note_results: Vec<&_> = all_notes
        .iter()
        .filter(|n| !n.is_deleted())
        .filter(|n| {
            n.title.as_deref().unwrap_or("").to_lowercase().contains(&q)
                || n.body.to_lowercase().contains(&q)
                || n.tags.iter().any(|t| t.to_lowercase().contains(&q))
                || n.language
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(&q)
        })
        .filter(|n| matches_proj(n.project_id))
        .filter(|n| matches_tags(&n.tags))
        .collect();

    // ── Projects ──────────────────────────────────────────────────────────────
    // Projects have no tags — hidden entirely when --tag is passed.
    let project_results: Vec<&_> = if !tags.is_empty() {
        vec![]
    } else {
        projects
            .iter()
            .filter(|p| !p.is_deleted())
            .filter(|p| {
                p.name.to_lowercase().contains(&q)
                    || p.tech.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .filter(|p| proj_uuid.is_none_or(|uuid| p.uuid == uuid))
            .collect()
    };

    // ── Resources ─────────────────────────────────────────────────────────────
    // Resources have no project_id — project filter does not apply.
    let resource_results: Vec<&_> = all_resources
        .iter()
        .filter(|r| !r.is_deleted())
        .filter(|r| {
            r.title.to_lowercase().contains(&q)
                || r.url.as_deref().unwrap_or("").to_lowercase().contains(&q)
                || r.description
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(&q)
                || r.tags.iter().any(|t| t.to_lowercase().contains(&q))
        })
        .filter(|r| matches_tags(&r.tags))
        .collect();

    if task_results.is_empty()
        && note_results.is_empty()
        && project_results.is_empty()
        && resource_results.is_empty()
    {
        return Err(TodoError::NoSearchResults(query).into());
    }

    // ── Render ────────────────────────────────────────────────────────────────
    let found_total =
        task_results.len() + note_results.len() + project_results.len() + resource_results.len();

    println!(
        "\nSearch results for \"{}\"  ({})\n",
        query,
        format!("{} found", found_total).dimmed()
    );

    if !task_results.is_empty() {
        let title = format!("Tasks  ({})", task_results.len());
        display_lists(
            &task_results,
            &title,
            &visible_tasks,
            &projects,
            &all_notes,
            &all_resources,
        );
    }

    if !project_results.is_empty() {
        display_projects(&project_results, &all_tasks, &all_notes);
    }

    if !note_results.is_empty() {
        display_notes(&note_results, &projects, &all_resources);
    }

    if !resource_results.is_empty() {
        display_resources(&resource_results, &all_notes);
    }

    Ok(())
}
