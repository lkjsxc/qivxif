use anyhow::Result;
use clap::{Parser, Subcommand};

const CLI_COMMANDS: &str = r#"Commands:
  docs validate-topology
  quality check-lines
  quality check-wording"#;

#[derive(Parser)]
#[command(
    name = "qivxifctl",
    about = "Agent-friendly quality CLI",
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

fn main() -> Result<()> {
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
    }
    Ok(())
}
