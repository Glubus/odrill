//! Global symbol table for tracking included functions

use crate::parser::FunctionDef;
use std::collections::HashMap;
use std::path::PathBuf;

/// Tracks symbols that have been included globally to avoid duplication
#[derive(Debug, Default)]
pub struct SymbolTable {
    /// symbol_name -> (source_file, FunctionDef)
    symbols: HashMap<String, (PathBuf, FunctionDef)>,
    /// Conflicts detected: symbol_name -> list of files defining it
    conflicts: HashMap<String, Vec<PathBuf>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a symbol. Returns true if new, false if already exists.
    pub fn register(&mut self, name: &str, file: &PathBuf, def: FunctionDef) -> bool {
        if let Some((existing_file, _)) = self.symbols.get(name) {
            // Conflict detected
            self.conflicts
                .entry(name.to_string())
                .or_insert_with(|| vec![existing_file.clone()])
                .push(file.clone());
            false
        } else {
            self.symbols.insert(name.to_string(), (file.clone(), def));
            true
        }
    }

    /// Check if a symbol is already included
    pub fn contains(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    /// Get a symbol's definition if exists
    pub fn get(&self, name: &str) -> Option<&FunctionDef> {
        self.symbols.get(name).map(|(_, def)| def)
    }

    /// Get all detected conflicts
    pub fn get_conflicts(&self) -> &HashMap<String, Vec<PathBuf>> {
        &self.conflicts
    }

    /// Log warnings for all conflicts
    pub fn warn_conflicts(&self) {
        for (symbol, files) in &self.conflicts {
            eprintln!(
                "⚠️  Symbol '{}' defined in multiple files: {:?}",
                symbol, files
            );
        }
    }
}
