//! Odrill Linter - Lua static analysis library
//!
//! A standalone Lua linter providing:
//! - Unused variable detection
//! - Undefined variable detection  
//! - Dead code detection
//! - Configurable rules and globals

pub mod diagnostics;
pub mod globals;
pub mod rules;

mod analyzer;

pub use analyzer::LuaAnalyzer;
pub use diagnostics::{Diagnostic, DiagnosticCode, DiagnosticLevel};
pub use globals::KnownGlobals;
