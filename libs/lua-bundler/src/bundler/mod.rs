//! Main bundler implementation

mod generate;
mod processor;
mod symbols;

pub use symbols::SymbolTable;

use crate::config::{BundleConfig, HookConfig};
use crate::parser::LuaParser;
use odrill_cache::FileCache;

use std::collections::HashSet;
use std::path::PathBuf;

/// Result of bundling a single hook
#[derive(Debug)]
pub struct BundleResult {
    pub output_path: PathBuf,
    pub source_files: Vec<PathBuf>,
    pub lines_total: usize,
    pub was_cached: bool,
}

/// The main Lua bundler
pub struct Bundler {
    config: BundleConfig,
    parser: LuaParser,
    cache: FileCache,
    project_dir: PathBuf,
    symbols: SymbolTable,
}

impl Bundler {
    pub fn new(project_dir: PathBuf, config: BundleConfig) -> Self {
        let parser = LuaParser::new(&config.options.include_directive);
        let cache = FileCache::load(&project_dir);

        Self {
            config,
            parser,
            cache,
            project_dir,
            symbols: SymbolTable::new(),
        }
    }

    pub fn from_project(project_dir: PathBuf) -> anyhow::Result<Self> {
        let config_path = project_dir.join("odrill.toml");
        let config = BundleConfig::from_file(&config_path)?;
        Ok(Self::new(project_dir, config))
    }

    pub fn bundle_all(&mut self) -> anyhow::Result<Vec<BundleResult>> {
        let mut results = Vec::new();

        for hook in &self.config.hooks.clone() {
            let result = self.bundle_hook(hook)?;
            results.push(result);
        }

        self.symbols.warn_conflicts();
        self.cache.mark_built();
        let _ = self.cache.save();

        Ok(results)
    }

    pub fn bundle_hook(&mut self, hook: &HookConfig) -> anyhow::Result<BundleResult> {
        let entry_path = self.project_dir.join(&hook.entry);
        let output_path = self.project_dir.join("dist").join(&hook.output);
        let src_root = self.project_dir.join("src");

        let mut visited = HashSet::new();
        let mut source_files = Vec::new();
        processor::collect_dependencies(
            &entry_path,
            &src_root,
            &self.parser,
            &mut visited,
            &mut source_files,
        )?;

        let needs_rebuild = !output_path.exists() || self.cache.any_modified(&source_files);
        if !needs_rebuild {
            return Ok(BundleResult {
                output_path,
                source_files,
                lines_total: 0,
                was_cached: true,
            });
        }

        let bundled = self.generate_bundle(&entry_path, &src_root)?;
        let bundled = self.apply_options(bundled);
        let lines_total = bundled.lines().count();

        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&output_path, &bundled)?;
        self.cache.update_all(&source_files);

        Ok(BundleResult {
            output_path,
            source_files,
            lines_total,
            was_cached: false,
        })
    }
}
