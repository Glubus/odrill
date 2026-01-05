use clap::{Args, Subcommand};

pub mod publish;

#[derive(Args)]
pub struct TemplatesArgs {
    #[command(subcommand)]
    pub command: TemplatesCommands,
}

#[derive(Subcommand)]
pub enum TemplatesCommands {
    /// Publish a template to the registry
    Publish(publish::PublishArgs),
}

pub fn run(args: TemplatesArgs) -> anyhow::Result<()> {
    match args.command {
        TemplatesCommands::Publish(args) => publish::run(args),
    }
}
