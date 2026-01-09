//! Documentation generator for Lua code
//!
//! Extracts documentation comments and generates HTML/Markdown output.

pub mod extractor;
pub mod html;
pub mod markdown;

pub use extractor::{DocComment, DocItem, extract_docs};
pub use html::generate_html;
pub use markdown::generate_markdown;
