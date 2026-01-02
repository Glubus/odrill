//! Error types for the Lua bundler

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BundlerError {
    #[error("Failed to read file: {path}")]
    FileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write file: {path}")]
    FileWrite {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Module not found: {module} (searched from {from})")]
    ModuleNotFound { module: String, from: PathBuf },

    #[error("Circular dependency detected: {chain}")]
    CircularDependency { chain: String },

    #[error("Invalid config: {message}")]
    InvalidConfig { message: String },

    #[error("Parse error in {file} at line {line}: {message}")]
    ParseError {
        file: PathBuf,
        line: usize,
        message: String,
    },
}
