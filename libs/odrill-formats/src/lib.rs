use rkyv::{Archive, Deserialize, Serialize};
use std::{collections::HashMap, path::Path};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PackageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

#[derive(Archive, Deserialize, Serialize, Debug)]
#[archive(check_bytes)] // Enables validation for trusted deserialization
pub struct ModPackage {
    pub name: String,
    pub version: String,
    // Relative path -> File content
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

    /// Serializes the package using rkyv and compresses deeply with Zstd.
    pub fn save_to_disk(&self, out_path: &Path) -> Result<(), PackageError> {
        // 1. Serialize to bytes (rkyv)
        let bytes = rkyv::to_bytes::<_, 4096>(self)
            .map_err(|e| PackageError::Serialization(e.to_string()))?;

        // 2. Compress (zstd) - using max compression for distribution
        let compressed = zstd::stream::encode_all(&bytes[..], 19)?; // Level 19 = Max standard

        // 3. Write to disk
        std::fs::write(out_path, compressed)?;
        Ok(())
    }

    /// Reads from disk, decompresses Zstd, and accesses the archive via rkyv.
    /// Returns the raw bytes to keep the archive valid (Zero-Copy possibility).
    pub fn load_from_disk(path: &Path) -> Result<ModPackage, PackageError> {
        let compressed = std::fs::read(path)?;
        let bytes = zstd::stream::decode_all(&compressed[..])?;

        // Fully deserialize for now (simpler than handling AlignedVec lifetimes for Archived<ModPackage>)
        let package: ModPackage =
            rkyv::from_bytes(&bytes).map_err(|e| PackageError::Deserialization(e.to_string()))?;

        Ok(package)
    }
}
