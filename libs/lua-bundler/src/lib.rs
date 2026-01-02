//! # lua-bundler
//!
//! A Lua module bundler that resolves requires/includes and produces a single file.
//!
//! ## Features
//! - Bundle multiple Lua files into one
//! - Resolve `require()` and custom `include()` statements
//! - Optional minification
//! - Caching for incremental builds (via odrill-cache)
//! - SuperBLT mod output generation

pub mod bundler;
pub mod config;
pub mod error;
pub mod parser;
pub mod superblt;

// Re-export cache from odrill-cache
pub use odrill_cache::FileCache;

pub use bundler::Bundler;
pub use config::BundleConfig;
pub use error::BundlerError;
pub use superblt::generate_superblt_files;
