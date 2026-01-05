//! odrill run - Dev launcher with mod isolation

mod game_detection;
mod isolation;
mod log_watcher;
mod utils;

use anyhow::{Context, Result, anyhow};
use colored::Colorize;
use std::path::{Path, PathBuf};

pub use game_detection::detect_game_path;
pub use isolation::IsolationGuard;
pub use log_watcher::spawn_log_watcher;
pub use utils::{copy_dir_all, copy_dist_to};

/// Run command - Build, install, and launch Payday 2 with mod isolation
pub fn run(path: Option<String>) -> Result<()> {
    // Switch to project directory if specified
    if let Some(p) = path {
        switch_to_project_dir(&p)?;
    }

    let game_path = detect_game_path()?;
    println!("🎮 Found Payday 2 at: {}", game_path.display());

    let mods_dir = game_path.join("mods");
    let overrides_dir = game_path.join("assets").join("mod_overrides");

    // Backup existing mods (Isolation)
    println!("📦 Backing up existing mods...");
    let mods_backup = isolation::backup_dir(&mods_dir)?;
    isolation::backup_dir(&overrides_dir)?;

    // Guard will auto-restore when dropped (end of function or panic)
    let _guard = IsolationGuard {
        mods_dir: mods_dir.clone(),
        overrides_dir: overrides_dir.clone(),
    };

    // Copy essential mods from backup
    copy_essential_mods(&mods_backup, &mods_dir)?;

    // Build current project
    println!("🔨 Building project...");
    if let Err(e) = crate::commands::build::run(false, true) {
        return Err(anyhow!("Build failed: {}", e));
    }

    // Install to game directory
    install_to_game(&mods_dir, &overrides_dir)?;

    // Launch game
    launch_game(&game_path)?;

    Ok(())
}

fn switch_to_project_dir(p: &str) -> Result<()> {
    let mut path_buf = PathBuf::from(p);

    if !path_buf.exists() {
        // Try explicit "mods/" prefix
        let mods_path = Path::new("mods").join(p);
        if mods_path.exists() {
            path_buf = mods_path;
        } else {
            return Err(anyhow!("Project path not found: {}", p));
        }
    }

    std::env::set_current_dir(&path_buf)
        .context(format!("Failed to set CWD to {}", path_buf.display()))?;

    println!("📂 Working in: {}", path_buf.display());
    Ok(())
}

fn copy_essential_mods(mods_backup: &Path, mods_dir: &Path) -> Result<()> {
    // Copy 'base' (BLT Hook)
    let base_src = mods_backup.join("base");
    if base_src.exists() {
        println!("  Copying base/ (BLT Hook)...");
        let base_dest = mods_dir.join("base");
        copy_dir_all(&base_src, &base_dest)?;
    } else {
        println!(
            "{}",
            "⚠️  Warning: 'base' folder not found. BLT might not work.".yellow()
        );
    }

    // Copy 'BeardLib' (Common dependency)
    let beardlib_src = mods_backup.join("BeardLib");
    if beardlib_src.exists() {
        println!("  Copying BeardLib/...");
        let beardlib_dest = mods_dir.join("BeardLib");
        copy_dir_all(&beardlib_src, &beardlib_dest)?;
    }

    // Copy 'saves' (if exists)
    let saves_src = mods_backup.join("saves");
    if saves_src.exists() {
        let saves_dest = mods_dir.join("saves");
        copy_dir_all(&saves_src, &saves_dest)?;
    }

    Ok(())
}

fn install_to_game(mods_dir: &Path, overrides_dir: &Path) -> Result<()> {
    let config = crate::config::OdrillConfig::load()?;
    let mod_dest = mods_dir.join(&config.package.name);

    // Create necessary base folders
    std::fs::create_dir_all(mods_dir)?;
    std::fs::create_dir_all(overrides_dir)?;

    println!("📁 Installing to {}", mod_dest.display());
    copy_dist_to(&mod_dest)?;

    Ok(())
}

fn launch_game(game_path: &Path) -> Result<()> {
    use std::process::Command;

    println!("🚀 Launching Payday 2...");
    let exe = game_path.join("payday2_win32_release.exe");

    // Spawn log watcher
    spawn_log_watcher(game_path);

    let mut child = Command::new(&exe)
        .current_dir(game_path)
        .spawn()
        .context("Failed to launch game")?;

    println!("   Game running with PID: {}", child.id());
    println!("   Waiting for game execution...");

    let start = std::time::Instant::now();
    let status = child.wait()?;
    let duration = start.elapsed();

    println!("🏁 Game exited with: {} (ran for {:.1?})", status, duration);

    // If game ran for less than 5 seconds, wait for user confirmation
    if duration.as_secs() < 5 {
        println!(
            "{}",
            "⚠️  Process exited quickly. Game might still be running (detached).".yellow()
        );
        println!(
            "{}",
            "🔴 Press ENTER to restore environment and exit..."
                .red()
                .bold()
        );

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
    } else {
        println!("✨ Session finished. Restoring...");
    }

    Ok(())
}
