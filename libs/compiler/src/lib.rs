//! Odrill Compiler Library
//!
//! Compiles Payday 2 mods from Odrill Projects.

pub mod docgen;
pub mod engine;
pub mod error;
pub mod parser;
pub mod superblt;

pub use docgen::{DocItem, extract_docs, generate_html, generate_markdown};
pub use engine::Compiler;
