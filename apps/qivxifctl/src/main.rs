use anyhow::Result;
use clap::{Parser, Subcommand};

const CLI_COMMANDS: &str = r#"Commands:
  docs validate-topology
  quality check-lines
  quality check-wording
  probe smoke --addr <ADDR>
  probe hello --addr <ADDR>
  probe join-world --addr <ADDR>
  probe chunk-request --addr <ADDR>
  probe place-block --addr <ADDR>
  probe flush-persistence --addr <ADDR>
  probe persist-place --addr <ADDR>
  probe request-replay --addr <ADDR>
  probe protocol-guards --addr <ADDR>
  probe malformed-wire --addr <ADDR>
  probe persist-check --addr <ADDR>"#;

#[derive(Parser)]
#[command(
    name = "qivxifctl",
    about = "Agent-friendly quality and probe CLI",
    subcommand_required = true,
    arg_required_else_help = true,
    after_help = CLI_COMMANDS,
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
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
#[command(rename_all = "kebab-case")]
enum DocsCommand {
    ValidateTopology,
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
enum QualityCommand {
    CheckLines,
    CheckWording,
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
enum ProbeCommand {
    Smoke {
        #[arg(long)]
        addr: String,
    },
    Hello {
        #[arg(long)]
        addr: String,
    },
    JoinWorld {
        #[arg(long)]
        addr: String,
    },
    ChunkRequest {
        #[arg(long)]
        addr: String,
    },
    PlaceBlock {
        #[arg(long)]
        addr: String,
    },
    FlushPersistence {
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
        Command::Quality {
            command: QualityCommand::CheckWording,
        } => qivxif_quality::check_wording()?,
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
        ProbeCommand::Hello { addr } => {
            qivxif_probe::hello(&addr).await?;
            "hello"
        }
        ProbeCommand::JoinWorld { addr } => {
            qivxif_probe::join_world(&addr).await?;
            "join-world"
        }
        ProbeCommand::ChunkRequest { addr } => {
            qivxif_probe::chunk_request(&addr).await?;
            "chunk-request"
        }
        ProbeCommand::PlaceBlock { addr } => {
            qivxif_probe::place_block(&addr).await?;
            "place-block"
        }
        ProbeCommand::FlushPersistence { addr } => {
            qivxif_probe::flush_persistence(&addr).await?;
            "flush-persistence"
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
