use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Serve {
        #[arg(long, default_value = "config/server.toml")]
        config: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    match Cli::parse().command {
        Command::Serve { config } => {
            let cfg = qivxif_core::ServerConfig::load(&config)?;
            tracing::info!(bind_addr = %cfg.bind_addr, "server config loaded");
            tokio::signal::ctrl_c().await?;
        }
    }
    Ok(())
}
