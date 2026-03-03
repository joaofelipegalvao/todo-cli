//! Command handlers — one submodule per CLI subcommand.
//!
//! Each submodule exposes a single public `execute` function that accepts a
//! [`Storage`](crate::storage::Storage) reference and the arguments parsed by
//! [`clap`](crate::cli). Commands are intentionally thin: they validate
//! input, delegate business logic to [`models`](crate::models), and persist
//! changes through the storage layer.
//!
//! | Submodule | Subcommand |
//! |---|---|
//! | [`add`] | `todo add` |
//! | [`clear`] | `todo clear` |
//! | [`clear_recur`] | `todo norecur <ID>` (remove recurrence) |
//! | [`deps`] | `todo deps <ID>` |
//! | [`done`] | `todo done <ID>` |
//! | [`edit`] | `todo edit <ID>` |
//! | [`info`] | `todo info` |
//! | [`list`] | `todo list` |
//! | [`projects`] | `todo projects` |
//! | [`recur`] | `todo recur <ID>` |
//! | [`remove`] | `todo remove <ID>` |
//! | [`search`] | `todo search <QUERY>` |
//! | [`stats`] | `todo stats` |
//! | [`tags`] | `todo tags` |
//! | [`undone`] | `todo undone <ID>` |
//! //! | [`purge`] | `todo purge` |

pub mod add;
pub mod clear;
pub mod clear_recur;
pub mod deps;
pub mod done;
pub mod edit;
pub mod info;
pub mod list;
pub mod projects;
pub mod purge;
pub mod recur;
pub mod remove;
pub mod search;
pub mod stats;
pub mod sync;
pub mod tags;
pub mod undone;
