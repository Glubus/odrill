//! Bundle generation logic

use crate::bundler::{Bundler, processor};
use crate::error::BundlerError;
use std::collections::HashSet;
use std::path::Path;

impl Bundler {
    /// Generate the bundled output for a hook
    pub fn generate_bundle(&mut self, entry: &Path, src_root: &Path) -> anyhow::Result<String> {
        let mut output = String::new();
        let mut processed = HashSet::new();

        output.push_str(&format!(
            "-- Bundled by odrill v{}\n",
            env!("CARGO_PKG_VERSION")
        ));
        output.push_str(&format!("-- Entry: {}\n", entry.display()));
        output.push_str("-- DO NOT EDIT - This file is auto-generated\n\n");

        self.bundle_file(entry, src_root, &mut output, &mut processed)?;

        Ok(output)
    }

    fn bundle_file(
        &mut self,
        file: &Path,
        src_root: &Path,
        output: &mut String,
        processed: &mut HashSet<std::path::PathBuf>,
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
        processor::process_use_directives(
            &uses,
            src_root,
            &self.parser,
            &mut self.symbols,
            output,
        )?;

        // Process traditional includes
        let includes = self.parser.extract_includes(&content);
        for inc in &includes {
            if let Some(resolved) =
                self.parser
                    .resolve_module_path(&inc.module_path, file, src_root)
            {
                self.bundle_file(&resolved, src_root, output, processed)?;
            }
        }

        // Add file content (filter out include/use lines)
        let relative = file.strip_prefix(&self.project_dir).unwrap_or(file);
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

    pub fn apply_options(&self, mut code: String) -> String {
        if self.config.options.strip_comments {
            code = self.parser.strip_comments(&code);
        }
        if self.config.options.minify {
            code = code
                .lines()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>()
                .join("\n");
        }
        code
    }
}
