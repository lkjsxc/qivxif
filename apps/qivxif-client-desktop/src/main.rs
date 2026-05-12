mod args;
mod native;
mod native_controls;
mod native_evidence;
mod native_window;
mod smoke;

use anyhow::Result;
use args::{Cli, Command};
use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt};

fn main() -> Result<()> {
    init_tracing();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    match Cli::parse().command {
        Command::Run(args) => native::run_interactive(args, runtime)?,
        Command::E2e(args) => native::run_e2e(args, runtime)?,
        Command::SmokeFrame(args) => runtime.block_on(smoke::run(args))?,
    }
    Ok(())
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter).with_target(false).init();
}
