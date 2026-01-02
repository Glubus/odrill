//! odrill clean command - Remove build artifacts

use colored::Colorize;

pub fn run() -> anyhow::Result<()> {
    let project_dir = std::env::current_dir()?;

    println!("{}", "Cleaning build artifacts...".cyan().bold());

    // Remove dist directory
    let dist_dir = project_dir.join("dist");
    if dist_dir.exists() {
        std::fs::remove_dir_all(&dist_dir)?;
        println!("  {} dist/", "remove".red());
    }

    // Remove cache file
    let cache_file = project_dir.join(".odrill-cache.json");
    if cache_file.exists() {
        std::fs::remove_file(&cache_file)?;
        println!("  {} .odrill-cache.json", "remove".red());
    }

    println!("\n{}", "Clean complete!".green().bold());

    Ok(())
}
