//! odrill - Lua bundler CLI for PAYDAY 2 mods

mod commands;
mod config;

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "odrill")]
#[command(author = "Glubus")]
#[command(version)]
#[command(about = "Lua module bundler", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project
    Init {
        /// Project name (or "." for current dir)
        #[arg(default_value = ".")]
        name: String,

        /// Template to use: "odrill" (default) or "superblt"
        #[arg(short, long)]
        template: Option<String>,

        /// Skip git initialization
        #[arg(long)]
        no_git: bool,
    },

    /// Build the project (bundle all hooks)
    Build {
        /// Force rebuild, ignore cache
        #[arg(short, long)]
        force: bool,

        /// Watch for changes and rebuild
        #[arg(short, long)]
        watch: bool,
    },

    /// Clean build artifacts
    Clean,

    /// Add a new hook to the project
    Add {
        /// Hook ID (e.g., "lib/managers/hudmanagerpd2")
        hook_id: String,

        /// Output filename
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Format Lua code
    Fmt {
        /// Check only, do not write
        #[arg(long)]
        check: bool,
    },

    /// Publish package to registry
    Publish(commands::publish::PublishArgs),
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init {
            name,
            template,
            no_git,
        } => commands::init::run(&name, !no_git, template.as_deref()),
        Commands::Build { force, watch } => commands::build::run(force, watch),
        Commands::Clean => commands::clean::run(),
        Commands::Add { hook_id, output } => commands::add::run(&hook_id, output.as_deref()),
        Commands::Fmt { check } => commands::fmt::run(commands::fmt::FmtArgs { check }),
        Commands::Publish(args) => commands::publish::run(args),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}
