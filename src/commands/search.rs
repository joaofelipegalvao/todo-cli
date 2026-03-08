//! Handler for `todo search <QUERY>`.
//!
//! Case-insensitive substring search over task descriptions, with optional
//! tag, project, and status filters. Delegates rendering to
//! [`display::display_lists`](crate::display::display_lists).

use anyhow::Result;

use crate::error::TodoError;
use crate::models::StatusFilter;
use crate::render::display_lists;
use crate::storage::Storage;

pub fn execute(
    storage: &impl Storage,
    query: String,
    tags: Vec<String>,
    project: Option<String>,
    status: StatusFilter,
) -> Result<()> {
    let (all_tasks, projects, notes) = storage.load_all()?;
    let resources = storage.load_resources()?;

    let visible: Vec<_> = all_tasks
        .iter()
        .filter(|t| !t.is_deleted())
        .cloned()
        .collect();

    let mut results: Vec<(usize, &_)> = visible
        .iter()
        .enumerate()
        .filter(|(_, task)| task.text.to_lowercase().contains(&query.to_lowercase()))
        .filter(|(_, task)| task.matches_status(status))
        .map(|(i, task)| (i + 1, task))
        .collect();

    // AND semantics: task must contain ALL specified tags
    if !tags.is_empty() {
        results.retain(|(_, task)| tags.iter().all(|tag| task.tags.contains(tag)));
    }

    // Filter by project: resolve name → UUID, then filter by project_id
    if let Some(ref project_name) = project {
        let proj_uuid = projects
            .iter()
            .find(|p| p.name.to_lowercase() == project_name.to_lowercase() && !p.is_deleted())
            .map(|p| p.uuid);
        results.retain(|(_, task)| proj_uuid.is_some() && task.project_id == proj_uuid);
    }

    if results.is_empty() {
        return Err(TodoError::NoSearchResults(query).into());
    }

    let title = format!("Search results for \"{}\"", query);
    display_lists(&results, &title, &visible, &projects, &notes, &resources);
    Ok(())
}
