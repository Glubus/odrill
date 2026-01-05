//! Mod isolation system with backup and restore

use crate::constants::BACKUP_PREFIX;
use anyhow::{Context, Result, anyhow};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

/// RAII guard that automatically restores backed-up directories on drop
pub struct IsolationGuard {
    pub mods_dir: PathBuf,
    pub overrides_dir: PathBuf,
}

impl Drop for IsolationGuard {
    fn drop(&mut self) {
        println!("\n🧹 Restoring environment...");

        if let Err(e) = restore_dir(&self.mods_dir) {
            eprintln!("Failed to restore mods: {}", e);
        }

        if let Err(e) = restore_dir(&self.overrides_dir) {
            eprintln!("Failed to restore mod_overrides: {}", e);
        }
    }
}

/// Backup a directory by renaming it with a ~ prefix
///
/// Returns the path to the backup directory
pub fn backup_dir(dir: &Path) -> Result<PathBuf> {
    if !dir.exists() {
        return Ok(dir.to_path_buf());
    }

    let dir_name = dir
        .file_name()
        .ok_or_else(|| anyhow!("Invalid path: no filename in {}", dir.display()))?;

    let backup_name = format!("{}{}", BACKUP_PREFIX, dir_name.to_string_lossy());

    let parent = dir
        .parent()
        .ok_or_else(|| anyhow!("Invalid path: no parent for {}", dir.display()))?;

    let backup_path = parent.join(&backup_name);

    if backup_path.exists() {
        println!(
            "  (Backup {} already exists, using it)",
            backup_path.display()
        );

        // Clean the current dir (dev environment)
        if let Err(e) = fs::remove_dir_all(dir) {
            return Err(anyhow!(
                "Failed to remove {}: {}\nMake sure the game is closed!",
                dir.display(),
                e
            ));
        }

        return Ok(backup_path);
    }

    // Rename original to backup
    if let Err(e) = fs::rename(dir, &backup_path) {
        return Err(anyhow!(
            "Failed to backup {}: {}\nMake sure the game is closed!",
            dir.display(),
            e
        ));
    }

    // Create fresh dir
    fs::create_dir_all(dir)?;

    let backup_file_name = backup_path
        .file_name()
        .ok_or_else(|| anyhow!("Invalid backup path: {}", backup_path.display()))?;

    println!(
        "  {} -> {}",
        dir_name.to_string_lossy().dimmed(),
        backup_file_name.to_string_lossy().green()
    );

    Ok(backup_path)
}

/// Restore a directory from its backup
pub fn restore_dir(dir: &Path) -> Result<()> {
    let dir_name = dir
        .file_name()
        .ok_or_else(|| anyhow!("Invalid path: no filename in {}", dir.display()))?;

    let backup_name = format!("{}{}", BACKUP_PREFIX, dir_name.to_string_lossy());

    let parent = dir
        .parent()
        .ok_or_else(|| anyhow!("Invalid path: no parent for {}", dir.display()))?;

    let backup_path = parent.join(&backup_name);

    if !backup_path.exists() {
        return Ok(());
    }

    // Remove the dev folder with retry logic (Windows file locks)
    if dir.exists() {
        let mut retries = 3;

        while retries > 0 {
            if fs::remove_dir_all(dir).is_err() {
                std::thread::sleep(std::time::Duration::from_millis(500));
                retries -= 1;
            } else {
                break;
            }
        }

        if dir.exists() {
            fs::remove_dir_all(dir).context("Failed to cleanup dev folder")?;
        }
    }

    fs::rename(&backup_path, dir)?;

    let backup_file_name = backup_path
        .file_name()
        .ok_or_else(|| anyhow!("Invalid backup path: {}", backup_path.display()))?;

    println!(
        "  {} -> {}",
        backup_file_name.to_string_lossy().green(),
        dir_name.to_string_lossy().dimmed()
    );

    Ok(())
}
