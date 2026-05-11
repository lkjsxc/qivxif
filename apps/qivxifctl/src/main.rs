use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Docs {
        #[command(subcommand)]
        command: DocsCommand,
    },
    Quality {
        #[command(subcommand)]
        command: QualityCommand,
    },
    Probe {
        #[command(subcommand)]
        command: ProbeCommand,
    },
}

#[derive(Subcommand)]
enum DocsCommand {
    ValidateTopology,
}

#[derive(Subcommand)]
enum QualityCommand {
    CheckLines,
}

#[derive(Subcommand)]
enum ProbeCommand {
    Smoke {
        #[arg(long)]
        addr: String,
    },
    PersistPlace {
        #[arg(long)]
        addr: String,
    },
    RequestReplay {
        #[arg(long)]
        addr: String,
    },
    ProtocolGuards {
        #[arg(long)]
        addr: String,
    },
    PersistCheck {
        #[arg(long)]
        addr: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Docs {
            command: DocsCommand::ValidateTopology,
        } => qivxif_quality::validate_docs_topology()?,
        Command::Quality {
            command: QualityCommand::CheckLines,
        } => qivxif_quality::check_lines()?,
        Command::Probe { command } => run_probe(command).await?,
    }
    Ok(())
}

async fn run_probe(command: ProbeCommand) -> Result<()> {
    match command {
        ProbeCommand::Smoke { addr } => qivxif_probe::smoke(&addr).await,
        ProbeCommand::PersistPlace { addr } => qivxif_probe::persist_place(&addr).await,
        ProbeCommand::RequestReplay { addr } => qivxif_probe::request_replay(&addr).await,
        ProbeCommand::ProtocolGuards { addr } => qivxif_probe::protocol_guards(&addr).await,
        ProbeCommand::PersistCheck { addr } => qivxif_probe::persist_check(&addr).await,
    }
}
