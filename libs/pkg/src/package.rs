use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// In-memory representation of a Mod Package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModPackage {
    pub name: String,
    pub version: String,
    // Files: Relative path -> Content
    pub files: HashMap<String, Vec<u8>>,
}

impl ModPackage {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            files: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, path: impl Into<String>, content: Vec<u8>) {
        self.files.insert(path.into(), content);
    }

    pub fn get_file(&self, path: &str) -> Option<&Vec<u8>> {
        self.files.get(path)
    }
}
