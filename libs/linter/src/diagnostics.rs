//! Diagnostic types for the Lua linter

use std::path::PathBuf;

/// Severity level of a diagnostic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}

impl DiagnosticLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            DiagnosticLevel::Error => "error",
            DiagnosticLevel::Warning => "warning",
            DiagnosticLevel::Info => "info",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            DiagnosticLevel::Error => "❌",
            DiagnosticLevel::Warning => "⚠️",
            DiagnosticLevel::Info => "ℹ️",
        }
    }
}

/// Diagnostic codes for categorization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    /// Missing dependency
    MissingDependency,
    /// Unused local variable
    UnusedVariable,
    /// Undefined variable (not in known globals)
    UndefinedVariable,
    /// Dead code after return statement
    DeadCode,
    /// Parse error in Lua file
    ParseError,
    /// Empty function body
    EmptyFunction,
    /// Shadowed variable in same scope
    ShadowedVariable,
}

impl DiagnosticCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            DiagnosticCode::MissingDependency => "E001",
            DiagnosticCode::UnusedVariable => "W001",
            DiagnosticCode::UndefinedVariable => "W002",
            DiagnosticCode::DeadCode => "W003",
            DiagnosticCode::ParseError => "E002",
            DiagnosticCode::EmptyFunction => "W004",
            DiagnosticCode::ShadowedVariable => "W005",
        }
    }
}

/// A diagnostic message from the linter
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub code: DiagnosticCode,
    pub message: String,
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
}

impl Diagnostic {
    pub fn error(
        code: DiagnosticCode,
        message: impl Into<String>,
        file: PathBuf,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            code,
            message: message.into(),
            file,
            line,
            column,
        }
    }

    pub fn warning(
        code: DiagnosticCode,
        message: impl Into<String>,
        file: PathBuf,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            code,
            message: message.into(),
            file,
            line,
            column,
        }
    }

    pub fn info(
        code: DiagnosticCode,
        message: impl Into<String>,
        file: PathBuf,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            level: DiagnosticLevel::Info,
            code,
            message: message.into(),
            file,
            line,
            column,
        }
    }
}
