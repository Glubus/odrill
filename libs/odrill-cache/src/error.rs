//! Error types for odrill-cache

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Failed to read cache file: {path}")]
    ReadError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write cache file: {path}")]
    WriteError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse cache: {message}")]
    ParseError { message: String },

    #[error("Failed to serialize cache")]
    SerializeError {
        #[source]
        source: serde_json::Error,
    },
}
