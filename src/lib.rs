//! # rustodo
//!
//! A fast, powerful, and colorful task manager for the terminal
//!
//! ## Library structure
//!
//! | Module | Purpose |
//! |---|---|
//! | [`cli`] | Command-line argument definitions (clap) |
//! | [`commands`] | One submodule per CLI command |
//! | [`date_parser`] | Natural language and strict date parsing |
//! | [`render`] | Table rendering and formatting |
//! | [`error`] | Typed error variants via `thiserror` |
//! | [`models`] | Core domain types: `Task`, `Priority`, `Recurrence` |
//! | [`services`] | Domain services: tag aggregation and cross-entity logic |
//! | [`storage`] | Storage trait with JSON and in-memory implementations |
//! | [`tui`] | Terminal User Interface (Ratatui) |
//! | [`validation`] | Input validation for task fields |

pub mod cli;
pub mod commands;
pub mod date_parser;
pub mod error;
pub mod models;
pub mod render;
pub mod services;
pub mod storage;
pub mod sync;
pub mod tui;
pub mod utils;
pub mod validation;
