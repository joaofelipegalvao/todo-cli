//! SQLite-backed storage implementation for rustodo.
//!
//! Uses `rusqlite` (synchronous) — no async runtime needed for a CLI.
//! UUIDs are stored as TEXT, arrays (tags, tech) as JSON TEXT via `JsonVec<T>`.

use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use directories::ProjectDirs;
use rusqlite::{
    Connection, Row, params,
    types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
};
use serde::{Serialize, de::DeserializeOwned};
use std::path::PathBuf;
use uuid::Uuid;

use super::Storage;
use crate::models::{
    Difficulty, Note, NoteFormat, Priority, Project, Recurrence, Resource, ResourceType, Task,
};

// ── JsonVec<T> ────────────────────────────────────────────────────────────────

/// A `Vec<T>` that serializes to/from a JSON TEXT column in SQLite.
///
/// Prevents manual `serde_json::to_string` / `from_str` calls scattered
/// across the codebase and guarantees a consistent compact JSON format.
#[derive(Debug)]
pub struct JsonVec<T>(pub Vec<T>);

impl<T: Serialize> ToSql for JsonVec<T> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let json = serde_json::to_string(&self.0)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(ToSqlOutput::from(json))
    }
}

impl<T: DeserializeOwned> FromSql for JsonVec<T> {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let s = value.as_str()?;
        serde_json::from_str(s)
            .map(JsonVec)
            .map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

// ── timestamp helpers ─────────────────────────────────────────────────────────

fn to_unix(dt: DateTime<Utc>) -> i64 {
    dt.timestamp()
}

fn from_unix(ts: i64) -> DateTime<Utc> {
    Utc.timestamp_opt(ts, 0)
        .single()
        .unwrap_or(DateTime::UNIX_EPOCH)
}

fn opt_to_unix(dt: Option<DateTime<Utc>>) -> Option<i64> {
    dt.map(to_unix)
}

fn opt_from_unix(ts: Option<i64>) -> Option<DateTime<Utc>> {
    ts.map(from_unix)
}

// ── SqliteStorage ─────────────────────────────────────────────────────────────

pub struct SqliteStorage {
    path: PathBuf,
}

impl SqliteStorage {
    pub fn new() -> Result<Self> {
        let path = get_db_path()?;
        let storage = Self { path };
        storage.initialize()?;
        Ok(storage)
    }

    #[cfg(test)]
    pub fn with_path(path: PathBuf) -> Result<Self> {
        let storage = Self { path };
        storage.initialize()?;
        Ok(storage)
    }

    fn open(&self) -> Result<Connection> {
        let conn = Connection::open(&self.path).context("Failed to open SQLite database")?;
        conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;
        Ok(conn)
    }

