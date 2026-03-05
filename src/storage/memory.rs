//! In-memory storage implementation for testing

use anyhow::Result;
use std::cell::RefCell;

use super::Storage;
use crate::models::{Note, Project, Resource, Task};

/// In-memory storage implementation
///
/// Stores tasks, projects, notes, and resources in memory without any file I/O,
/// making tests fast and isolated.
/// Uses `RefCell` for interior mutability since `Storage` trait methods take `&self`.
#[derive(Default)]
pub struct InMemoryStorage {
    tasks: RefCell<Vec<Task>>,
    projects: RefCell<Vec<Project>>,
    notes: RefCell<Vec<Note>>,
    resources: RefCell<Vec<Resource>>,
}

#[allow(dead_code)]
impl InMemoryStorage {
    pub fn with_tasks(tasks: Vec<Task>) -> Self {
        Self {
            tasks: RefCell::new(tasks),
            projects: RefCell::new(vec![]),
            notes: RefCell::new(vec![]),
            resources: RefCell::new(vec![]),
        }
    }

    pub fn len(&self) -> usize {
        self.tasks.borrow().len()
    }
    pub fn is_empty(&self) -> bool {
        self.tasks.borrow().is_empty()
    }
}

impl Storage for InMemoryStorage {
    fn load(&self) -> Result<Vec<Task>> {
        Ok(self.tasks.borrow().clone())
    }

    fn save(&self, tasks: &[Task]) -> Result<()> {
        *self.tasks.borrow_mut() = tasks.to_vec();
        Ok(())
    }

    fn load_projects(&self) -> Result<Vec<Project>> {
        Ok(self.projects.borrow().clone())
    }

    fn save_projects(&self, projects: &[Project]) -> Result<()> {
        *self.projects.borrow_mut() = projects.to_vec();
        Ok(())
    }

    fn load_notes(&self) -> Result<Vec<Note>> {
        Ok(self.notes.borrow().clone())
    }

    fn save_notes(&self, notes: &[Note]) -> Result<()> {
        *self.notes.borrow_mut() = notes.to_vec();
        Ok(())
    }

    fn load_resources(&self) -> Result<Vec<Resource>> {
        Ok(self.resources.borrow().clone())
    }

    fn save_resources(&self, resources: &[Resource]) -> Result<()> {
        *self.resources.borrow_mut() = resources.to_vec();
        Ok(())
    }

