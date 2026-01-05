//! odrill install - Download and unpack dependencies

use crate::config::OdrillConfig;
use crate::constants::DEFAULT_REGISTRY;
use anyhow::{Context, Result, anyhow};
use colored::Colorize;
use pkg::{OdrillLockfile, compute_checksum};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct VersionDetail {
    #[allow(dead_code)]
    version: String,
    url: String,
}

pub fn run() -> Result<()> {
    let config = OdrillConfig::load()?;

    if config.dependencies.is_empty() {
        println!("No dependencies to install.");
        return Ok(());
    }

    println!("📦 Installing dependencies...");
    let registry_url =
        std::env::var("ODRILL_REGISTRY").unwrap_or_else(|_| DEFAULT_REGISTRY.to_string());

    let client = Client::new();
    let pkg_dir = Path::new("target").join("pkg");
    fs::create_dir_all(&pkg_dir)?;

    // Load existing lockfile
    let lock_path = Path::new("odrill.lock");
    let mut lockfile = OdrillLockfile::load(lock_path)?;
    let mut lockfile_updated = false;

    // Queue for recursive installation: (name, version)
    use std::collections::HashSet;
    use std::collections::VecDeque;

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    for (name, version) in &config.dependencies {
        queue.push_back((name.clone(), version.clone()));
    }

    while let Some((name, version)) = queue.pop_front() {
        if visited.contains(&name) {
            continue;
        }
        visited.insert(name.clone());

        // Check if already installed with correct checksum
        let dest = pkg_dir.join(&name);

        // Helper to process dependencies of an installed package
        let mut process_params_dependencies = |package_path: &Path| -> Result<()> {
            let sub_config_path = package_path.join("odrill.toml");
            if sub_config_path.exists() {
                let sub_config = OdrillConfig::load_from_path(&sub_config_path)?;
                for (dep_name, dep_version) in sub_config.dependencies {
                    if !visited.contains(&dep_name) {
                        queue.push_back((dep_name, dep_version));
                    }
                }
            }
            Ok(())
        };

        if dest.exists() {
            if let Some(locked) = lockfile.get(&name) {
                if locked.version == version {
                    println!("  {} {}@{} (cached)", "skip".yellow(), name, version);
                    // Even if cached, we must scan its dependencies!
                    process_params_dependencies(&dest)?;
                    continue;
                }
            }
        }

        println!("  {} {}@{}", "fetch".blue(), name, version);

        let url = format!("{}/api/packages/{}/{}", registry_url, name, version);
        let response = client
            .get(&url)
            .send()
            .context(format!("Failed to fetch {}", name))?;

        if !response.status().is_success() {
            return Err(anyhow!("Version {} of {} not found", version, name));
        }

        let detail: VersionDetail = response.json()?;

        // Download the .odrl file
        let download_url = format!("{}/{}", registry_url, detail.url);
        let data = client.get(&download_url).send()?.bytes()?;

        // Compute checksum
        let checksum = compute_checksum(&data);

        // Verify against lockfile if exists
        if let Some(locked) = lockfile.get(&name) {
            if locked.checksum != checksum {
                println!("  {} {} checksum mismatch!", "warn".yellow(), name);
            }
        }

        // Decode package
        let pkg = container::decode(&data)?;

        // Unpack to target/pkg/<name>/
        // Ensure clean directory
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
        lockfile.lock_package(&name, &version, &checksum, &registry_url);
        lockfile_updated = true;

        println!(
            "  {} {} ({} files)",
            "unpack".green(),
            name,
            pkg.files.len()
        );

        // Process new dependencies from the unpacked package
        process_params_dependencies(&dest)?;
    }

    // Save lockfile if updated
    if lockfile_updated {
        lockfile.save(lock_path)?;
        println!("🔒 Updated odrill.lock");
    }

    println!("✨ Done!");
    Ok(())
}
