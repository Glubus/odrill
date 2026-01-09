//! Core analyzer that orchestrates lint rules

use crate::diagnostics::{Diagnostic, DiagnosticCode};
use crate::globals::KnownGlobals;
use crate::rules::{UndefinedVariableVisitor, UnusedVariableVisitor, check_dead_code};
use full_moon::ast::Ast;
use full_moon::visitors::Visitor;
use std::path::Path;

/// Configuration for the Lua analyzer
#[derive(Debug, Clone)]
pub struct AnalyzerConfig {
    /// Check for unused variables
    pub check_unused: bool,
    /// Check for undefined variables
    pub check_undefined: bool,
    /// Check for dead code
    pub check_dead_code: bool,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            check_unused: true,
            check_undefined: true,
            check_dead_code: true,
        }
    }
}

/// Lua analyzer for checking code quality
pub struct LuaAnalyzer {
    globals: KnownGlobals,
    config: AnalyzerConfig,
}

impl LuaAnalyzer {
    /// Create with default configuration
    pub fn new() -> Self {
        Self {
            globals: KnownGlobals::with_defaults(),
            config: AnalyzerConfig::default(),
        }
    }

    /// Create with custom globals
    pub fn with_globals(globals: KnownGlobals) -> Self {
        Self {
            globals,
            config: AnalyzerConfig::default(),
        }
    }

    /// Create with custom config
    pub fn with_config(config: AnalyzerConfig) -> Self {
        Self {
            globals: KnownGlobals::with_defaults(),
            config,
        }
    }

    /// Add custom globals
    pub fn add_globals(&mut self, names: impl IntoIterator<Item = String>) {
        self.globals.extend(names);
    }

    /// Get mutable reference to globals
    pub fn globals_mut(&mut self) -> &mut KnownGlobals {
        &mut self.globals
    }

    /// Analyze a Lua file and return diagnostics
    pub fn analyze_file(&self, path: &Path, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Filter odrill-specific syntax
        let filtered_source = filter_odrill_syntax(source);

        // Parse with full_moon
        let ast = match full_moon::parse(&filtered_source) {
            Ok(ast) => ast,
            Err(errors) => {
                for err in errors {
                    diagnostics.push(Diagnostic::error(
                        DiagnosticCode::ParseError,
                        format!("Parse error: {}", err),
                        path.to_path_buf(),
                        1,
                        1,
                    ));
                }
                return diagnostics;
            }
        };

        // Run enabled lint passes
        if self.config.check_unused {
            diagnostics.extend(self.check_unused_variables(&ast, path));
        }
        if self.config.check_undefined {
            diagnostics.extend(self.check_undefined_variables(&ast, path, source));
        }
        if self.config.check_dead_code {
            diagnostics.extend(check_dead_code(path, source));
        }

        diagnostics
    }

    fn check_unused_variables(&self, ast: &Ast, path: &Path) -> Vec<Diagnostic> {
        let mut visitor = UnusedVariableVisitor::new(path.to_path_buf());
        visitor.visit_ast(ast);
        visitor.finish()
    }

    fn check_undefined_variables(&self, ast: &Ast, path: &Path, source: &str) -> Vec<Diagnostic> {
        let imports = extract_imports(source);
        let mut visitor = UndefinedVariableVisitor::new(path.to_path_buf(), &self.globals, imports);
        visitor.visit_ast(ast);
        visitor.finish()
    }
}

impl Default for LuaAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Filter out odrill-specific syntax (use statements)
fn filter_odrill_syntax(source: &str) -> String {
    source
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("use ") && (trimmed.contains("::") || trimmed.ends_with("*")) {
                format!("-- odrill: {}", line)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Extract imported variable names from use statements
fn extract_imports(source: &str) -> Vec<String> {
    let mut imports = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("use ") {
            let content = trimmed[4..].trim();

            // Handle multiple imports: use foo::{a, b}
            if let Some(brace_start) = content.find('{') {
                if let Some(brace_end) = content.find('}') {
                    let vars = &content[brace_start + 1..brace_end];
                    for var in vars.split(',') {
                        let var = var.trim();
                        if !var.is_empty() {
                            imports.push(var.to_string());
                        }
                    }
                }
            }
            // Handle single import: use foo::bar
            else if let Some(last_colons) = content.rfind("::") {
                let var = content[last_colons + 2..].trim();
                // Ignore wildcard imports as we can't resolve them without analyzing the target file
                if var != "*" {
                    imports.push(var.to_string());
                }
            }
        }
    }
    imports
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_unused_variable() {
        let analyzer = LuaAnalyzer::new();
        let source = r#"
local unused = 42
local used = 10
print(used)
"#;
        let diagnostics = analyzer.analyze_file(&PathBuf::from("test.lua"), source);
        assert!(diagnostics.iter().any(|d| d.message.contains("unused")));
    }

    #[test]
    fn test_function_params_not_undefined() {
        let analyzer = LuaAnalyzer::new();
        let source = r#"
function test(a, b)
    return a + b
end
"#;
        let diagnostics = analyzer.analyze_file(&PathBuf::from("test.lua"), source);
        assert!(
            !diagnostics
                .iter()
                .any(|d| d.message.contains("Undefined variable: a"))
        );
        assert!(
            !diagnostics
                .iter()
                .any(|d| d.message.contains("Undefined variable: b"))
        );
    }

    #[test]
    fn test_odrill_use_ignored() {
        let analyzer = LuaAnalyzer::new();
        let source = r#"
use mylib::func
local x = 1
print(x)
"#;
        let diagnostics = analyzer.analyze_file(&PathBuf::from("test.lua"), source);
        assert!(
            !diagnostics
                .iter()
                .any(|d| matches!(d.code, DiagnosticCode::ParseError))
        );
    }
}
