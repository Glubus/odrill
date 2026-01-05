//! Auth utilities for token management

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

fn get_token_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".odrill")
        .join("token")
}

/// Save JWT token to ~/.odrill/token
pub fn save_token(token: &str) -> Result<()> {
    let path = get_token_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, token).context("Failed to save token")?;
    Ok(())
}

/// Load JWT token from ~/.odrill/token
pub fn load_token() -> Result<String> {
    let path = get_token_path();
    fs::read_to_string(&path).context("Not logged in. Run 'odrill login' first.")
}

/// Check if user is logged in
#[allow(dead_code)]
pub fn is_logged_in() -> bool {
    get_token_path().exists()
}

/// Clear saved token (logout)
pub fn clear_token() -> Result<()> {
    let path = get_token_path();
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(())
}
