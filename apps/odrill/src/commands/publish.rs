use anyhow::{Context, Result};
use clap::Args;
// use formats::ModPackage; // Removed
use reqwest::blocking::Client;
use std::fs;
// use walkdir::WalkDir; // Unused

#[derive(Args)]
pub struct PublishArgs {
    #[clap(long, short)]
    pub registry: Option<String>,
}

pub fn run(args: PublishArgs) -> Result<()> {
    let root = std::env::current_dir()?;
    let project = pkg::OdrillProject::load(&root)?;

    println!(
        "📦 Packaging '{}' v{}...",
        project.manifest.package.name, project.manifest.package.version
    );

    // 2. Pack (Scan + Serialize + Compress)
    let bytes = container::pack(&project)?;

    let temp_path = root.join("package.odrl");
    fs::write(&temp_path, &bytes).context("Failed to write package file")?;

    // 3. Upload
    let registry_url = args
        .registry
        .unwrap_or_else(|| "http://localhost:5150".to_string());
    println!("🚀 Uploading to {}...", registry_url);

    let file_content = fs::read(&temp_path)?;

    // Get auth token
    let token = crate::auth::load_token()?;

    let client = Client::new();
    let res = client
        .post(format!("{}/api/packages/publish", registry_url))
        .header("Authorization", format!("Bearer {}", token))
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
