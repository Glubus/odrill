use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::{DocumentMut, Item, Table, value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdrillConfig {
    pub package: PackageConfig,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageConfig {
    pub name: String,
    #[serde(default = "default_version")]
    pub version: String,
}

fn default_version() -> String {
    "0.1.0".to_string()
}

impl OdrillConfig {
    pub fn load() -> Result<Self> {
        Self::load_from_path(Path::new("odrill.toml"))
    }

    pub fn load_from_path(path: &Path) -> Result<Self> {
        if !path.exists() {
            anyhow::bail!("{} not found", path.display());
        }
        let content = fs::read_to_string(path)?;
        let config: OdrillConfig =
            toml::from_str(&content).context(format!("Failed to parse {}", path.display()))?;
        Ok(config)
    }

    pub fn get_path() -> PathBuf {
        PathBuf::from("odrill.toml")
    }
}

/// Adds a dependency to odrill.toml preserving comments via toml_edit
pub fn add_dependency(name: &str, version: &str) -> Result<()> {
    let path = OdrillConfig::get_path();
    let content = fs::read_to_string(&path).context("odrill.toml not found")?;
    let mut doc = content
        .parse::<DocumentMut>()
        .context("Failed to parse odrill.toml")?;

    if !doc.contains_key("dependencies") {
        doc["dependencies"] = Item::Table(Table::new());
    }

    doc["dependencies"][name] = value(version);

    fs::write(path, doc.to_string())?;
    Ok(())
}
