//! Handler for `todo search <QUERY>`.

use anyhow::Result;

use crate::display::display_lists;
use crate::error::TodoError;
use crate::models::StatusFilter;
use crate::storage::Storage;

pub fn execute(
    storage: &impl Storage,
    query: String,
    tag: Option<String>,
    project: Option<String>,
    status: StatusFilter,
) -> Result<()> {
    let (all_tasks, projects, _) = storage.load_all()?;

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

    if let Some(tag_name) = &tag {
        results.retain(|(_, task)| task.tags.contains(tag_name));
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
    display_lists(&results, &title, &visible, &projects);
    Ok(())
}
