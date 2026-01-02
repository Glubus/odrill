//! File modification time based cache

use crate::error::CacheError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Cache file name
const DEFAULT_CACHE_FILE: &str = ".odrill-cache.json";

/// File-based cache tracking modification times
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileCache {
    /// Map of file path to last modified time (unix timestamp)
    files: HashMap<PathBuf, u64>,

    /// Last successful build timestamp
    last_build: Option<u64>,

    /// Project version (for cache invalidation on version changes)
    version: Option<String>,

    /// Path to the cache file (not serialized)
    #[serde(skip)]
    cache_path: PathBuf,
}

impl FileCache {
    /// Create a new empty cache
    pub fn new(project_dir: &Path) -> Self {
        Self {
            files: HashMap::new(),
            last_build: None,
            version: None,
            cache_path: project_dir.join(DEFAULT_CACHE_FILE),
        }
    }

    /// Create cache with a custom cache file name
    pub fn with_filename(project_dir: &Path, filename: &str) -> Self {
        Self {
            files: HashMap::new(),
            last_build: None,
            version: None,
            cache_path: project_dir.join(filename),
        }
    }

    /// Load cache from project directory
    pub fn load(project_dir: &Path) -> Self {
        Self::load_from(project_dir, DEFAULT_CACHE_FILE)
    }

    /// Load cache with custom filename
    pub fn load_from(project_dir: &Path, filename: &str) -> Self {
        let cache_path = project_dir.join(filename);

        if cache_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&cache_path) {
                if let Ok(mut cache) = serde_json::from_str::<FileCache>(&content) {
                    cache.cache_path = cache_path;
                    return cache;
                }
            }
        }

        Self::with_filename(project_dir, filename)
    }

    /// Save cache to disk
    pub fn save(&self) -> Result<(), CacheError> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| CacheError::SerializeError { source: e })?;

        std::fs::write(&self.cache_path, content).map_err(|e| CacheError::WriteError {
            path: self.cache_path.clone(),
            source: e,
        })?;

        Ok(())
    }

    /// Check if a file has been modified since last cache update
    pub fn is_modified(&self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        let cached_time = self.files.get(path).copied();
        let current_time = Self::get_mtime(path);

        match (cached_time, current_time) {
            (Some(cached), Some(current)) => current > cached,
            (None, Some(_)) => true, // New file
            _ => true,               // File missing or error, assume modified
        }
    }

    /// Check if any files in a list have been modified
    pub fn any_modified(&self, paths: impl IntoIterator<Item = impl AsRef<Path>>) -> bool {
        paths.into_iter().any(|p| self.is_modified(p))
    }

    /// Update cache entry for a file
    pub fn update(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        if let Some(mtime) = Self::get_mtime(path) {
            self.files.insert(path.to_path_buf(), mtime);
        }
    }

    /// Update cache entries for multiple files
    pub fn update_all(&mut self, paths: impl IntoIterator<Item = impl AsRef<Path>>) {
        for path in paths {
            self.update(path);
        }
    }

    /// Mark build as complete
    pub fn mark_built(&mut self) {
        self.last_build = Some(Self::current_timestamp());
    }

    /// Get last build timestamp
    pub fn last_build_time(&self) -> Option<u64> {
        self.last_build
    }

    /// Set cache version (for invalidation)
    pub fn set_version(&mut self, version: impl Into<String>) {
        self.version = Some(version.into());
    }

    /// Check if version matches
    pub fn version_matches(&self, version: &str) -> bool {
        self.version.as_deref() == Some(version)
    }

    /// Clear the entire cache
    pub fn clear(&mut self) {
        self.files.clear();
        self.last_build = None;
    }

    /// Remove a specific file from cache
    pub fn remove(&mut self, path: impl AsRef<Path>) {
        self.files.remove(path.as_ref());
    }

    /// Get number of tracked files
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Get all tracked file paths
    pub fn tracked_files(&self) -> impl Iterator<Item = &PathBuf> {
        self.files.keys()
    }

    /// Get file modification time as unix timestamp
    fn get_mtime(path: &Path) -> Option<u64> {
        std::fs::metadata(path)
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
    }

    /// Get current timestamp
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_cache_new_file_is_modified() {
        let cache = FileCache::new(Path::new("."));
        assert!(cache.is_modified("nonexistent_file.txt"));
    }
}
