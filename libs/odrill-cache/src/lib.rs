//! # odrill-cache
//!
//! A generic file caching library for incremental builds and change detection.
//!
//! ## Features
//! - Track file modification times
//! - Incremental build support
//! - JSON-based persistent cache
//! - Hash-based content change detection (TODO)
//!
//! ## Example
//! ```rust,ignore
//! use odrill_cache::FileCache;
//!
//! let mut cache = FileCache::load("./my-project")?;
//!
//! if cache.is_modified("src/main.lua") {
//!     // Rebuild...
//!     cache.update("src/main.lua");
//! }
//!
//! cache.save()?;
//! ```

mod error;
mod file_cache;
mod hash_cache;

pub use error::CacheError;
pub use file_cache::FileCache;
pub use hash_cache::HashCache;
