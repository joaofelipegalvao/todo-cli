//! Command handlers — one submodule per CLI subcommand.
//!
//! | Submodule            | Subcommand                      |
//! |----------------------|---------------------------------|
//! | [`task_add`]         | `todo add`                      |
//! | [`task_clear`]       | `todo clear`                    |
//! | [`task_clear_recur`] | `todo norecur <ID>`             |
//! | [`context`]          | `todo context <ID>`             |
//! | [`task_deps`]        | `todo deps <ID>`                |
//! | [`task_done`]        | `todo done <ID>`                |
//! | [`task_edit`]        | `todo edit <ID>`                |
//! | [`task_info`]        | `todo info`                     |
//! | [`task_list`]        | `todo list`                     |
//! | [`task_recur`]       | `todo recur <ID>`               |
//! | [`task_remove`]      | `todo remove <ID>`              |
//! | [`task_undone`]      | `todo undone <ID>`              |
//! | [`note_add`]         | `todo note add`                 |
//! | [`note_clear`]       | `todo note clear`               |
//! | [`note_edit`]        | `todo note edit <ID>`           |
//! | [`note_list`]        | `todo note list`                |
//! | [`note_remove`]      | `todo note remove <ID>`         |
//! | [`note_show`]        | `todo note show <ID>`           |
//! | [`project_add`]      | `todo project add`              |
//! | [`project_clear`]    | `todo project clear`            |
//! | [`project_done`]     | `todo project done <ID>`        |
//! | [`project_undone`]   | `todo project undone <ID>`      |
//! | [`project_edit`]     | `todo project edit <ID>`        |
//! | [`project_list`]     | `todo project list`             |
//! | [`project_remove`]   | `todo project remove <ID>`      |
//! | [`project_show`]     | `todo project show <ID>`        |
//! | [`resource_add`]     | `todo resource add`             |
//! | [`resource_clear`]   | `todo resource clear`           |
//! | [`resource_edit`]    | `todo resource edit <ID>`       |
//! | [`resource_list`]    | `todo resource list`            |
//! | [`resource_remove`]  | `todo resource remove <ID>`     |
//! | [`resource_show`]    | `todo resource show <ID>`       |
//! | [`purge`]            | `todo purge`                    |
//! | [`search`]           | `todo search <QUERY>`           |
//! | [`stats`]            | `todo stats`                    |
//! | [`sync`]             | `todo sync`                     |
//! | [`tags`]             | `todo tags`                     |

pub mod context;
pub mod note_add;
pub mod note_clear;
pub mod note_edit;
pub mod note_list;
pub mod note_remove;
pub mod note_show;
pub mod project_add;
pub mod project_clear;
pub mod project_done;
pub mod project_edit;
pub mod project_list;
pub mod project_remove;
pub mod project_show;
pub mod project_undone;
pub mod purge;
pub mod resource_add;
pub mod resource_clear;
pub mod resource_edit;
pub mod resource_list;
pub mod resource_remove;
pub mod resource_show;
pub mod search;
pub mod stats;
pub mod sync;
pub mod tags;
pub mod task_add;
pub mod task_clear;
pub mod task_clear_recur;
pub mod task_deps;
pub mod task_done;
pub mod task_edit;
pub mod task_info;
pub mod task_list;
pub mod task_recur;
pub mod task_remove;
pub mod task_undone;
