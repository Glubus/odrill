use crate::template_engine::{TemplateContext, render};
use anyhow::{Context, Result};
use clap::Args;
use colored::Colorize;
use pkg::ModPackage;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::fs;

// Default constants removed. We now fetch "odrill" from registry by default.

#[derive(Args)]
pub struct NewArgs {
    /// First argument: Project name OR Template name (if second arg present)
    pub arg1: String,

    /// Second argument: Project name (optional)
    pub arg2: Option<String>,

    #[clap(long, short)]
    pub registry: Option<String>,

    #[clap(long, short)]
    pub force: bool,

    #[clap(long)]
    pub no_git: bool,
}

#[derive(Deserialize)]
struct TemplateSearchResponse {
    guid: String,
    // url: String, // Removed as it's not in the list response
}

#[derive(Deserialize)]
struct DownloadResponse {
    url: String,
}

pub fn run(args: NewArgs) -> Result<()> {
    // 1. Parse arguments
    let (template_name, project_name) = match args.arg2 {
        Some(name) => (Some(args.arg1), name),
        None => (None, args.arg1),
    };

    // 2. Determine target directory
    let current_dir = std::env::current_dir()?;
    let target_dir = if project_name == "." {
        current_dir.clone()
    } else {
        current_dir.join(&project_name)
    };

    // 3. Check target directory
    if target_dir.exists() {
        if !target_dir.is_dir() {
            anyhow::bail!("Target path exists and is not a directory");
        }
        let is_empty = target_dir.read_dir()?.next().is_none();
        if !is_empty && !args.force {
            anyhow::bail!("Target directory is not empty. Use --force to override.");
        }
    } else {
        fs::create_dir_all(&target_dir)?;
    }

    println!(
        "✨ Creating project '{}' in {}",
        project_name,
        target_dir.display()
    );

    // 4. Get Template (Default to "odrill" from registry)
    let template_to_fetch = template_name.as_deref().unwrap_or("odrill");
    let package = fetch_template(template_to_fetch, args.registry.as_deref())?;

    // 5. Render
    // Context needs the actual project name (leaf of path)
    let leaf_name = if project_name == "." {
        current_dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    } else {
        project_name.clone()
    };

    let context = TemplateContext::new(&leaf_name)?;
    render(&package, &target_dir, &context)?;

    // 6. Git Init
    if !args.no_git {
        println!("\n{}", "Initializing git...".cyan());
        match std::process::Command::new("git")
            .args(["init"])
            .current_dir(&target_dir)
            .output()
        {
            Ok(o) if o.status.success() => println!("  {} git", "init".green()),
            _ => println!("  {} git", "skip".yellow()),
        }
    }

    // 7. Success Message
    println!("\n{}", "Project created successfully!".green().bold());
    println!("cd {}\nodrill build", project_name);

    Ok(())
}

fn fetch_template(name: &str, registry_opt: Option<&str>) -> Result<ModPackage> {
    let base_url = registry_opt.unwrap_or("http://localhost:5150/api");
    let client = Client::new();

    println!("🔍 Searching for template '{}'...", name);

    // Search
    let search_res = client
        .get(format!("{}/templates", base_url))
        .query(&[("q", name), ("limit", "1")])
        .send()?
        .error_for_status()?;

    let results: Vec<TemplateSearchResponse> = search_res.json()?;
    let tmpl = results.first().context("Template not found")?;

    println!("📥 Downloading {}...", tmpl.guid);

    // Get download URL
    let dl_res = client
        .get(format!("{}/templates/{}/download", base_url, tmpl.guid))
        .send()?
        .error_for_status()?;
    let dl_info: DownloadResponse = dl_res.json()?;

    // Download File
    // Note: dl_info.url is relative ("/uploads/...") usually. Need to prepend base host.
    // Assuming base_url is "http://host/api", we need "http://host".
    // Hacky URL join:
    let host = base_url
        .rsplit_once("/api")
        .map(|(h, _)| h)
        .unwrap_or(base_url);
    let full_url = format!("{}{}", host, dl_info.url);

    println!("⬇️ Fetching from {}...", full_url);
    let bytes = client.get(full_url).send()?.error_for_status()?.bytes()?;

    // Deserialize (Zstd + Rkyv)
    // Deserialize (Zstd + Rkyv)
    let pkg = container::decode(&bytes).context("Failed to deserialize template package")?;
    Ok(pkg)
}
