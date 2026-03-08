//! Handler for `todo project list`.

use anyhow::Result;

use crate::error::TodoError;
use crate::render::display_projects;
use crate::storage::Storage;

pub fn execute(storage: &impl Storage) -> Result<()> {
    let (tasks, projects, notes) = storage.load_all()?;

    let mut visible: Vec<_> = projects.iter().filter(|p| !p.is_deleted()).collect();

    if visible.is_empty() {
        return Err(TodoError::NoProjectsFound.into());
    }

    visible.sort_by(|a, b| a.name.cmp(&b.name));

    display_projects(&visible, &tasks, &notes);
    Ok(())
}