    /// Create all tables and indices if they don't exist yet.
    fn initialize(&self) -> Result<()> {
        let conn = self.open()?;
        conn.execute_batch(SCHEMA)?;
        Ok(())
    }
}

// ── schema ────────────────────────────────────────────────────────────────────

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS projects (
    uuid        TEXT PRIMARY KEY NOT NULL,
    name        TEXT NOT NULL,
    completed   INTEGER NOT NULL DEFAULT 0,
    difficulty  TEXT NOT NULL DEFAULT 'medium',
    tech        TEXT NOT NULL DEFAULT '[]',
    due_date    TEXT,
    completed_at TEXT,
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER,
    deleted_at  INTEGER
);

CREATE TABLE IF NOT EXISTS tasks (
    uuid        TEXT PRIMARY KEY NOT NULL,
    text        TEXT NOT NULL,
    completed   INTEGER NOT NULL DEFAULT 0,
    priority    TEXT NOT NULL DEFAULT 'medium'
                    CHECK(priority IN ('low','medium','high')),
    due_date    TEXT,
    recurrence  TEXT,
    project_id  TEXT REFERENCES projects(uuid),
    parent_id   TEXT REFERENCES tasks(uuid),
    tags        TEXT NOT NULL DEFAULT '[]',
    completed_at INTEGER,
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER,
    deleted_at  INTEGER
);

CREATE TABLE IF NOT EXISTS task_dependencies (
    task_uuid        TEXT NOT NULL REFERENCES tasks(uuid),
    depends_on_uuid  TEXT NOT NULL REFERENCES tasks(uuid),
    PRIMARY KEY (task_uuid, depends_on_uuid)
);

CREATE TABLE IF NOT EXISTS notes (
    uuid        TEXT PRIMARY KEY NOT NULL,
    title       TEXT,
    body        TEXT NOT NULL,
    format      TEXT NOT NULL DEFAULT 'plain',
    language    TEXT,
    project_id  TEXT REFERENCES projects(uuid),
    task_id     TEXT REFERENCES tasks(uuid),
    tags        TEXT NOT NULL DEFAULT '[]',
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER,
    deleted_at  INTEGER
);

CREATE TABLE IF NOT EXISTS resources (
    uuid          TEXT PRIMARY KEY NOT NULL,
    title         TEXT NOT NULL,
    resource_type TEXT,
    url           TEXT,
    description   TEXT,
    tags          TEXT NOT NULL DEFAULT '[]',
    created_at    INTEGER NOT NULL,
    updated_at    INTEGER,
    deleted_at    INTEGER
);

CREATE TABLE IF NOT EXISTS note_resources (
    note_uuid     TEXT NOT NULL REFERENCES notes(uuid),
    resource_uuid TEXT NOT NULL REFERENCES resources(uuid),
    PRIMARY KEY (note_uuid, resource_uuid)
);

-- partial indices (only active rows)
CREATE INDEX IF NOT EXISTS idx_tasks_active
    ON tasks(created_at) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_tasks_project_active
    ON tasks(project_id, created_at) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_tasks_completed
    ON tasks(completed_at);
CREATE INDEX IF NOT EXISTS idx_notes_active
    ON notes(created_at) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_notes_project
    ON notes(project_id, created_at) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_notes_task
    ON notes(task_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_resources_active
    ON resources(created_at) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_projects_active
    ON projects(created_at) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_task_deps
    ON task_dependencies(depends_on_uuid);
";

// ── row mappers ───────────────────────────────────────────────────────────────

fn row_to_task(row: &Row, conn: &Connection, uuid_str: &str) -> rusqlite::Result<Task> {
    let uuid = Uuid::parse_str(uuid_str).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })?;

    let priority_str: String = row.get("priority")?;
    let priority = match priority_str.as_str() {
        "high" => Priority::High,
        "low" => Priority::Low,
        _ => Priority::Medium,
    };

    let recurrence_str: Option<String> = row.get("recurrence")?;
    let recurrence = recurrence_str.as_deref().and_then(|s| match s {
        "daily" => Some(Recurrence::Daily),
        "weekly" => Some(Recurrence::Weekly),
        "monthly" => Some(Recurrence::Monthly),
        _ => None,
    });

    let project_id_str: Option<String> = row.get("project_id")?;
    let project_id = project_id_str
        .as_deref()
        .and_then(|s| Uuid::parse_str(s).ok());

    let due_date_str: Option<String> = row.get("due_date")?;
    let due_date = due_date_str
        .as_deref()
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());

    let tags: JsonVec<String> = row.get("tags")?;

    let created_at = from_unix(row.get("created_at")?);
    let updated_at = opt_from_unix(row.get("updated_at")?);
    let deleted_at = opt_from_unix(row.get("deleted_at")?);
    let completed_at_ts: Option<i64> = row.get("completed_at")?;
    let completed_at = completed_at_ts
        .map(from_unix)
        .map(|dt| dt.naive_local().date());

    // Load dependencies from task_dependencies table
    let mut dep_stmt =
        conn.prepare_cached("SELECT depends_on_uuid FROM task_dependencies WHERE task_uuid = ?1")?;
    let depends_on: Vec<Uuid> = dep_stmt
        .query_map(params![uuid_str], |r| r.get::<_, String>(0))?
        .filter_map(|r| r.ok())
        .filter_map(|s| Uuid::parse_str(&s).ok())
        .collect();

    let parent_id_str: Option<String> = row.get("parent_id")?;
    let parent_id = parent_id_str
        .as_deref()
        .and_then(|s| Uuid::parse_str(s).ok());

    Ok(Task {
        uuid,
        text: row.get("text")?,
        completed: row.get::<_, i64>("completed")? != 0,
        priority,
        due_date,
        recurrence,
        project_id,
        project_name_legacy: None, // JSON-only field, always None in SQLite
        parent_id,
        tags: tags.0,
        depends_on,
        created_at,
        updated_at,
        deleted_at,
        completed_at,
    })
}

