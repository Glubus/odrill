use super::Compiler;
use super::compiler_result::CompilerResult;
use crate::engine::processor;
use pkg::manifest::HookConfig;
use std::collections::HashSet;

impl Compiler {
    pub fn compile_all(&mut self) -> anyhow::Result<Vec<CompilerResult>> {
        let mut results = Vec::new();

        for hook in &self.project.manifest.hooks.clone() {
            let result = self.compile_hook(hook)?;
            results.push(result);
        }

        self.symbols.warn_conflicts();

        Ok(results)
    }

    pub fn compile_hook(&mut self, hook: &HookConfig) -> anyhow::Result<CompilerResult> {
        let entry_path = self.project.root.join(&hook.entry);
        let output_path = self.project.root.join("dist").join(&hook.output);
        let src_root = self.project.root.join("src");

        let mut visited = HashSet::new();
        let mut source_files = Vec::new();

        processor::collect_dependencies(
            &entry_path,
            &src_root,
            &self.parser,
            &mut visited,
            &mut source_files,
        )?;

        let bundled = self.generate_bundle(&entry_path, &src_root)?;
        // apply options? project.manifest.options...
        // let bundled = self.apply_options(bundled); // Need to check if apply_options exists
        // generate_bundle is in `generate.rs`. I need to update `generate.rs` too.

        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&output_path, &bundled)?;

        Ok(CompilerResult {
            output_path,
            source_files,
            lines_total: bundled.lines().count(),
            was_cached: false,
        })
    }
}
