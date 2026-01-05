//! odrill add - Add a package dependency and download it

use crate::config;
use crate::constants::DEFAULT_REGISTRY;
use anyhow::{Context, Result, anyhow};
use colored::Colorize;
use pkg::{OdrillLockfile, compute_checksum};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct PackageInfo {
    name: String,
    latest_version: String,
}

#[derive(Deserialize)]
struct VersionDetail {
    #[allow(dead_code)]
    version: String,
    url: String,
}

pub fn run(pkg_name: &str, _output: Option<&str>) -> Result<()> {
    println!("🔍 Searching for package '{}'...", pkg_name);

    let registry_url =
        std::env::var("ODRILL_REGISTRY").unwrap_or_else(|_| DEFAULT_REGISTRY.to_string());

    let client = Client::new();

    // 1. Get package info
    let url = format!("{}/api/packages/{}", registry_url, pkg_name);
    let response = client
        .get(&url)
        .send()
        .context("Failed to connect to registry")?;

    if !response.status().is_success() {
        return Err(anyhow!("Package '{}' not found", pkg_name));
    }

    let info: PackageInfo = response.json().context("Invalid response from registry")?;
    println!("✅ Found {} v{}", info.name.green(), info.latest_version);

    // 2. Get version details
    let version_url = format!(
        "{}/api/packages/{}/{}",
        registry_url, pkg_name, info.latest_version
    );
    let version_response = client.get(&version_url).send()?;

    if !version_response.status().is_success() {
        return Err(anyhow!("Failed to get version details"));
    }

    let detail: VersionDetail = version_response.json()?;

    // 3. Download the .odrl file
    println!("📥 Downloading...");
    let download_url = format!("{}/{}", registry_url, detail.url);
    let data = client.get(&download_url).send()?.bytes()?;

    // 4. Compute BLAKE3 checksum
    let checksum = compute_checksum(&data);
    println!("🔐 Checksum: {}", &checksum[..16]);

    // 5. Decode package
    let pkg = container::decode(&data)?;

    // 6. Extract to target/pkg/<name>/
    let pkg_dir = Path::new("target").join("pkg").join(&info.name);
    fs::create_dir_all(&pkg_dir)?;

    for (path, content) in &pkg.files {
        let file_path = pkg_dir.join(path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&file_path, content)?;
    }

    println!(
        "📦 Extracted {} files to {}",
        pkg.files.len(),
        pkg_dir.display()
    );

    // 7. Update odrill.toml
    config::add_dependency(&info.name, &info.latest_version)
        .context("Failed to update odrill.toml")?;
    println!("📝 Updated [dependencies] in odrill.toml");

    // 8. Update odrill.lock
    let lock_path = Path::new("odrill.lock");
    let mut lockfile = OdrillLockfile::load(lock_path)?;
    lockfile.lock_package(&info.name, &info.latest_version, &checksum, &registry_url);
    lockfile.save(lock_path)?;
    println!("🔒 Updated odrill.lock");

    println!("✨ Done!");
    Ok(())
}