fn row_to_project(row: &Row) -> rusqlite::Result<Project> {
    let uuid_str: String = row.get("uuid")?;
    let uuid = Uuid::parse_str(&uuid_str).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })?;

    let difficulty_str: String = row.get("difficulty")?;
    let difficulty = match difficulty_str.as_str() {
        "easy" => Difficulty::Easy,
        "hard" => Difficulty::Hard,
        _ => Difficulty::Medium,
    };

    let tech: JsonVec<String> = row.get("tech")?;

    let due_date_str: Option<String> = row.get("due_date")?;
    let due_date = due_date_str
        .as_deref()
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());

    let completed_at_str: Option<String> = row.get("completed_at")?;
    let completed_at = completed_at_str
        .as_deref()
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());

    Ok(Project {
        uuid,
        name: row.get("name")?,
        completed: row.get::<_, i64>("completed")? != 0,
        difficulty,
        tech: tech.0,
        due_date,
        completed_at,
        created_at: from_unix(row.get("created_at")?),
        updated_at: opt_from_unix(row.get("updated_at")?),
        deleted_at: opt_from_unix(row.get("deleted_at")?),
    })
}

fn row_to_note(row: &Row, conn: &Connection) -> rusqlite::Result<Note> {
    let uuid_str: String = row.get("uuid")?;
    let uuid = Uuid::parse_str(&uuid_str).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })?;

    let format_str: String = row.get("format")?;
    let format = match format_str.as_str() {
        "markdown" => NoteFormat::Markdown,
        _ => NoteFormat::Plain,
    };

    let project_id_str: Option<String> = row.get("project_id")?;
    let project_id = project_id_str
        .as_deref()
        .and_then(|s| Uuid::parse_str(s).ok());

    let task_id_str: Option<String> = row.get("task_id")?;
    let task_id = task_id_str.as_deref().and_then(|s| Uuid::parse_str(s).ok());

    let tags: JsonVec<String> = row.get("tags")?;

    // Load resource links from note_resources
    let mut res_stmt =
        conn.prepare_cached("SELECT resource_uuid FROM note_resources WHERE note_uuid = ?1")?;
    let resource_ids: Vec<Uuid> = res_stmt
        .query_map(params![uuid_str], |r| r.get::<_, String>(0))?
        .filter_map(|r| r.ok())
        .filter_map(|s| Uuid::parse_str(&s).ok())
        .collect();

    Ok(Note {
        uuid,
        title: row.get("title")?,
        body: row.get("body")?,
        format,
        language: row.get("language")?,
        project_id,
        task_id,
        tags: tags.0,
        resource_ids,
        created_at: from_unix(row.get("created_at")?),
        updated_at: opt_from_unix(row.get("updated_at")?),
        deleted_at: opt_from_unix(row.get("deleted_at")?),
    })
}

fn row_to_resource(row: &Row) -> rusqlite::Result<Resource> {
    let uuid_str: String = row.get("uuid")?;
    let uuid = Uuid::parse_str(&uuid_str).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })?;

    let resource_type_str: Option<String> = row.get("resource_type")?;
    let resource_type = resource_type_str.as_deref().and_then(|s| match s {
        "docs" => Some(ResourceType::Docs),
        "article" => Some(ResourceType::Article),
        "video" => Some(ResourceType::Video),
        "repo" => Some(ResourceType::Repo),
        "crate" => Some(ResourceType::Crate),
        "book" => Some(ResourceType::Book),
        "spec" => Some(ResourceType::Spec),
        "tool" => Some(ResourceType::Tool),
        _ => None,
    });

    let tags: JsonVec<String> = row.get("tags")?;

    Ok(Resource {
        uuid,
        title: row.get("title")?,
        resource_type,
        url: row.get("url")?,
        description: row.get("description")?,
        tags: tags.0,
        created_at: from_unix(row.get("created_at")?),
        updated_at: opt_from_unix(row.get("updated_at")?),
        deleted_at: opt_from_unix(row.get("deleted_at")?),
    })
}

// ── Storage impl ──────────────────────────────────────────────────────────────

