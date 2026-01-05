//! Odrill Lockfile - Dependency lock with BLAKE3 checksums

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Lockfile version for forward compatibility
const LOCKFILE_VERSION: u32 = 1;

/// Odrill lockfile containing locked dependencies
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OdrillLockfile {
    pub version: u32,
    #[serde(default)]
    pub packages: HashMap<String, LockedPackage>,
}

/// A locked package with version and checksum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedPackage {
    pub version: String,
    pub checksum: String,
    pub source: String,
}

impl OdrillLockfile {
    /// Create a new empty lockfile
    pub fn new() -> Self {
        Self {
            version: LOCKFILE_VERSION,
            packages: HashMap::new(),
        }
    }

    /// Load lockfile from path, or create new if not exists
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }
        let content = fs::read_to_string(path)?;
        let lockfile: Self = toml::from_str(&content)?;
        Ok(lockfile)
    }

    /// Save lockfile to path
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Add or update a locked package
    pub fn lock_package(&mut self, name: &str, version: &str, checksum: &str, source: &str) {
        self.packages.insert(
            name.to_string(),
            LockedPackage {
                version: version.to_string(),
                checksum: checksum.to_string(),
                source: source.to_string(),
            },
        );
    }

    /// Get a locked package by name
    pub fn get(&self, name: &str) -> Option<&LockedPackage> {
        self.packages.get(name)
    }

    /// Verify a package checksum matches
    pub fn verify(&self, name: &str, checksum: &str) -> bool {
        self.packages
            .get(name)
            .map(|p| p.checksum == checksum)
            .unwrap_or(false)
    }
}

/// Compute BLAKE3 checksum of data
pub fn compute_checksum(data: &[u8]) -> String {
    blake3::hash(data).to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_checksum() {
        let data = b"hello world";
        let hash = compute_checksum(data);
        assert_eq!(hash.len(), 64); // BLAKE3 produces 256-bit (64 hex chars)
    }

    #[test]
    fn test_lockfile_roundtrip() {
        let mut lockfile = OdrillLockfile::new();
        lockfile.lock_package("math", "1.0.0", "abc123", "registry");
        
        assert!(lockfile.verify("math", "abc123"));
        assert!(!lockfile.verify("math", "wrong"));
    }
}
