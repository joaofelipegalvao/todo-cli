//! Terminal rendering for all entity lists.
//!
//! Each submodule owns the table layout for its entity type:
//!
//! - [`task_table`]     — `todo list`, `todo search`
//! - [`note_table`]     — `todo note list`
//! - [`project_table`]  — `todo project list`
//! - [`resource_table`] — `todo resource list`
//! - [`formatting`]     — shared helpers (truncate, due text, colors)
//! - [`next_table`]
//! - [`calendar`]

pub mod calendar;
pub mod formatting;
pub mod next_table;
pub mod note_table;
pub mod project_table;
pub mod resource_table;
pub mod task_table;

pub use next_table::display_next;
pub use note_table::display_notes;
pub use project_table::display_projects;
pub use resource_table::display_resources;
pub use task_table::display_lists;
