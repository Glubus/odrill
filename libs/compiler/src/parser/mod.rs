//! Lua parser for extracting include/require/use statements

mod functions;
mod includes;
mod uses;

pub use functions::{FunctionDef, build_symbol_table, extract_functions};
pub use includes::{IncludeType, ModuleInclude, extract_includes};
pub use uses::{UseDirective, extract_uses};

use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;

/// Parser for extracting includes/uses from Lua source code
pub struct LuaParser {
    include_pattern: Regex,
    require_pattern: Regex,
    dofile_pattern: Regex,
    comment_pattern: Regex,
    use_single_pattern: Regex,
    use_multi_pattern: Regex,
    use_all_pattern: Regex,
    function_pattern: Regex,
}

impl LuaParser {
    pub fn new(include_directive: &str) -> Self {
        Self {
            include_pattern: Regex::new(&format!(
                r#"{}\s*\(\s*["']([^"']+)["']\s*\)"#,
                regex::escape(include_directive)
            ))
            .unwrap(),
            require_pattern: Regex::new(r#"require\s*\(\s*["']([^"']+)["']\s*\)"#).unwrap(),
            dofile_pattern: Regex::new(r#"dofile\s*\([^"']*["']([^"']+\.lua)["']\s*\)"#).unwrap(),
            comment_pattern: Regex::new(r#"--\[\[[\s\S]*?\]\]|--[^\n]*"#).unwrap(),
            use_single_pattern: Regex::new(r#"^use\s+(.+)::(\w+)\s*$"#).unwrap(),
            use_multi_pattern: Regex::new(r#"^use\s+(.+)::\{([^}]+)\}\s*$"#).unwrap(),
            use_all_pattern: Regex::new(r#"^use\s+(.+)::\*\s*$"#).unwrap(),
            function_pattern: Regex::new(r#"^\s*(local\s+)?function\s+([\w\.:]+)\s*\("#).unwrap(),
        }
    }

    pub fn extract_includes(&self, source: &str) -> Vec<ModuleInclude> {
        includes::extract_includes(
            source,
            &self.include_pattern,
            &self.require_pattern,
            &self.dofile_pattern,
            &self.comment_pattern,
        )
    }

    pub fn extract_uses(&self, source: &str) -> Vec<UseDirective> {
        uses::extract_uses(
            source,
            &self.use_single_pattern,
            &self.use_multi_pattern,
            &self.use_all_pattern,
        )
    }

    pub fn extract_functions(&self, source: &str) -> Vec<FunctionDef> {
        functions::extract_functions(source, &self.function_pattern)
    }

    pub fn build_symbol_table(&self, source: &str) -> HashMap<String, FunctionDef> {
        functions::build_symbol_table(source, &self.function_pattern)
    }

    pub fn strip_comments(&self, source: &str) -> String {
        self.comment_pattern.replace_all(source, "").to_string()
    }

    pub fn resolve_module_path(
        &self,
        module_path: &str,
        current_file: &std::path::Path,
        src_root: &std::path::Path,
    ) -> Option<PathBuf> {
        let current_dir = current_file.parent()?;
        let resolved = if module_path.starts_with("./") || module_path.starts_with("../") {
            current_dir.join(module_path)
        } else {
            src_root.join(module_path)
        };

        try_resolve_path(&resolved, module_path)
    }
}

fn try_resolve_path(resolved: &PathBuf, module_path: &str) -> Option<PathBuf> {
    if resolved.exists() {
        return Some(resolved.clone());
    }
    if !module_path.ends_with(".lua") {
        let with_ext = resolved.with_extension("lua");
        if with_ext.exists() {
            return Some(with_ext);
        }
        let as_mod = resolved.join("mod.lua");
        if as_mod.exists() {
            return Some(as_mod);
        }
    }
    None
}
