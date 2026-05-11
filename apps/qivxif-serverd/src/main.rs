use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod app;
mod request;
mod session;

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
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
    match Cli::parse().command {
        Command::Serve { config } => {
            let cfg = qivxif_core::ServerConfig::load(&config)?;
            tracing::info!(
                config = %config.display(),
                bind_addr = %cfg.bind_addr,
                data_dir = %cfg.data_dir,
                build_epoch = %cfg.build_epoch,
                protocol_epoch = cfg.protocol_epoch,
                "server starting"
            );
            app::serve(cfg).await?;
        }
    }
    Ok(())
}
