//! Configuration for the Lua bundler

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration for a bundle project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleConfig {
    pub package: PackageConfig,
    #[serde(default)]
    pub target: TargetConfig,
    #[serde(default)]
    pub hooks: Vec<HookConfig>,
    #[serde(default)]
    pub options: BundlerOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageConfig {
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetConfig {
    /// Output format: "superblt" or "raw"
    #[serde(default = "default_format")]
    pub format: String,
}

impl Default for TargetConfig {
    fn default() -> Self {
        Self {
            format: "superblt".to_string(),
        }
    }
}

fn default_format() -> String {
    "superblt".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookConfig {
    /// The game file to hook into (e.g., "lib/managers/hudmanagerpd2")
    pub id: String,
    /// Entry point Lua file in src/
    pub entry: PathBuf,
    /// Output path in dist/ (relative, without dist/ prefix)
    pub output: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BundlerOptions {
    /// Enable minification
    #[serde(default)]
    pub minify: bool,
    /// Remove comments
    #[serde(default)]
    pub strip_comments: bool,
    /// Keep original line numbers for debugging
    #[serde(default)]
    pub source_map: bool,
    /// Custom include directive (default: "include")
    #[serde(default = "default_include_directive")]
    pub include_directive: String,
}

fn default_include_directive() -> String {
    "include".to_string()
}

impl Default for BundleConfig {
    fn default() -> Self {
        Self {
            package: PackageConfig {
                name: "my-mod".to_string(),
                version: "0.1.0".to_string(),
                author: String::new(),
                description: String::new(),
            },
            target: TargetConfig::default(),
            hooks: vec![],
            options: BundlerOptions::default(),
        }
    }
}

impl BundleConfig {
    /// Load config from a TOML file
    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: BundleConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save config to a TOML file
    pub fn to_file(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Check if target format is SuperBLT
    pub fn is_superblt(&self) -> bool {
        self.target.format == "superblt"
    }
}
