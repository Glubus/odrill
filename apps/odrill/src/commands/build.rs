//! odrill build command - Bundle all hooks

use colored::Colorize;
use compiler::{Compiler, superblt::generate_superblt_files};
use pkg::OdrillProject;
use std::time::Instant;

pub fn run(force: bool, _watch: bool) -> anyhow::Result<()> {
    let project_dir = std::env::current_dir()?;
    // Load project
    let project = OdrillProject::load(&project_dir)?;

    // We clone project for the compiler, but we also nede it for superblt generation later.
    // Compiler consumes it or takes reference?
    // Compiler::new(project) takes ownership. OdrillProject is Clone?
    // Project might be expensive to clone if it lists many files? No, just manifest and root.
    // Assuming OdrillProject derives Clone. Check project.rs?

    println!("{}", "Building project...".cyan().bold());
    let start = Instant::now();

    let mut compiler = Compiler::new(project.clone());

    if force {
        println!("  {} cache", "clear".yellow());
    }

    let results = compiler.compile_all()?;
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
    // Check manifest options. Assuming is_superblt() check was implicitly "if hooks > 0" or strictly structure.
    // Payday 2 mods are usually SuperBLT.
    // We can just always generate them or check a flag.
    // For now, always generate if output mode is not specified or assumes SuperBLT.
    // Or check manifest.options.

    let dist_dir = project_dir.join("dist");

    // TODO: Add condition for superblt generation
    std::fs::create_dir_all(&dist_dir)?;
    generate_superblt_files(&project.manifest, &dist_dir, &project_dir)?;

    // Count loc files if any
    let loc_dir = project_dir.join("loc");
    if loc_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&loc_dir) {
            let loc_count = entries.count();
            if loc_count > 0 {
                println!("  {} loc/ ({} files)", "copy".green(), loc_count);
            }
        }
    }
    println!("  {} mod.txt, main.xml", "generate".green());

    println!(
        "\n{} {} bundled, {} cached in {:.2}s",
        "Done!".green().bold(),
        bundled,
        cached,
        elapsed.as_secs_f64()
    );

    println!("\nOutput: {}", dist_dir.display().to_string().cyan());
    println!("Copy dist/ contents to your PAYDAY 2 mods folder to test.");

    Ok(())
}
