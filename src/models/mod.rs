//! Core domain types for rustodo.
//!
//! | Type | Description |
//! |---|---|
//! | [`Task`]     | A single todo item with all its metadata |
//! | [`Note`]     | A free-form documentation note, optionally linked to a Project, Task, or Resources |
//! | [`Project`]  | A project entity that groups tasks and notes |
//! | [`Resource`] | An independent external reference (URL, doc, asset) attached via Notes |
//! | [`Priority`]         | High / Medium / Low priority levels |
//! | [`Recurrence`]       | Daily / Weekly / Monthly repeat patterns |
//! | [`StatusFilter`]     | Filter tasks by completion status |
//! | [`DueFilter`]        | Filter tasks by due-date window |
//! | [`RecurrenceFilter`] | Filter tasks by recurrence pattern |
//! | [`SortBy`]           | Sort order options for task lists |

mod filters;
mod note;
mod priority;
mod project;
mod recurrence;
mod resource;
mod task;

pub use filters::{DueFilter, RecurrenceFilter, SortBy, StatusFilter};
pub use note::Note;
pub use priority::Priority;
pub use project::{Difficulty, Project};
pub use recurrence::Recurrence;
pub use resource::Resource;
pub(crate) use task::detect_cycle;
pub use task::{Task, count_by_project};
