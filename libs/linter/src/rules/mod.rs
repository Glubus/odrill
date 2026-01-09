//! Lint rules module
//!
//! Individual lint rule implementations.

pub mod dead_code;
pub mod undefined_vars;
pub mod unused_vars;

pub use dead_code::check_dead_code;
pub use undefined_vars::UndefinedVariableVisitor;
pub use unused_vars::UnusedVariableVisitor;