    fn location(&self) -> String {
        "memory".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Priority;

    #[test]
    fn test_memory_storage_starts_empty() {
        let storage = InMemoryStorage::default();
        assert_eq!(storage.len(), 0);
        assert!(storage.is_empty());
        assert_eq!(storage.load().unwrap().len(), 0);
        assert_eq!(storage.load_projects().unwrap().len(), 0);
        assert_eq!(storage.load_notes().unwrap().len(), 0);
        assert_eq!(storage.load_resources().unwrap().len(), 0);
    }

    #[test]
    fn test_memory_storage_save_and_load() {
        let storage = InMemoryStorage::default();
        let tasks = vec![
            Task::new("Task 1".into(), Priority::High, vec![], None, None, None),
            Task::new("Task 2".into(), Priority::Low, vec![], None, None, None),
        ];
        storage.save(&tasks).unwrap();
        let loaded = storage.load().unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].text, "Task 1");
        assert_eq!(loaded[1].text, "Task 2");
    }

    #[test]
    fn test_memory_storage_projects() {
        let storage = InMemoryStorage::default();
        let projects = vec![Project::new("Backend".into())];
        storage.save_projects(&projects).unwrap();
        let loaded = storage.load_projects().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].name, "Backend");
    }

    #[test]
    fn test_memory_storage_notes() {
        let storage = InMemoryStorage::default();
        let mut note = Note::new("Documentação inicial".into());
        note.title = Some("Setup".into());
        note.language = Some("Rust".into());
        storage.save_notes(&[note]).unwrap();
        let loaded = storage.load_notes().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].body, "Documentação inicial");
        assert_eq!(loaded[0].title.as_deref(), Some("Setup"));
        assert_eq!(loaded[0].language.as_deref(), Some("Rust"));
    }

    #[test]
    fn test_memory_storage_resources() {
        let storage = InMemoryStorage::default();
        let mut resource = Resource::new("sqlx docs".into());
        resource.url = Some("https://docs.rs/sqlx".into());
        resource.tags = vec!["rust".into(), "db".into()];
        storage.save_resources(&[resource]).unwrap();
        let loaded = storage.load_resources().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].title, "sqlx docs");
        assert_eq!(loaded[0].url.as_deref(), Some("https://docs.rs/sqlx"));
    }

    #[test]
    fn test_note_links_to_project() {
        let storage = InMemoryStorage::default();
        let project = Project::new("MeuProjeto".into());
        let project_uuid = project.uuid;

        let mut note = Note::new("Nota vinculada ao projeto".into());
        note.project_id = Some(project_uuid);

        storage.save_projects(&[project]).unwrap();
        storage.save_notes(&[note]).unwrap();

        let notes = storage.load_notes().unwrap();
        assert_eq!(notes[0].project_id, Some(project_uuid));
        assert!(notes[0].belongs_to_project(project_uuid));
    }

    #[test]
    fn test_note_links_to_task() {
        let storage = InMemoryStorage::default();
        let task = Task::new(
            "Minha task".into(),
            Priority::Medium,
            vec![],
            None,
            None,
            None,
        );
        let task_uuid = task.uuid;

        let mut note = Note::new("Nota vinculada à task".into());
        note.task_id = Some(task_uuid);

        storage.save(&[task]).unwrap();
        storage.save_notes(&[note]).unwrap();

        let notes = storage.load_notes().unwrap();
        assert_eq!(notes[0].task_id, Some(task_uuid));
        assert!(notes[0].belongs_to_task(task_uuid));
    }

    #[test]
    fn test_note_links_to_resources() {
        let storage = InMemoryStorage::default();
        let r1 = Resource::new("sqlx docs".into());
        let r2 = Resource::new("tokio docs".into());
        let (r1_uuid, r2_uuid) = (r1.uuid, r2.uuid);

        let mut note = Note::new("Async DB setup".into());
        note.add_resource(r1_uuid);
        note.add_resource(r2_uuid);

        storage.save_resources(&[r1, r2]).unwrap();
        storage.save_notes(&[note]).unwrap();

        let notes = storage.load_notes().unwrap();
        assert!(notes[0].references_resource(r1_uuid));
        assert!(notes[0].references_resource(r2_uuid));
        assert_eq!(notes[0].resource_ids.len(), 2);
    }

    #[test]
    fn test_note_add_remove_resource() {
        let storage = InMemoryStorage::default();
        let r = Resource::new("Some doc".into());
        let r_uuid = r.uuid;

        let mut note = Note::new("Test note".into());
        note.add_resource(r_uuid);
        // add_resource is idempotent
        note.add_resource(r_uuid);
        assert_eq!(note.resource_ids.len(), 1);

        note.remove_resource(r_uuid);
        assert!(note.resource_ids.is_empty());

        storage.save_resources(&[r]).unwrap();
        storage.save_notes(&[note]).unwrap();
        let notes = storage.load_notes().unwrap();
        assert!(notes[0].resource_ids.is_empty());
    }

    #[test]
    fn test_note_links_to_both() {
        let storage = InMemoryStorage::default();
        let project = Project::new("P".into());
        let task = Task::new("T".into(), Priority::Low, vec![], None, None, None);
        let (p_uuid, t_uuid) = (project.uuid, task.uuid);

        let mut note = Note::new("Nota dupla".into());
        note.project_id = Some(p_uuid);
        note.task_id = Some(t_uuid);

        storage.save_projects(&[project]).unwrap();
        storage.save(&[task]).unwrap();
        storage.save_notes(&[note]).unwrap();

        let notes = storage.load_notes().unwrap();
        assert!(notes[0].belongs_to_project(p_uuid));
        assert!(notes[0].belongs_to_task(t_uuid));
    }

    #[test]
    fn test_tasks_and_projects_independent() {
        let storage = InMemoryStorage::default();
        storage
            .save(&[Task::new(
                "T".into(),
                Priority::Medium,
                vec![],
                None,
                None,
                None,
            )])
            .unwrap();
        storage.save_projects(&[Project::new("P".into())]).unwrap();
        assert_eq!(storage.load().unwrap().len(), 1);
        assert_eq!(storage.load_projects().unwrap().len(), 1);
        assert_eq!(storage.load_notes().unwrap().len(), 0);
        assert_eq!(storage.load_resources().unwrap().len(), 0);
    }

    #[test]
    fn test_memory_storage_with_tasks() {
        let tasks = vec![Task::new(
            "Existing".into(),
            Priority::Medium,
            vec![],
            None,
            None,
            None,
        )];
        let storage = InMemoryStorage::with_tasks(tasks);
        assert_eq!(storage.len(), 1);
        assert_eq!(storage.load().unwrap()[0].text, "Existing");
    }

    #[test]
    fn test_memory_storage_overwrite() {
        let storage = InMemoryStorage::default();
        storage
            .save(&[Task::new(
                "Task 1".into(),
                Priority::Medium,
                vec![],
                None,
                None,
                None,
            )])
            .unwrap();
        storage
            .save(&[
                Task::new("Task 2".into(), Priority::High, vec![], None, None, None),
                Task::new("Task 3".into(), Priority::Low, vec![], None, None, None),
            ])
            .unwrap();
        let loaded = storage.load().unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].text, "Task 2");
    }

    #[test]
    fn test_memory_storage_location() {
        assert_eq!(InMemoryStorage::default().location(), "memory");
    }
}
