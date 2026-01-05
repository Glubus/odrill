//! odrill - Lua bundler CLI for PAYDAY 2 mods

mod auth;
mod commands;
mod config;
pub mod config_global;
pub mod constants;
pub mod template_engine; // [NEW]

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
    /// Create a new project (from template)
    New(commands::new::NewArgs),

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

    /// Install dependencies from odrill.toml
    Install,

    /// Run project in isolated dev environment
    Run {
        /// Project path (optional)
        path: Option<String>,
    },

    /// Login to registry
    Login,

    /// Logout from registry
    Logout,

    /// Manage global configuration
    Config(commands::config::ConfigArgs),

    /// Manage templates
    Templates(commands::templates::TemplatesArgs),
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New(args) => commands::new::run(args),
        Commands::Build { force, watch } => commands::build::run(force, watch),
        Commands::Clean => commands::clean::run(),
        Commands::Add { hook_id, output } => commands::add::run(&hook_id, output.as_deref()),
        Commands::Fmt { check } => commands::fmt::run(commands::fmt::FmtArgs { check }),
        Commands::Publish(args) => commands::publish::run(args),
        Commands::Install => commands::install::run(),
        Commands::Run { path } => commands::run::run(path),
        Commands::Login => commands::login::run(),
        Commands::Logout => {
            let _ = auth::clear_token();
            println!("👋 Logged out");
            Ok(())
        }
        Commands::Config(args) => commands::config::run(args),
        Commands::Templates(args) => commands::templates::run(args),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}
