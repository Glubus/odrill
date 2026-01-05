//! File processing utilities for bundler

use crate::engine::symbols::SymbolTable;
use crate::error::BundlerError;
use crate::parser::{LuaParser, UseDirective};
// parser/mod.rs exports ModuleInclude.
// processor.rs might not use the type name explicitly?
// Just use ModuleInclude.
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
    visited_modules: &mut HashSet<PathBuf>,
) -> anyhow::Result<()> {
    let project_root = src_root.parent().unwrap_or(src_root);

    for use_dir in uses {
        // Check for mods:: prefix (local module)
        let (is_local_mod, clean_path) =
            if let Some(stripped) = use_dir.module_path.strip_prefix("mods::") {
                (true, stripped)
            } else {
                (false, use_dir.module_path.as_str())
            };

        // Convert Rust-style path (hud::colors) to filesystem path (hud/colors)
        let fs_path = clean_path.replace("::", "/");

        let module_file = if is_local_mod {
            // Local module: only check src/ directory
            let local_file = src_root.join(&fs_path).with_extension("lua");
            let local_mod_file = src_root.join(&fs_path).join("mod.lua");

            if local_file.exists() {
                local_file
            } else if local_mod_file.exists() {
                local_mod_file
            } else {
                eprintln!("⚠️  Local module not found: mods::{}", clean_path);
                continue;
            }
        } else {
            // External package: check target/pkg/ first, then fallback to src/
            let first_segment = clean_path.split("::").next().unwrap_or(clean_path);
            let pkg_file = project_root
                .join("target")
                .join("pkg")
                .join(first_segment)
                .join("src")
                .join("init.lua");

            let local_file = src_root.join(&fs_path).with_extension("lua");
            let local_mod_file = src_root.join(&fs_path).join("mod.lua");

            if pkg_file.exists() {
                pkg_file
            } else if local_file.exists() {
                local_file
            } else if local_mod_file.exists() {
                local_mod_file
            } else {
                eprintln!("⚠️  Module not found: {}", use_dir.module_path);
                continue;
            }
        };

        // Read module content
        let content = std::fs::read_to_string(&module_file)?;
        let file_symbols = parser.build_symbol_table(&content);

        // Process module-level dependencies first (transitive imports)
        let module_uses = parser.extract_uses(&content);
        if !module_uses.is_empty() {
            // Check if we already processed this module's imports to avoid cycles
            let canonical_mod = module_file
                .canonicalize()
                .unwrap_or_else(|_| module_file.clone());
            if !visited_modules.contains(&canonical_mod) {
                visited_modules.insert(canonical_mod.clone());

                // Recurse for module dependencies
                process_use_directives(
                    &module_uses,
                    src_root,
                    parser,
                    symbols,
                    output,
                    visited_modules,
                )?;
            }
        }

        if use_dir.symbols.is_empty() {
            // Import all (::*)
            for (name, def) in file_symbols.iter() {
                include_symbol(
                    &name,
                    &def.content,
                    &module_file,
                    symbols,
                    output,
                    &file_symbols,
                );
            }
        } else {
            // Import specific symbols
            for sym_name in &use_dir.symbols {
                if let Some(def) = file_symbols.get(sym_name) {
                    include_symbol(
                        sym_name,
                        &def.content,
                        &module_file,
                        symbols,
                        output,
                        &file_symbols,
                    );
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
    all_file_symbols: &std::collections::HashMap<String, crate::parser::FunctionDef>,
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

    // Detect dependencies: find all uppercase identifiers used in this symbol
    // that might be local constants/tables from the same file
    let deps = extract_dependencies(content, all_file_symbols);

    // Recursively include dependencies first
    for dep_name in deps {
        if let Some(dep_def) = all_file_symbols.get(&dep_name) {
            include_symbol(
                &dep_name,
                &dep_def.content,
                file,
                symbols,
                output,
                all_file_symbols,
            );
        }
    }

    output.push_str(&format!("-- [use {}::{}]\n", file.display(), name));
    output.push_str(content);
    output.push('\n');
}

/// Extract dependencies from symbol content
/// Returns names of uppercase identifiers (likely constants/tables)
fn extract_dependencies(
    content: &str,
    all_symbols: &std::collections::HashMap<String, crate::parser::FunctionDef>,
) -> Vec<String> {
    use regex::Regex;

    // Match uppercase identifiers (local constants/tables)
    let id_pattern = Regex::new(r"\b([A-Z_][A-Z0-9_]*)\b").unwrap();
    let mut deps = std::collections::HashSet::new();

    for cap in id_pattern.captures_iter(content) {
        let ident = cap[1].to_string();
        // Only include if it's defined in the same module
        if all_symbols.contains_key(&ident) {
            deps.insert(ident);
        }
    }

    deps.into_iter().collect()
}