impl Storage for SqliteStorage {
    fn load(&self) -> Result<Vec<Task>> {
        let conn = self.open()?;
        let mut stmt = conn.prepare("SELECT * FROM tasks ORDER BY created_at")?;
        let tasks = stmt
            .query_map([], |row| {
                let uuid_str: String = row.get("uuid")?;
                row_to_task(row, &conn, &uuid_str)
            })?
            .collect::<rusqlite::Result<Vec<_>>>()
            .context("Failed to load tasks")?;
        Ok(tasks)
    }

    fn save(&self, tasks: &[Task]) -> Result<()> {
        let conn = self.open()?;
        let tx = conn;

        // Upsert all tasks
        for task in tasks {
            let uuid_str = task.uuid.to_string();
            tx.execute(
                "INSERT INTO tasks (uuid, text, completed, priority, due_date, recurrence, project_id, parent_id, tags, completed_at, created_at, updated_at, deleted_at)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)
                 ON CONFLICT(uuid) DO UPDATE SET
                   text=excluded.text, completed=excluded.completed, priority=excluded.priority,
                   due_date=excluded.due_date, recurrence=excluded.recurrence,
                   project_id=excluded.project_id, parent_id=excluded.parent_id,
                   tags=excluded.tags, completed_at=excluded.completed_at,
                   updated_at=excluded.updated_at, deleted_at=excluded.deleted_at",
                params![
                    uuid_str,
                    task.text,
                    task.completed as i64,
                    priority_to_str(task.priority),
                    task.due_date.map(|d| d.format("%Y-%m-%d").to_string()),
                    task.recurrence.map(recurrence_to_str),
                    task.project_id.map(|u| u.to_string()),
                    task.parent_id.map(|u| u.to_string()),
                    JsonVec(task.tags.clone()),
                    task.completed_at.map(|d| {
                        let dt: DateTime<Utc> = Utc.from_utc_datetime(
                            &d.and_hms_opt(0,0,0).unwrap()
                        );
                        to_unix(dt)
                    }),
                    to_unix(task.created_at),
                    opt_to_unix(task.updated_at),
                    opt_to_unix(task.deleted_at),
                ],
            )?;

            // Sync dependencies
            tx.execute(
                "DELETE FROM task_dependencies WHERE task_uuid = ?1",
                params![uuid_str],
            )?;
            for dep_uuid in &task.depends_on {
                tx.execute(
                    "INSERT OR IGNORE INTO task_dependencies (task_uuid, depends_on_uuid) VALUES (?1, ?2)",
                    params![uuid_str, dep_uuid.to_string()],
                )?;
            }
        }
        Ok(())
    }

    fn load_projects(&self) -> Result<Vec<Project>> {
        let conn = self.open()?;
        let mut stmt = conn.prepare("SELECT * FROM projects ORDER BY created_at")?;
        let projects = stmt
            .query_map([], |row| row_to_project(row))?
            .collect::<rusqlite::Result<Vec<_>>>()
            .context("Failed to load projects")?;
        Ok(projects)
    }

    fn save_projects(&self, projects: &[Project]) -> Result<()> {
        let conn = self.open()?;
        for project in projects {
            conn.execute(
                "INSERT INTO projects (uuid, name, completed, difficulty, tech, due_date, completed_at, created_at, updated_at, deleted_at)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)
                 ON CONFLICT(uuid) DO UPDATE SET
                   name=excluded.name, completed=excluded.completed, difficulty=excluded.difficulty,
                   tech=excluded.tech, due_date=excluded.due_date, completed_at=excluded.completed_at,
                   updated_at=excluded.updated_at, deleted_at=excluded.deleted_at",
                params![
                    project.uuid.to_string(),
                    project.name,
                    project.completed as i64,
                    difficulty_to_str(project.difficulty),
                    JsonVec(project.tech.clone()),
                    project.due_date.map(|d| d.format("%Y-%m-%d").to_string()),
                    project.completed_at.map(|d| d.format("%Y-%m-%d").to_string()),
                    to_unix(project.created_at),
                    opt_to_unix(project.updated_at),
                    opt_to_unix(project.deleted_at),
                ],
            )?;
        }
        Ok(())
    }

    fn load_notes(&self) -> Result<Vec<Note>> {
        let conn = self.open()?;
        let mut stmt = conn.prepare("SELECT * FROM notes ORDER BY created_at")?;
        let notes = stmt
            .query_map([], |row| row_to_note(row, &conn))?
            .collect::<rusqlite::Result<Vec<_>>>()
            .context("Failed to load notes")?;
        Ok(notes)
    }

