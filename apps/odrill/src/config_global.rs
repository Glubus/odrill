use anyhow::{Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalConfig {
    pub author: Option<String>,
    pub api_token: Option<String>,
    pub pd2_path: Option<String>,
}

impl GlobalConfig {
    pub fn get_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().context("Could not find config directory")?;
        Ok(config_dir.join("odrill").join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::get_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)?;
        let config: GlobalConfig =
            toml::from_str(&content).context("Failed to parse config.toml")?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "author" => self.author = Some(value.to_string()),
            "api_token" => self.api_token = Some(value.to_string()),
            "pd2" | "path" => self.pd2_path = Some(value.to_string()),
            _ => anyhow::bail!("Unknown config key: {}", key),
        }
        self.save()?;
        println!("{} {} = {}", "Updated".green(), key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "author" => self.author.clone(),
            "api_token" => self.api_token.clone(),
            "pd2" => self.pd2_path.clone(),
            _ => None,
        }
    }
}
