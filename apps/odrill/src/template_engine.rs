use crate::config_global::GlobalConfig;
use anyhow::Result;
use chrono::Utc;
use colored::Colorize;
use pkg::ModPackage;
use std::path::Path;

pub struct TemplateContext {
    pub name: String,
    pub author: String,
    pub version: String,
    pub date: String,
    pub pd2_path: String,
}

impl TemplateContext {
    pub fn new(name: &str) -> Result<Self> {
        let config = GlobalConfig::load()?;
        Ok(Self {
            name: name.to_string(),
            author: config.author.unwrap_or_else(|| "Unknown".to_string()),
            version: "0.1.0".to_string(),
            date: Utc::now().format("%Y-%m-%d").to_string(),
            pd2_path: config.pd2_path.unwrap_or_default(),
        })
    }
}

pub fn render(package: &ModPackage, target_dir: &Path, context: &TemplateContext) -> Result<()> {
    println!("🎨 Rendering template...");

    for (rel_path, content) in &package.files {
        // Determine if file is binary or text. Naive check: if it contains null bytes, binary.
        // Actually, for templates, we usually assume text unless extension is known binary (png, etc).
        // Let's try to convert to string. If fail, treat as binary.

        let path = target_dir.join(rel_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        match String::from_utf8(content.clone()) {
            Ok(text) => {
                // Perform replacement
                let render_text = text
                    .replace("{{ ctx.name }}", &context.name)
                    .replace("{{ctx.name}}", &context.name)
                    .replace("{{ ctx.author }}", &context.author)
                    .replace("{{ctx.author}}", &context.author)
                    .replace("{{ ctx.version }}", &context.version)
                    .replace("{{ctx.version}}", &context.version)
                    .replace("{{ ctx.date }}", &context.date)
                    .replace("{{ctx.date}}", &context.date)
                    .replace("{{ ctx.pd2_path }}", &context.pd2_path)
                    .replace("{{ctx.pd2_path}}", &context.pd2_path);

                std::fs::write(&path, render_text)?;
            }
            Err(_) => {
                // Binary file, just write
                std::fs::write(&path, content)?;
            }
        }
    }

    println!(
        "✨ Template rendered to {}",
        target_dir.display().to_string().cyan()
    );
    Ok(())
}
