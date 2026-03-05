//! Command handlers — one submodule per CLI subcommand.
//!
//! | Submodule       | Subcommand                    |
//! |-----------------|-------------------------------|
//! | [`add`]         | `todo add`                    |
//! | [`clear`]       | `todo clear`                  |
//! | [`clear_recur`] | `todo norecur <ID>`           |
//! | [`deps`]        | `todo deps <ID>`              |
//! | [`done`]        | `todo done <ID>`              |
//! | [`edit`]        | `todo edit <ID>`              |
//! | [`info`]        | `todo info`                   |
//! | [`list`]        | `todo list`                   |
//! | [`note_add`]    | `todo note add`               |
//! | [`note_edit`]   | `todo note edit <ID>`         |
//! | [`note_list`]   | `todo note list`              |
//! | [`note_remove`] | `todo note remove <ID>`       |
//! | [`note_show`]   | `todo note show <ID>`         |
//! | [`project_add`] | `todo project add`            |
//! | [`project_edit`]| `todo project edit <ID>`      |
//! | [`project_remove`]| `todo project remove <ID>` |
//! | [`projects`]    | `todo projects`               |
//! | [`purge`]       | `todo purge`                  |
//! | [`recur`]       | `todo recur <ID>`             |
//! | [`remove`]      | `todo remove <ID>`            |
//! | [`search`]      | `todo search <QUERY>`         |
//! | [`stats`]       | `todo stats`                  |
//! | [`sync`]        | `todo sync`                   |
//! | [`tags`]        | `todo tags`                   |
//! | [`undone`]      | `todo undone <ID>`            |

pub mod add;
pub mod clear;
pub mod clear_recur;
pub mod deps;
pub mod done;
pub mod edit;
pub mod info;
pub mod list;
pub mod note_add;
pub mod note_edit;
pub mod note_list;
pub mod note_remove;
pub mod note_show;
pub mod project_add;
pub mod project_edit;
pub mod project_remove;
pub mod projects;
pub mod purge;
pub mod recur;
pub mod remove;
pub mod resource_add;
pub mod resource_edit;
pub mod resource_list;
pub mod resource_remove;
pub mod resource_show;
pub mod search;
pub mod stats;
pub mod sync;
pub mod tags;
pub mod undone;
