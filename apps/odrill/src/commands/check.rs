//! odrill check - Validate project, dependencies and Lua code
//!
//! This command:
//! 1. Verifies dependencies are present (auto-downloads if missing)
//! 2. Runs static analysis on Lua source files
//! 3. Reports diagnostics (errors, warnings, info)

use crate::config::OdrillConfig;
use crate::constants::DEFAULT_REGISTRY;
use anyhow::{Context, Result};
use colored::Colorize;
use linter::{Diagnostic, DiagnosticLevel, LuaAnalyzer};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Run the check command
pub fn run(fix: bool) -> Result<()> {
    println!("🔍 Checking project...\n");

    let config = OdrillConfig::load()?;
    let mut total_errors = 0;
    let mut total_warnings = 0;

    // Step 1: Check/install dependencies
    println!("{}", "Dependencies".bold());
    let deps_ok = check_dependencies(&config)?;
    if !deps_ok {
        total_errors += 1;
    }
    println!();

    // Step 2: Lint Lua files
    println!("{}", "Static Analysis".bold());
    let diagnostics = lint_lua_files(&config)?;

    for diag in &diagnostics {
        print_diagnostic(diag);
        match diag.level {
            DiagnosticLevel::Error => total_errors += 1,
            DiagnosticLevel::Warning => total_warnings += 1,
            DiagnosticLevel::Info => {}
        }
    }

    if diagnostics.is_empty() {
        println!("  {} No issues found", "✓".green());
    }
    println!();

    // Summary
    println!("{}", "Summary".bold());
    if total_errors > 0 {
        println!(
            "  {} {} error(s), {} warning(s)",
            "✗".red(),
            total_errors,
            total_warnings
        );
        std::process::exit(1);
    } else if total_warnings > 0 {
        println!("  {} {} warning(s)", "⚠".yellow(), total_warnings);
    } else {
        println!("  {} All checks passed!", "✓".green());
    }

    if fix {
        println!("\n{}", "Note: --fix is not yet implemented".dimmed());
    }

    Ok(())
}

/// Check and auto-install missing dependencies
fn check_dependencies(config: &OdrillConfig) -> Result<bool> {
    if config.dependencies.is_empty() {
        println!("  {} No dependencies", "○".dimmed());
        return Ok(true);
    }

    let pkg_dir = Path::new("target").join("pkg");
    let mut missing: Vec<(&String, &String)> = Vec::new();

    for (name, version) in &config.dependencies {
        let dep_path = pkg_dir.join(name);
        if dep_path.exists() {
            println!("  {} {}@{}", "✓".green(), name, version);
        } else {
            println!("  {} {}@{} (missing)", "✗".red(), name, version);
            missing.push((name, version));
        }
    }

    // Auto-download missing dependencies
    if !missing.is_empty() {
        println!("\n  {} Fetching missing dependencies...", "↓".blue());

        for (name, version) in &missing {
            match fetch_dependency(name, version) {
                Ok(_) => println!("    {} {}@{}", "✓".green(), name, version),
                Err(e) => {
                    println!("    {} {}@{}: {}", "✗".red(), name, version, e);
                    return Ok(false);
                }
            }
        }
    }

    Ok(true)
}

/// Fetch a single dependency from the registry
fn fetch_dependency(name: &str, version: &str) -> Result<()> {
    use pkg::{OdrillLockfile, compute_checksum};
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct VersionDetail {
        #[allow(dead_code)]
        version: String,
        url: String,
    }

    let registry_url =
        std::env::var("ODRILL_REGISTRY").unwrap_or_else(|_| DEFAULT_REGISTRY.to_string());

    let client = reqwest::blocking::Client::new();
    let pkg_dir = Path::new("target").join("pkg");
    fs::create_dir_all(&pkg_dir)?;

    let url = format!("{}/api/packages/{}/{}", registry_url, name, version);
    let response = client
        .get(&url)
        .send()
        .context(format!("Failed to fetch {}", name))?;

    if !response.status().is_success() {
        anyhow::bail!("Version {} not found", version);
    }

    let detail: VersionDetail = response.json()?;
    let download_url = format!("{}/{}", registry_url, detail.url);
    let data = client.get(&download_url).send()?.bytes()?;

    // Compute checksum
    let checksum = compute_checksum(&data);

    // Decode and unpack
    let pkg = container::decode(&data)?;
    let dest = pkg_dir.join(name);

    if dest.exists() {
        fs::remove_dir_all(&dest)?;
    }
    fs::create_dir_all(&dest)?;

    for (path, content) in &pkg.files {
        let file_path = dest.join(path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&file_path, content)?;
    }

    // Update lockfile
    let lock_path = Path::new("odrill.lock");
    let mut lockfile = OdrillLockfile::load(lock_path)?;
    lockfile.lock_package(name, version, &checksum, &registry_url);
    lockfile.save(lock_path)?;

    Ok(())
}

/// Lint all Lua files in src/
fn lint_lua_files(config: &OdrillConfig) -> Result<Vec<Diagnostic>> {
    let src_dir = Path::new("src");
    if !src_dir.exists() {
        return Ok(Vec::new());
    }

    let mut analyzer = LuaAnalyzer::new();

    // Add globals from installed dependencies
    let dep_globals = collect_dependency_globals(config)?;
    analyzer.add_globals(dep_globals);

    let mut all_diagnostics = Vec::new();

    // Walk src/ for .lua files
    for entry in walkdir::WalkDir::new(src_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "lua"))
    {
        let path = entry.path();
        let source =
            fs::read_to_string(path).context(format!("Failed to read {}", path.display()))?;

        let diagnostics = analyzer.analyze_file(path, &source);
        all_diagnostics.extend(diagnostics);
    }

    Ok(all_diagnostics)
}

/// Collect exported globals from installed dependencies
fn collect_dependency_globals(config: &OdrillConfig) -> Result<HashSet<String>> {
    let mut globals = HashSet::new();
    let pkg_dir = Path::new("target").join("pkg");

    for name in config.dependencies.keys() {
        let dep_manifest_path = pkg_dir.join(name).join("odrill.toml");
        if dep_manifest_path.exists() {
            // Try to extract exports from the dependency
            // For now, just add the package name as a global
            globals.insert(name.clone());
        }
    }

    Ok(globals)
}

/// Print a diagnostic with colored output
fn print_diagnostic(diag: &Diagnostic) {
    let code = diag.code.as_str();
    let location = format!("{}:{}:{}", diag.file.display(), diag.line, diag.column);

    println!(
        "  {} [{}] {}: {}",
        diag.level.symbol(),
        code.dimmed(),
        location.dimmed(),
        diag.message
    );
}
