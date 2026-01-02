//! odrill build command - Bundle all hooks

use colored::Colorize;
use lua_bundler::{Bundler, generate_superblt_files};
use std::time::Instant;

pub fn run(force: bool, _watch: bool) -> anyhow::Result<()> {
    let project_dir = std::env::current_dir()?;
    let config_path = project_dir.join("odrill.toml");

    if !config_path.exists() {
        anyhow::bail!("No odrill.toml found. Run 'odrill init' first.");
    }

    println!("{}", "Building project...".cyan().bold());
    let start = Instant::now();

    let mut bundler = Bundler::from_project(project_dir.clone())?;

    if force {
        println!("  {} cache", "clear".yellow());
    }

    let results = bundler.bundle_all()?;
    let elapsed = start.elapsed();

    let mut bundled = 0;
    let mut cached = 0;

    for result in &results {
        if result.was_cached {
            cached += 1;
            println!(
                "  {} {} (cached)",
                "skip".yellow(),
                result.output_path.display()
            );
        } else {
            bundled += 1;
            println!(
                "  {} {} ({} files, {} lines)",
                "bundle".green(),
                result.output_path.display(),
                result.source_files.len(),
                result.lines_total
            );
        }
    }

    // Generate SuperBLT files if target format is superblt
    let config = lua_bundler::BundleConfig::from_file(&config_path)?;
    let dist_dir = project_dir.join("dist");

    if config.is_superblt() {
        std::fs::create_dir_all(&dist_dir)?;
        generate_superblt_files(&config, &dist_dir, &project_dir)?;
        // Count loc files if any
        let loc_dir = project_dir.join("loc");
        if loc_dir.exists() {
            let loc_count = std::fs::read_dir(&loc_dir)?.count();
            if loc_count > 0 {
                println!("  {} loc/ ({} files)", "copy".green(), loc_count);
            }
        }
        println!("  {} mod.txt, main.xml", "generate".green());
    }

    println!(
        "\n{} {} bundled, {} cached in {:.2}s",
        "Done!".green().bold(),
        bundled,
        cached,
        elapsed.as_secs_f64()
    );

    if config.is_superblt() {
        println!("\nOutput: {}", dist_dir.display().to_string().cyan());
        println!("Copy dist/ contents to your PAYDAY 2 mods folder to test.");
    }

    Ok(())
}
