use crate::engine::{Compiler, processor};
use crate::error::BundlerError;
use std::collections::HashSet;
use std::path::Path;

impl Compiler {
    /// Generate the bundled output for a hook
    pub fn generate_bundle(&mut self, entry: &Path, src_root: &Path) -> anyhow::Result<String> {
        let mut output = String::new();
        let mut processed = HashSet::new();
        let mut local_symbols = crate::engine::symbols::SymbolTable::new();

        output.push_str(&format!(
            "-- Bundled by odrill v{}\n",
            env!("CARGO_PKG_VERSION")
        ));
        output.push_str(&format!("-- Entry: {}\n", entry.display()));
        output.push_str("-- DO NOT EDIT - This file is auto-generated\n\n");

        self.bundle_file(
            entry,
            src_root,
            &mut output,
            &mut processed,
            &mut local_symbols,
        )?;

        Ok(output)
    }

    fn bundle_file(
        &mut self,
        file: &Path,
        src_root: &Path,
        output: &mut String,
        processed: &mut HashSet<std::path::PathBuf>,
        local_symbols: &mut crate::engine::symbols::SymbolTable,
    ) -> anyhow::Result<()> {
        let canonical = file.canonicalize().unwrap_or_else(|_| file.to_path_buf());

        if processed.contains(&canonical) {
            return Ok(());
        }
        processed.insert(canonical.clone());

        let content = std::fs::read_to_string(file).map_err(|e| BundlerError::FileRead {
            path: file.to_path_buf(),
            source: e,
        })?;

        // Process use directives first
        let uses = self.parser.extract_uses(&content);
        let mut visited_modules = HashSet::new();
        processor::process_use_directives(
            &uses,
            src_root,
            &self.parser,
            local_symbols,
            output,
            &mut visited_modules,
        )?;

        // Process traditional includes
        let includes = self.parser.extract_includes(&content);
        for inc in &includes {
            if let Some(resolved) =
                self.parser
                    .resolve_module_path(&inc.module_path, file, src_root)
            {
                self.bundle_file(&resolved, src_root, output, processed, local_symbols)?;
            }
        }

        // Add file content (filter out include/use lines)
        let relative = file.strip_prefix(&self.project.root).unwrap_or(file);
        output.push_str(&format!("\n-- [{}]\n", relative.display()));

        for line in content.lines() {
            let is_directive = includes.iter().any(|i| line.contains(&i.full_match))
                || uses.iter().any(|u| line.contains(&u.full_match));
            if !is_directive {
                output.push_str(line);
                output.push('\n');
            }
        }

        Ok(())
    }

    pub fn apply_options(&self, code: String) -> String {
        // Use manifest.options (assuming OptionsConfig has strict types field)
        // For now assume default, or check manifest
        // let options = &self.project.manifest.options;
        // if options.strip_comments ...

        // TODO: Implement options in OdrillManifest properly

        code
    }
}
