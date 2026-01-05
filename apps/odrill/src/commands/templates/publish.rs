use anyhow::Result;
use clap::Args;
use templates::publish;

#[derive(Args)]
pub struct PublishArgs {
    #[clap(long, short)]
    pub registry: Option<String>,
}

pub fn run(args: PublishArgs) -> Result<()> {
    let root = std::env::current_dir()?;
    let token = crate::auth::load_token()?;

    let registry_url = args
        .registry
        .unwrap_or_else(|| "http://localhost:5150/api".to_string());

    publish(&root, &registry_url, &token)?;

    Ok(())
}
