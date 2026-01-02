use crate::config;
use anyhow::{Context, Result};
use colored::Colorize;
use reqwest::blocking::Client;

pub fn run(pkg_name: &str, _output: Option<&str>) -> Result<()> {
    println!("🔍 Searching for package '{}'...", pkg_name);

    let _registry_url = "http://localhost:3000"; // TODO config
    let _client = Client::new();

    // Stub retrieval
    let version = "0.1.0"; // Mock

    println!("✅ Found {} v{}", pkg_name.green(), version);

    config::add_dependency(pkg_name, version).context("Failed to update odrill.toml")?;

    println!("📝 Added to [dependencies]");
    Ok(())
}
