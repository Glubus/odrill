use super::Compiler;
use crate::engine::processor;
use std::collections::HashSet;

impl Compiler {
    /// Verify all hooks without generating output
    pub fn verify_all(&self) -> anyhow::Result<()> {
        for hook in &self.project.manifest.hooks {
            self.verify_hook(hook)?;
        }
        Ok(())
    }

    pub fn verify_hook(&self, hook: &pkg::manifest::HookConfig) -> anyhow::Result<()> {
        let entry_path = self.project.root.join(&hook.entry);
        let src_root = self.project.root.join("src");

        let mut visited = HashSet::new();
        let mut source_files = Vec::new();

        // Just run collection to check for errors
        processor::collect_dependencies(
            &entry_path,
            &src_root,
            &self.parser,
            &mut visited,
            &mut source_files,
        )?;

        Ok(())
    }
}
