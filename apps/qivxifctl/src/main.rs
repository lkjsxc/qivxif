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
    MalformedWire {
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
    let label = match command {
        ProbeCommand::Smoke { addr } => {
            qivxif_probe::smoke(&addr).await?;
            "smoke"
        }
        ProbeCommand::PersistPlace { addr } => {
            qivxif_probe::persist_place(&addr).await?;
            "persist-place"
        }
        ProbeCommand::RequestReplay { addr } => {
            qivxif_probe::request_replay(&addr).await?;
            "request-replay"
        }
        ProbeCommand::ProtocolGuards { addr } => {
            qivxif_probe::protocol_guards(&addr).await?;
            "protocol-guards"
        }
        ProbeCommand::MalformedWire { addr } => {
            qivxif_probe::malformed_wire(&addr).await?;
            "malformed-wire"
        }
        ProbeCommand::PersistCheck { addr } => {
            qivxif_probe::persist_check(&addr).await?;
            "persist-check"
        }
    };
    println!("probe {label} ... ok");
    Ok(())
}
