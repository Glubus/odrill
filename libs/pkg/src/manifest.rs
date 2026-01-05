use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OdrillManifest {
    pub package: PackageConfig,
    #[serde(default)]
    pub hooks: Vec<HookConfig>,
    #[serde(default)]
    pub options: OptionsConfig,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default)]
    pub assets: Vec<PathBuf>,
    #[serde(default)]
    pub localization: Vec<LocalizationConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizationConfig {
    pub directory: String,
    pub default: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PackageConfig {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookConfig {
    pub id: String,
    pub entry: PathBuf,
    pub output: PathBuf,
    #[serde(default)]
    pub priority: i32,
}

#[allow(dead_code)]
fn default_hook_type() -> String {
    "post".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptionsConfig {
    // defaults?
}

// Reverted
