use anyhow::Result;
use clap::Args;
use colored::Colorize;
use std::fs;
use std::path::Path;
use stylua_lib::{Config, OutputVerification, format_code};
use walkdir::WalkDir;

#[derive(Args)]
pub struct FmtArgs {
    #[clap(long, short)]
    pub check: bool,
}

pub fn run(args: FmtArgs) -> Result<()> {
    println!("🎨 Formatting Lua files...");
    let root = std::env::current_dir()?;
    let mut count = 0;

    for entry in WalkDir::new(&root) {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "lua") {
            if path.to_string_lossy().contains("target") || path.to_string_lossy().contains(".git")
            {
                continue;
            }

            match process_file(path, args.check) {
                Ok(formatted) => {
                    if formatted {
                        count += 1;
                    }
                }
                Err(e) => eprintln!("Failed to format {}: {}", path.display(), e),
            }
        }
    }

    if args.check {
        println!("✨ Checked {} files.", count);
    } else {
        println!("✨ Formatted {} files.", count);
    }
    Ok(())
}

fn process_file(path: &Path, check: bool) -> Result<bool> {
    let content = fs::read_to_string(path)?;

    // Default config
    let config = Config::default();

    // Attempt format
    let formatted = format_code(&content, config, None, OutputVerification::None)?;

    if content != formatted {
        if !check {
            fs::write(path, &formatted)?;
            println!("  {} {}", "fixed".green(), path.display());
        } else {
            println!("  {} {}", "diff".red(), path.display());
        }
        Ok(true)
    } else {
        Ok(false)
    }
}
