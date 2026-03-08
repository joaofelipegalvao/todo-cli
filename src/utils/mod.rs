//! Shared terminal utilities.
//!
//! | Module | Purpose |
//! |---|---|
//! | [`confirm`] | Yes/no prompt for destructive operations |
//! | [`tag_normalizer`] | Fuzzy tag normalization with Levenshtein distance |

pub mod confirm;
pub mod tag_normalizer;

pub use confirm::confirm;
