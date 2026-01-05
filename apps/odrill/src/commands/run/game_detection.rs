//! Payday 2 game path detection

use crate::config_global::GlobalConfig;
use anyhow::{Context, Result, anyhow};
use std::path::PathBuf;

/// Detect the Payday 2 installation path from global config
///
/// Requires `odrill config set path` to be configured.
/// No automatic fallback to Steam paths.
pub fn detect_game_path() -> Result<PathBuf> {
    let config = GlobalConfig::load().context("Failed to load global config")?;

    let path = config.pd2_path
        .ok_or_else(|| anyhow!(
            "Payday 2 path not configured.\nPlease run: odrill config set path \"<PATH_TO_PAYDAY_2>\""
        ))?;

    let p = PathBuf::from(&path);

    if !p.exists() {
        return Err(anyhow!(
            "Configured path does not exist: {}\nPlease update with: odrill config set path \"<PATH>\"",
            path
        ));
    }

    Ok(p)
}