    fn save_notes(&self, notes: &[Note]) -> Result<()> {
        let conn = self.open()?;
        for note in notes {
            let uuid_str = note.uuid.to_string();
            conn.execute(
                "INSERT INTO notes (uuid, title, body, format, language, project_id, task_id, tags, created_at, updated_at, deleted_at)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)
                 ON CONFLICT(uuid) DO UPDATE SET
                   title=excluded.title, body=excluded.body, format=excluded.format,
                   language=excluded.language, project_id=excluded.project_id,
                   task_id=excluded.task_id, tags=excluded.tags,
                   updated_at=excluded.updated_at, deleted_at=excluded.deleted_at",
                params![
                    uuid_str,
                    note.title,
                    note.body,
                    format_to_str(note.format),
                    note.language,
                    note.project_id.map(|u| u.to_string()),
                    note.task_id.map(|u| u.to_string()),
                    JsonVec(note.tags.clone()),
                    to_unix(note.created_at),
                    opt_to_unix(note.updated_at),
                    opt_to_unix(note.deleted_at),
                ],
            )?;

            // Sync resource links
            conn.execute(
                "DELETE FROM note_resources WHERE note_uuid = ?1",
                params![uuid_str],
            )?;
            for resource_id in &note.resource_ids {
                conn.execute(
                    "INSERT OR IGNORE INTO note_resources (note_uuid, resource_uuid) VALUES (?1, ?2)",
                    params![uuid_str, resource_id.to_string()],
                )?;
            }
        }
        Ok(())
    }

    fn load_resources(&self) -> Result<Vec<Resource>> {
        let conn = self.open()?;
        let mut stmt = conn.prepare("SELECT * FROM resources ORDER BY created_at")?;
        let resources = stmt
            .query_map([], |row| row_to_resource(row))?
            .collect::<rusqlite::Result<Vec<_>>>()
            .context("Failed to load resources")?;
        Ok(resources)
    }

    fn save_resources(&self, resources: &[Resource]) -> Result<()> {
        let conn = self.open()?;
        for resource in resources {
            conn.execute(
                "INSERT INTO resources (uuid, title, resource_type, url, description, tags, created_at, updated_at, deleted_at)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)
                 ON CONFLICT(uuid) DO UPDATE SET
                   title=excluded.title, resource_type=excluded.resource_type,
                   url=excluded.url, description=excluded.description, tags=excluded.tags,
                   updated_at=excluded.updated_at, deleted_at=excluded.deleted_at",
                params![
                    resource.uuid.to_string(),
                    resource.title,
                    resource.resource_type.map(resource_type_to_str),
                    resource.url,
                    resource.description,
                    JsonVec(resource.tags.clone()),
                    to_unix(resource.created_at),
                    opt_to_unix(resource.updated_at),
                    opt_to_unix(resource.deleted_at),
                ],
            )?;
        }
        Ok(())
    }

    fn location(&self) -> String {
        self.path.display().to_string()
    }
}

// ── enum helpers ──────────────────────────────────────────────────────────────

fn priority_to_str(p: Priority) -> &'static str {
    match p {
        Priority::High => "high",
        Priority::Medium => "medium",
        Priority::Low => "low",
    }
}

fn recurrence_to_str(r: Recurrence) -> &'static str {
    match r {
        Recurrence::Daily => "daily",
        Recurrence::Weekly => "weekly",
        Recurrence::Monthly => "monthly",
    }
}

fn difficulty_to_str(d: Difficulty) -> &'static str {
    match d {
        Difficulty::Easy => "easy",
        Difficulty::Medium => "medium",
        Difficulty::Hard => "hard",
    }
}

fn format_to_str(f: NoteFormat) -> &'static str {
    match f {
        NoteFormat::Plain => "plain",
        NoteFormat::Markdown => "markdown",
    }
}

