use crate::ops::pack::pack;
use crate::project::TemplateProject;
use anyhow::{Context, Result, anyhow};
use colored::Colorize;
use reqwest::blocking::Client;
use reqwest::blocking::multipart;
use std::fs;
use std::path::Path;

pub fn publish(path: impl AsRef<Path>, registry_url: &str, token: &str) -> Result<()> {
    let root = path.as_ref();

    // 1. Pack (Includes validation)
    let bytes = pack(root)?;

    // 2. Load Project for logging (re-loading is cheap/safe)
    let project = TemplateProject::load(root)?;
    let name = project.manifest.template.name;
    let version = project.manifest.template.version;

    println!(
        "🚀 Uploading '{}' v{} to {}...",
        name.cyan(),
        version.cyan(),
        registry_url
    );

    // 3. Create Temp File for Upload (multipart requires file or io stream)
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path().join("template.odrl");
    fs::write(&temp_path, &bytes).context("Failed to write temporary package file")?;

    // 4. Upload
    let client = Client::new();
    let form = multipart::Form::new().file("file", &temp_path)?;

    let res = client
        .post(format!("{}/templates/publish", registry_url))
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send()
        .context("Failed to send socket request")?;

    if res.status().is_success() {
        println!("✅ Template published successfully!");
        Ok(())
    } else {
        let text = res.text().unwrap_or_default();
        Err(anyhow!("Failed to publish template: {}", text))
    }
}
