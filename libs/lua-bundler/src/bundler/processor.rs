//! File processing utilities for bundler

use crate::bundler::symbols::SymbolTable;
use crate::error::BundlerError;
use crate::parser::{LuaParser, UseDirective};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Collect all dependencies recursively
pub fn collect_dependencies(
    file: &Path,
    src_root: &Path,
    parser: &LuaParser,
    visited: &mut HashSet<PathBuf>,
    files: &mut Vec<PathBuf>,
) -> anyhow::Result<()> {
    let canonical = file.canonicalize().unwrap_or_else(|_| file.to_path_buf());

    if visited.contains(&canonical) {
        return Ok(());
    }
    visited.insert(canonical.clone());
    files.push(canonical.clone());

    let content = std::fs::read_to_string(file).map_err(|e| BundlerError::FileRead {
        path: file.to_path_buf(),
        source: e,
    })?;

    for inc in parser.extract_includes(&content) {
        if let Some(resolved) = parser.resolve_module_path(&inc.module_path, file, src_root) {
            collect_dependencies(&resolved, src_root, parser, visited, files)?;
        }
    }

    Ok(())
}

/// Process use directives and extract only requested symbols
pub fn process_use_directives(
    uses: &[UseDirective],
    src_root: &Path,
    parser: &LuaParser,
    symbols: &mut SymbolTable,
    output: &mut String,
) -> anyhow::Result<()> {
    for use_dir in uses {
        let module_file = src_root.join(&use_dir.module_path).with_extension("lua");
        if !module_file.exists() {
            eprintln!("⚠️  Module not found: {}", use_dir.module_path);
            continue;
        }

        let content = std::fs::read_to_string(&module_file)?;
        let file_symbols = parser.build_symbol_table(&content);

        if use_dir.symbols.is_empty() {
            // Import all (::*)
            for (name, def) in file_symbols {
                include_symbol(&name, &def.content, &module_file, symbols, output);
            }
        } else {
            // Import specific symbols
            for sym_name in &use_dir.symbols {
                if let Some(def) = file_symbols.get(sym_name) {
                    include_symbol(sym_name, &def.content, &module_file, symbols, output);
                } else {
                    eprintln!(
                        "⚠️  Symbol '{}' not found in {}",
                        sym_name, use_dir.module_path
                    );
                }
            }
        }
    }
    Ok(())
}

fn include_symbol(
    name: &str,
    content: &str,
    file: &Path,
    symbols: &mut SymbolTable,
    output: &mut String,
) {
    if symbols.contains(name) {
        return; // Already included, skip
    }

    // Register (conflict detection happens inside)
    let def = crate::parser::FunctionDef {
        name: name.to_string(),
        start_line: 0,
        end_line: 0,
        content: content.to_string(),
        is_local: false,
    };
    symbols.register(name, &file.to_path_buf(), def);

    output.push_str(&format!("-- [use {}::{}]\n", file.display(), name));
    output.push_str(content);
    output.push('\n');
}
