//! Command handlers — one submodule per CLI subcommand.
//!
//! | Submodule          | Subcommand                      |
//! |--------------------|---------------------------------|
//! | [`task::add`]      | `todo add`                      |
//! | [`task::clear`]    | `todo clear`                    |
//! | [`task::clear_recur`] | `todo norecur <ID>`          |
//! | [`task::deps`]     | `todo deps <ID>`                |
//! | [`task::done`]     | `todo done <ID>`                |
//! | [`task::edit`]     | `todo edit <ID>`                |
//! | [`task::info`]     | `todo info`                     |
//! | [`task::list`]     | `todo list`                     |
//! | [`task::recur`]    | `todo recur <ID>`               |
//! | [`task::remove`]   | `todo remove <ID>`              |
//! | [`task::undone`]   | `todo undone <ID>`              |
//! | [`note::add`]      | `todo note add`                 |
//! | [`note::clear`]    | `todo note clear`               |
//! | [`note::edit`]     | `todo note edit <ID>`           |
//! | [`note::list`]     | `todo note list`                |
//! | [`note::remove`]   | `todo note remove <ID>`         |
//! | [`note::show`]     | `todo note show <ID>`           |
//! | [`project::add`]   | `todo project add`              |
//! | [`project::clear`] | `todo project clear`            |
//! | [`project::done`]  | `todo project done <ID>`        |
//! | [`project::undone`]| `todo project undone <ID>`      |
//! | [`project::edit`]  | `todo project edit <ID>`        |
//! | [`project::list`]  | `todo project list`             |
//! | [`project::remove`]| `todo project remove <ID>`      |
//! | [`project::show`]  | `todo project show <ID>`        |
//! | [`resource::add`]  | `todo resource add`             |
//! | [`resource::clear`]| `todo resource clear`           |
//! | [`resource::edit`] | `todo resource edit <ID>`       |
//! | [`resource::list`] | `todo resource list`            |
//! | [`resource::remove`]| `todo resource remove <ID>`    |
//! | [`resource::show`] | `todo resource show <ID>`       |
//! | [`calendar`]       | `todo calendar [MONTH] [YEAR]`  |
//! | [`context`]        | `todo context <ID>`             |
//! | [`holidays_cmd`]   | `todo holidays`                 |
//! | [`next`]           | `todo next`                     |
//! | [`purge`]          | `todo purge`                    |
//! | [`search`]         | `todo search <QUERY>`           |
//! | [`stats`]          | `todo stats`                    |
//! | [`stats_history`]  | `todo stats history`            |
//! | [`tags`]           | `todo tags`                     |

pub mod note;
pub mod project;
pub mod resource;
pub mod task;

pub mod calendar;
pub mod context;
pub mod holidays_cmd;
pub mod next;
pub mod purge;
pub mod search;
pub mod stats;
pub mod stats_history;
pub mod tags;
