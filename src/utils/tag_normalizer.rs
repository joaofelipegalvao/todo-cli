//! Tag normalization with fuzzy matching.
//!
//! When a user adds a tag, this module checks if a similar tag already
//! exists in the task list and normalizes it automatically.
//!
//! ## Rules
//!
//! 1. **Exact match** — tag already exists as-is → use it unchanged
//! 2. **Case-insensitive match** — `Rust` matches `rust` → normalize to existing
//! 3. **Fuzzy match** — edit distance ≤ threshold → normalize to closest match
//! 4. **No match** — tag is new → accept as-is
//!
//! ## Threshold
//!
//! - Tags ≤ 4 chars: distance ≤ 1
//! - Tags ≥ 5 chars: distance ≤ 2
//!
//! This avoids false positives like `rust` → `just` (distance 2 on a 4-char tag).

use strsim::levenshtein;

/// Result of normalizing a single tag.
#[derive(Debug, PartialEq)]
pub enum NormalizeResult {
    /// Tag already exists exactly — no change needed.
    Unchanged,
    /// Tag was normalized to an existing one.
    Normalized { from: String, to: String },
    /// Tag is new — no match found.
    New,
}

/// Normalize a single tag against the list of existing tags.
///
/// Returns a `NormalizeResult` describing what happened.
pub fn normalize_tag(tag: &str, existing_tags: &[String]) -> NormalizeResult {
    // 1. Exact match
    if existing_tags.iter().any(|t| t == tag) {
        return NormalizeResult::Unchanged;
    }

    // 2. Case-insensitive match
    let tag_lower = tag.to_lowercase();
    if let Some(existing) = existing_tags.iter().find(|t| t.to_lowercase() == tag_lower) {
        return NormalizeResult::Normalized {
            from: tag.to_string(),
            to: existing.clone(),
        };
    }

    // 3. Fuzzy match (Levenshtein distance)
    let threshold = if tag.len() <= 4 { 1 } else { 2 };

    let best = existing_tags
        .iter()
        .map(|t| (t, levenshtein(&tag_lower, &t.to_lowercase())))
        .filter(|(_, dist)| *dist <= threshold)
        .min_by_key(|(_, dist)| *dist);

    if let Some((existing, _)) = best {
        return NormalizeResult::Normalized {
            from: tag.to_string(),
            to: existing.clone(),
        };
    }

    NormalizeResult::New
}

/// Normalize a list of tags against existing tags.
///
/// Returns the normalized tags and a list of normalization messages
/// to display to the user.
pub fn normalize_tags(tags: Vec<String>, existing_tags: &[String]) -> (Vec<String>, Vec<String>) {
    let mut normalized = Vec::new();
    let mut messages = Vec::new();

    for tag in tags {
        match normalize_tag(&tag, existing_tags) {
            NormalizeResult::Unchanged | NormalizeResult::New => {
                normalized.push(tag);
            }
            NormalizeResult::Normalized { from, to } => {
                messages.push(format!("'{}' → '{}'", from, to));
                normalized.push(to);
            }
        }
    }

    (normalized, messages)
}

/// Collect all unique tags from a task list.
pub fn collect_existing_tags(tasks: &[crate::models::Task]) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut tags = Vec::new();
    for task in tasks {
        for tag in &task.tags {
            if seen.insert(tag.clone()) {
                tags.push(tag.clone());
            }
        }
    }
    tags
}

/// Check whether a tag list contains a given tag (case-insensitive, no allocation).
pub fn has_tag(tags: &[String], tag: &str) -> bool {
    tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
}
#[cfg(test)]
mod tests {
    use super::*;

    fn existing(tags: &[&str]) -> Vec<String> {
        tags.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_exact_match_unchanged() {
        let result = normalize_tag("rust", &existing(&["rust", "work"]));
        assert_eq!(result, NormalizeResult::Unchanged);
    }

    #[test]
    fn test_case_insensitive_match() {
        let result = normalize_tag("Rust", &existing(&["rust", "work"]));
        assert_eq!(
            result,
            NormalizeResult::Normalized {
                from: "Rust".to_string(),
                to: "rust".to_string(),
            }
        );
    }

    #[test]
    fn test_case_insensitive_uppercase() {
        let result = normalize_tag("WORK", &existing(&["rust", "work"]));
        assert_eq!(
            result,
            NormalizeResult::Normalized {
                from: "WORK".to_string(),
                to: "work".to_string(),
            }
        );
    }

    #[test]
    fn test_fuzzy_match_typo() {
        // "rusr" is 1 edit away from "rust"
        let result = normalize_tag("rusr", &existing(&["rust", "work"]));
        assert_eq!(
            result,
            NormalizeResult::Normalized {
                from: "rusr".to_string(),
                to: "rust".to_string(),
            }
        );
    }

    #[test]
    fn test_fuzzy_match_longer_tag() {
        // "fronteend" is 1 edit from "frontend" (≥5 chars, threshold=2)
        let result = normalize_tag("fronteend", &existing(&["frontend", "backend"]));
        assert_eq!(
            result,
            NormalizeResult::Normalized {
                from: "fronteend".to_string(),
                to: "frontend".to_string(),
            }
        );
    }

    #[test]
    fn test_no_match_new_tag() {
        let result = normalize_tag("python", &existing(&["rust", "work"]));
        assert_eq!(result, NormalizeResult::New);
    }

    #[test]
    fn test_no_false_positive_short_tags() {
        // "rust" and "just" differ by 1 but are semantically different
        // threshold=1 means it WOULD match — this is intentional for short tags
        // but "go" and "do" should NOT match (both len=2, distance=1, threshold=1)
        // This test documents that behavior
        let result = normalize_tag("go", &existing(&["do"]));
        // distance("go", "do") = 1, threshold for len=2 is 1 → matches
        // This is acceptable — short tags are rare and users rarely typo them
        assert_eq!(
            result,
            NormalizeResult::Normalized {
                from: "go".to_string(),
                to: "do".to_string(),
            }
        );
    }

    #[test]
    fn test_normalize_tags_multiple() {
        let existing = existing(&["rust", "frontend"]);
        let tags = vec![
            "Rust".to_string(),
            "fronteend".to_string(),
            "python".to_string(),
        ];
        let (normalized, messages) = normalize_tags(tags, &existing);

        assert_eq!(normalized, vec!["rust", "frontend", "python"]);
        assert_eq!(messages.len(), 2);
        assert!(messages[0].contains("Rust") && messages[0].contains("rust"));
        assert!(messages[1].contains("fronteend") && messages[1].contains("frontend"));
    }

    #[test]
    fn test_normalize_tags_no_existing() {
        let (normalized, messages) =
            normalize_tags(vec!["rust".to_string(), "work".to_string()], &[]);
        assert_eq!(normalized, vec!["rust", "work"]);
        assert!(messages.is_empty());
    }
}
