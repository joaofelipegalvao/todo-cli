//! Tag aggregation service.
//!
//! Pure domain logic — no CLI, no storage, no I/O.
//! Collects and counts tags across all taggable entities:
//! [`Task`], [`Note`], and [`Resource`].

use crate::models::{Note, Resource, Task};
use std::collections::HashMap;

// ── TagStat ───────────────────────────────────────────────────────────────────

/// Aggregated tag statistics across all entity types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagStat {
    /// The tag name.
    pub name: String,
    /// Number of non-deleted tasks with this tag.
    pub tasks: usize,
    /// Number of non-deleted notes with this tag.
    pub notes: usize,
    /// Number of non-deleted resources with this tag.
    pub resources: usize,
}

impl TagStat {
    /// Total occurrences across all entity types.
    pub fn total(&self) -> usize {
        self.tasks + self.notes + self.resources
    }
}

// ── collect_tags ──────────────────────────────────────────────────────────────

/// Collect and count tags from tasks, notes, and resources.
///
/// - Ignores soft-deleted entities (`is_deleted() == true`).
/// - Tags are case-sensitive (normalization is handled upstream by
///   [`tag_normalizer`]).
/// - Results are sorted by total count descending; ties broken alphabetically.
///
/// [`tag_normalizer`]: crate::tag_normalizer
pub fn collect_tags(tasks: &[Task], notes: &[Note], resources: &[Resource]) -> Vec<TagStat> {
    let mut map: HashMap<String, TagStat> = HashMap::new();

    for task in tasks.iter().filter(|t| !t.is_deleted()) {
        for tag in &task.tags {
            let entry = map.entry(tag.clone()).or_insert_with(|| TagStat {
                name: tag.clone(),
                tasks: 0,
                notes: 0,
                resources: 0,
            });
            entry.tasks += 1;
        }
    }

    for note in notes.iter().filter(|n| !n.is_deleted()) {
        for tag in &note.tags {
            let entry = map.entry(tag.clone()).or_insert_with(|| TagStat {
                name: tag.clone(),
                tasks: 0,
                notes: 0,
                resources: 0,
            });
            entry.notes += 1;
        }
    }

    for resource in resources.iter().filter(|r| !r.is_deleted()) {
        for tag in &resource.tags {
            let entry = map.entry(tag.clone()).or_insert_with(|| TagStat {
                name: tag.clone(),
                tasks: 0,
                notes: 0,
                resources: 0,
            });
            entry.resources += 1;
        }
    }

    let mut stats: Vec<TagStat> = map.into_values().collect();
    stats.sort_by(|a, b| b.total().cmp(&a.total()).then(a.name.cmp(&b.name)));
    stats
}

/// Collect all unique tag names across all entity types.
///
/// Useful for autocomplete, fuzzy matching, and normalization.
/// Replaces the task-only `collect_existing_tags` in `tag_normalizer`.
pub fn collect_all_tag_names(
    tasks: &[Task],
    notes: &[Note],
    resources: &[Resource],
) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut tags = Vec::new();

    let task_tags = tasks
        .iter()
        .filter(|t| !t.is_deleted())
        .flat_map(|t| &t.tags);
    let note_tags = notes
        .iter()
        .filter(|n| !n.is_deleted())
        .flat_map(|n| &n.tags);
    let resource_tags = resources
        .iter()
        .filter(|r| !r.is_deleted())
        .flat_map(|r| &r.tags);

    for tag in task_tags.chain(note_tags).chain(resource_tags) {
        if seen.insert(tag.clone()) {
            tags.push(tag.clone());
        }
    }

    tags
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Note, Priority, Resource, Task};

    fn make_task(tags: &[&str]) -> Task {
        Task::new(
            "task".into(),
            Priority::Medium,
            tags.iter().map(|s| s.to_string()).collect(),
            None,
            None,
            None,
        )
    }

    fn make_note(tags: &[&str]) -> Note {
        let mut n = Note::new("note".into());
        n.tags = tags.iter().map(|s| s.to_string()).collect();
        n
    }

    fn make_resource(tags: &[&str]) -> Resource {
        let mut r = Resource::new("resource".into());
        r.tags = tags.iter().map(|s| s.to_string()).collect();
        r
    }

    #[test]
    fn test_collect_tags_empty() {
        let stats = collect_tags(&[], &[], &[]);
        assert!(stats.is_empty());
    }

    #[test]
    fn test_collect_tags_tasks_only() {
        let tasks = vec![make_task(&["rust", "work"]), make_task(&["rust"])];
        let stats = collect_tags(&tasks, &[], &[]);

        assert_eq!(stats.len(), 2);
        let rust = stats.iter().find(|s| s.name == "rust").unwrap();
        assert_eq!(rust.tasks, 2);
        assert_eq!(rust.notes, 0);
        assert_eq!(rust.resources, 0);
    }

    #[test]
    fn test_collect_tags_aggregates_across_entities() {
        let tasks = vec![make_task(&["rust"])];
        let notes = vec![make_note(&["rust", "async"])];
        let resources = vec![make_resource(&["rust"])];

        let stats = collect_tags(&tasks, &notes, &resources);

        let rust = stats.iter().find(|s| s.name == "rust").unwrap();
        assert_eq!(rust.tasks, 1);
        assert_eq!(rust.notes, 1);
        assert_eq!(rust.resources, 1);
        assert_eq!(rust.total(), 3);

        let async_stat = stats.iter().find(|s| s.name == "async").unwrap();
        assert_eq!(async_stat.total(), 1);
    }

    #[test]
    fn test_collect_tags_ignores_deleted() {
        let mut task = make_task(&["rust"]);
        task.soft_delete();

        let mut note = make_note(&["rust"]);
        note.soft_delete();

        let stats = collect_tags(&[task], &[note], &[]);
        assert!(stats.is_empty());
    }

    #[test]
    fn test_collect_tags_sorted_by_total_descending() {
        let tasks = vec![make_task(&["rust", "async"]), make_task(&["rust"])];
        let notes = vec![make_note(&["rust"])];
        let stats = collect_tags(&tasks, &notes, &[]);

        // rust: total 3, async: total 1 — rust comes first
        let names: Vec<_> = stats.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["rust", "async"]);
    }

    #[test]
    fn test_collect_tags_ties_broken_alphabetically() {
        let tasks = vec![make_task(&["work", "async", "rust"])];
        let stats = collect_tags(&tasks, &[], &[]);

        // all have total 1 — sorted alphabetically
        let names: Vec<_> = stats.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["async", "rust", "work"]);
    }

    #[test]
    fn test_collect_all_tag_names() {
        let tasks = vec![make_task(&["rust"])];
        let notes = vec![make_note(&["async"])];
        let resources = vec![make_resource(&["rust", "crate"])];

        let names = collect_all_tag_names(&tasks, &notes, &resources);
        assert!(names.contains(&"rust".to_string()));
        assert!(names.contains(&"async".to_string()));
        assert!(names.contains(&"crate".to_string()));
        assert_eq!(names.len(), 3); // rust appears in task + resource but deduped
    }
}
