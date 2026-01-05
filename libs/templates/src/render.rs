use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct RenderContext {
    pub name: String,
    pub author: String,
    pub version: String,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self {
            name: "test_project".to_string(),
            author: "odrill_tester".to_string(),
            version: "0.1.0".to_string(),
        }
    }
}

pub fn render_dir(src: &Path, dst: &Path, ctx: &RenderContext) -> Result<()> {
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let path = entry.path();

        // Skip hidden/system directories usually
        if path.to_string_lossy().contains(".git") || path.to_string_lossy().contains("target") {
            continue;
        }

        let relative = path.strip_prefix(src)?;
        let target_path = dst.join(relative);

        if path.is_dir() {
            fs::create_dir_all(&target_path)?;
            continue;
        }

        // Try reading as text
        match fs::read_to_string(path) {
            Ok(text) => {
                let rendered = text
                    .replace("{{ ctx.name }}", &ctx.name)
                    .replace("{{ctx.name}}", &ctx.name)
                    .replace("{{ ctx.author }}", &ctx.author)
                    .replace("{{ctx.author}}", &ctx.author)
                    .replace("{{ ctx.version }}", &ctx.version)
                    .replace("{{ctx.version}}", &ctx.version);

                fs::write(&target_path, rendered)?;
            }
            Err(_) => {
                // Binary, just copy
                fs::copy(path, &target_path)?;
            }
        }
    }
    Ok(())
}
