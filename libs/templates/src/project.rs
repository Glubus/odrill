use crate::manifest::TemplateManifest;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct TemplateProject {
    pub root: PathBuf,
    pub manifest: TemplateManifest,
}

impl TemplateProject {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let root = path.as_ref().to_path_buf();
        let manifest_path = root.join("template.toml");

        let content =
            std::fs::read_to_string(&manifest_path).context("Failed to read template.toml")?;

        let manifest: TemplateManifest =
            toml::from_str(&content).context("Failed to parse template.toml")?;

        Ok(Self { root, manifest })
    }
}
