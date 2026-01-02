use crate::config::OdrillConfig;
use anyhow::{Context, Result};
use clap::Args;
use odrill_formats::ModPackage;
use reqwest::blocking::Client;
use std::fs;
use walkdir::WalkDir;

#[derive(Args)]
pub struct PublishArgs {
    #[clap(long, short)]
    pub registry: Option<String>,
}

pub fn run(args: PublishArgs) -> Result<()> {
    let config = OdrillConfig::load()?;
    println!(
        "📦 Packaging '{}' v{}...",
        config.package.name, config.package.version
    );

    let mut package = ModPackage::new(config.package.name, config.package.version);
    let root = std::env::current_dir()?;

    // 1. Collect files
    for entry in WalkDir::new(&root) {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        // Basic ignore list (naive)
        if path.to_string_lossy().contains("target") || path.to_string_lossy().contains(".git") {
            continue;
        }

        let relative = path
            .strip_prefix(&root)?
            .to_string_lossy()
            .replace("\\", "/");
        let content = fs::read(path)?;
        package.add_file(relative, content);
    }

    // 2. Pack (Serialize + Compress)
    // We can use save_to_disk for temp file or implement to_bytes in lib if needed.
    // The lib has save_to_disk. Let's use a temp path.
    let temp_path = root.join("package.odrl");
    package
        .save_to_disk(&temp_path)
        .context("Failed to pack package")?;

    // 3. Upload
    let registry_url = args
        .registry
        .unwrap_or_else(|| "http://localhost:3000".to_string());
    println!("🚀 Uploading to {}...", registry_url);

    let file_content = fs::read(&temp_path)?;

    let client = Client::new();
    let res = client
        .post(format!("{}/packages/publish", registry_url))
        //.header("Authorization", "Bearer TODO")
        .body(file_content)
        .send()?;

    if res.status().is_success() {
        println!("✅ Published successfully!");
    } else {
        println!("❌ Failed: {}", res.text()?);
    }

    // Cleanup
    let _ = fs::remove_file(temp_path);

    Ok(())
}
