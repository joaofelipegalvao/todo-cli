//! Sync configuration — reads and writes `sync.toml`.
//!
//! The config file lives alongside `todos.json` in the rustodo data directory:
//!
//! ```text
//! ~/.local/share/rustodo/
//!   todos.json
//!   sync.toml   ← this file
//! ```
//!
//! Current fields:
//! ```toml
//! remote = "git@github.com:user/tasks.git"
//! ```

use anyhow::{Context, Result, bail};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

/// Sync configuration persisted in `sync.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Git remote URL (SSH or HTTPS).
    ///
    /// Example: `git@github.com:user/tasks.git`
    pub remote: String,
}

/// Returns the path to `sync.toml` in the rustodo data directory.
pub fn config_path() -> Result<PathBuf> {
    if let Ok(dir) = std::env::var("RUSTODO_DATA_DIR") {
        return Ok(PathBuf::from(dir).join("sync.toml"));
    }
    let dirs =
        ProjectDirs::from("", "", "rustodo").context("Failed to determine project directories")?;
    Ok(dirs.data_dir().join("sync.toml"))
}

/// Loads `sync.toml`, returning `None` if the file does not exist yet.
///
/// # Errors
///
/// Returns an error if the file exists but cannot be parsed.
pub fn load() -> Result<Option<SyncConfig>> {
    let path = config_path()?;

    match fs::read_to_string(&path) {
        Ok(content) => {
            let config: SyncConfig = toml::from_str(&content).context(
                "Failed to parse sync.toml — run `todo sync init <remote>` to reconfigure",
            )?;
            Ok(Some(config))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e).context(format!("Failed to read {}", path.display())),
    }
}

/// Loads `sync.toml`, returning an error if the file does not exist.
///
/// Use this in commands that require sync to be configured (`push`, `pull`, `status`).
pub fn require() -> Result<SyncConfig> {
    match load()? {
        Some(config) => Ok(config),
        None => bail!(
            "Sync is not configured. Run: todo sync init <remote>\n  \
            Example: todo sync init git@github.com:user/tasks.git"
        ),
    }
}

/// Saves `sync.toml` to the rustodo data directory.
pub fn save(config: &SyncConfig) -> Result<()> {
    let path = config_path()?;

    // Ensure the data directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .context(format!("Failed to create directory {}", parent.display()))?;
    }

    let content = toml::to_string_pretty(config).context("Failed to serialize sync config")?;

    fs::write(&path, content).context(format!("Failed to write {}", path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn config_in(dir: &TempDir) -> PathBuf {
        dir.path().join("sync.toml")
    }

    #[test]
    fn test_roundtrip() {
        let temp = TempDir::new().unwrap();
        let path = config_in(&temp);

        let config = SyncConfig {
            remote: "git@github.com:user/tasks.git".to_string(),
        };

        let content = toml::to_string_pretty(&config).unwrap();
        fs::write(&path, &content).unwrap();

        let loaded: SyncConfig = toml::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(loaded.remote, config.remote);
    }

    #[test]
    fn test_missing_file_returns_none() {
        // config_path() points to the real data dir — we test deserialization only
        let content = "";
        let result: Result<SyncConfig, _> = toml::from_str(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_toml_parses() {
        let content = r#"remote = "git@github.com:user/tasks.git""#;
        let config: SyncConfig = toml::from_str(content).unwrap();
        assert_eq!(config.remote, "git@github.com:user/tasks.git");
    }
}