fn resource_type_to_str(rt: ResourceType) -> &'static str {
    match rt {
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

// ── path helper ───────────────────────────────────────────────────────────────

pub fn get_db_path() -> Result<PathBuf> {
    let data_dir = if let Ok(dir) = std::env::var("RUSTODO_DATA_DIR") {
        PathBuf::from(dir)
    } else {
        let proj_dirs =
            ProjectDirs::from("", "", "rustodo").context("Could not determine data directory")?;
        proj_dirs.data_dir().to_path_buf()
    };
    std::fs::create_dir_all(&data_dir).context("Failed to create data directory")?;
    Ok(data_dir.join("rustodo.db"))
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Priority;
    use tempfile::TempDir;

    fn make_storage() -> (SqliteStorage, TempDir) {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("test.db");
        let storage = SqliteStorage::with_path(path).unwrap();
        (storage, tmp)
    }

    #[test]
    fn test_save_and_load_tasks() {
        let (storage, _tmp) = make_storage();
        let task = Task::new(
            "Buy milk".into(),
            Priority::Medium,
            vec![],
            None,
            None,
            None,
        );
        storage.save(&[task]).unwrap();
        let loaded = storage.load().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].text, "Buy milk");
    }

    #[test]
    fn test_save_and_load_projects() {
        let (storage, _tmp) = make_storage();
        let project = Project::new("Backend".into());
        storage.save_projects(&[project]).unwrap();
        let loaded = storage.load_projects().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].name, "Backend");
    }

    #[test]
    fn test_save_and_load_notes() {
        let (storage, _tmp) = make_storage();
        let mut note = Note::new("Documentação".into());
        note.title = Some("Intro".into());
        note.language = Some("Rust".into());
        storage.save_notes(&[note]).unwrap();
        let loaded = storage.load_notes().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].body, "Documentação");
        assert_eq!(loaded[0].language.as_deref(), Some("Rust"));
    }

    #[test]
    fn test_save_and_load_resources() {
        let (storage, _tmp) = make_storage();
        let mut r = Resource::new("sqlx docs".into());
        r.url = Some("https://docs.rs/sqlx".into());
        r.tags = vec!["rust".into(), "db".into()];
        storage.save_resources(&[r]).unwrap();
        let loaded = storage.load_resources().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].url.as_deref(), Some("https://docs.rs/sqlx"));
        assert_eq!(loaded[0].tags, vec!["rust", "db"]);
    }

    #[test]
    fn test_task_dependencies() {
        let (storage, _tmp) = make_storage();
        let t1 = Task::new("Task 1".into(), Priority::Medium, vec![], None, None, None);
        let t2 = Task::new(
            "Task 2".into(),
            Priority::Medium,
            vec![t1.uuid],
            None,
            None,
            None,
        );
        storage.save(&[t1.clone(), t2.clone()]).unwrap();
        let loaded = storage.load().unwrap();
        let loaded_t2 = loaded.iter().find(|t| t.uuid == t2.uuid).unwrap();
        assert_eq!(loaded_t2.depends_on, vec![t1.uuid]);
    }

    #[test]
    fn test_note_resource_links() {
        let (storage, _tmp) = make_storage();
        let r = Resource::new("sqlx docs".into());
        let r_uuid = r.uuid;
        storage.save_resources(&[r]).unwrap();

        let mut note = Note::new("Setup".into());
        note.add_resource(r_uuid);
        storage.save_notes(&[note]).unwrap();

        let loaded = storage.load_notes().unwrap();
        assert!(loaded[0].references_resource(r_uuid));
    }

    #[test]
    fn test_upsert_does_not_duplicate() {
        let (storage, _tmp) = make_storage();
        let task = Task::new("T".into(), Priority::Medium, vec![], None, None, None);
        storage.save(&[task.clone()]).unwrap();
        storage.save(&[task]).unwrap();
        assert_eq!(storage.load().unwrap().len(), 1);
    }

    #[test]
    fn test_soft_delete_preserved() {
        let (storage, _tmp) = make_storage();
        let mut task = Task::new("T".into(), Priority::Medium, vec![], None, None, None);
        task.soft_delete();
        storage.save(&[task]).unwrap();
        let loaded = storage.load().unwrap();
        assert!(loaded[0].is_deleted());
    }

    #[test]
    fn test_markdown_note_format() {
        let (storage, _tmp) = make_storage();
        let note = Note::new_markdown("# Hello".into());
        storage.save_notes(&[note]).unwrap();
        let loaded = storage.load_notes().unwrap();
        assert_eq!(loaded[0].format, NoteFormat::Markdown);
    }
}
