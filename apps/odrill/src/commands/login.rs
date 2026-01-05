//! odrill login - Authenticate with registry
use crate::auth;
use crate::constants::DEFAULT_REGISTRY;
use anyhow::{Context, Result, anyhow};
use colored::Colorize;
use reqwest::blocking::Client;
use std::io::{self, Write};

pub fn run() -> Result<()> {
    println!("🔐 Login to Odrill Registry");
    println!("Please enter your API Key found on your profile page.\n");

    let api_key = prompt("API Key: ")?;

    if api_key.is_empty() {
        return Err(anyhow!("API Key cannot be empty"));
    }

    // Verify key by hitting a protected endpoint
    let registry_url =
        std::env::var("ODRILL_REGISTRY").unwrap_or_else(|_| DEFAULT_REGISTRY.to_string());

    // We try to fetch user profile to verify token
    let client = Client::new();
    let url = format!("{}/api/user", registry_url);

    println!("\n⏳ Verifying key...");

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .context("Failed to connect to registry")?;

    if !response.status().is_success() {
        return Err(anyhow!("Invalid API Key"));
    }

    auth::save_token(&api_key)?;

    println!("✅ Authenticated successfully! {}", "Saved.".green());
    Ok(())
}

fn prompt(label: &str) -> Result<String> {
    print!("{}", label);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
