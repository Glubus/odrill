use crate::config_global::GlobalConfig;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct ConfigArgs {
    #[clap(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Set a configuration value
    Set { key: String, value: String },
    /// Get a configuration value
    Get { key: String },
}

pub fn run(args: ConfigArgs) -> Result<()> {
    match args.command {
        ConfigCommand::Set { key, value } => {
            let mut config = GlobalConfig::load()?;
            config.set(&key, &value)?;
        }
        ConfigCommand::Get { key } => {
            let config = GlobalConfig::load()?;
            if let Some(val) = config.get(&key) {
                println!("{}", val);
            } else {
                println!("(not set)");
            }
        }
    }
    Ok(())
}
