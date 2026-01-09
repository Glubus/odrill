//! odrill doc - Generate documentation from Lua source
//!
//! Extracts LDoc-style comments and generates HTML or Markdown documentation.

use crate::config::OdrillConfig;
use anyhow::{Context, Result};
use colored::Colorize;
use compiler::{DocItem, extract_docs, generate_html, generate_markdown};
use std::fs;
use std::path::Path;

/// Run the doc command
pub fn run(format: &str, output: &str) -> Result<()> {
    println!("📚 Generating documentation...\n");

    let config = OdrillConfig::load()?;
    let project_name = &config.package.name;

    // Collect all doc items from src/
    let all_items = collect_all_docs()?;

    if all_items.is_empty() {
        println!("{}", "No documented items found.".yellow());
        println!("Tip: Add doc comments using --- before functions:");
        println!();
        println!("  --- Calculate the sum");
        println!("  --- @param a number First number");
        println!("  --- @param b number Second number");
        println!("  --- @return number The sum");
        println!("  function add(a, b)");
        println!("      return a + b");
        println!("  end");
        return Ok(());
    }

    // Create output directory
    let output_dir = Path::new(output);
    fs::create_dir_all(output_dir)?;

    match format {
        "html" => {
            let html = generate_html(&all_items, project_name);
            let output_path = output_dir.join("index.html");
            fs::write(&output_path, html)?;
            println!("  {} Generated {}", "✓".green(), output_path.display());
        }
        "markdown" | "md" => {
            let md = generate_markdown(&all_items, project_name);
            let output_path = output_dir.join("README.md");
            fs::write(&output_path, md)?;
            println!("  {} Generated {}", "✓".green(), output_path.display());
        }
        "both" => {
            let html = generate_html(&all_items, project_name);
            let html_path = output_dir.join("index.html");
            fs::write(&html_path, html)?;
            println!("  {} Generated {}", "✓".green(), html_path.display());

            let md = generate_markdown(&all_items, project_name);
            let md_path = output_dir.join("README.md");
            fs::write(&md_path, md)?;
            println!("  {} Generated {}", "✓".green(), md_path.display());
        }
        _ => {
            anyhow::bail!(
                "Unknown format: {}. Use 'html', 'markdown', or 'both'",
                format
            );
        }
    }

    println!();
    println!("📖 Documented {} items", all_items.len());

    Ok(())
}

/// Collect documentation from all Lua files in src/
fn collect_all_docs() -> Result<Vec<DocItem>> {
    let src_dir = Path::new("src");
    if !src_dir.exists() {
        return Ok(Vec::new());
    }

    let mut all_items = Vec::new();

    for entry in walkdir::WalkDir::new(src_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "lua"))
    {
        let path = entry.path();
        let source =
            fs::read_to_string(path).context(format!("Failed to read {}", path.display()))?;

        let items = extract_docs(path, &source);
        all_items.extend(items);
    }

    // Sort by name for consistent output
    all_items.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(all_items)
}
