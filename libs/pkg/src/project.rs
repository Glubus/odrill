use crate::manifest::OdrillManifest;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
#[derive(Debug, Clone)]
pub struct OdrillProject {
    pub root: PathBuf,
    pub manifest: OdrillManifest,
}

impl OdrillProject {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let root = path.as_ref().to_path_buf();
        let manifest_path = root.join("odrill.toml");

        let content =
            std::fs::read_to_string(&manifest_path).context("Failed to read odrill.toml")?;

        let manifest: OdrillManifest =
            toml::from_str(&content).context("Failed to parse odrill.toml")?;

        Ok(Self { root, manifest })
    }
}

// Reverted
