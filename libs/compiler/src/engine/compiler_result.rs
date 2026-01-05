use std::path::PathBuf;

/// Result of compiling a single hook/entry
#[derive(Debug)]
pub struct CompilerResult {
    pub output_path: PathBuf,
    pub source_files: Vec<PathBuf>,
    pub lines_total: usize,
    pub was_cached: bool,
}
