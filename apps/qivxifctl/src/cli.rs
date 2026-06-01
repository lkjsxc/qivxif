use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "qivxifctl", about = "qivxif administration and quality CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
pub enum Command {
    Admin(AdminCommand),
    Docs {
        #[command(subcommand)]
        command: DocsCommand,
    },
    Quality {
        #[command(subcommand)]
        command: QualityCommand,
    },
    Store {
        #[command(subcommand)]
        command: StoreCommand,
    },
    Feeds {
        #[command(subcommand)]
        command: FeedsCommand,
    },
}

#[derive(Args)]
pub struct AdminCommand {
    #[command(subcommand)]
    pub command: AdminSubcommand,
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
pub enum AdminSubcommand {
    Bootstrap(AdminBootstrap),
}

#[derive(Args)]
pub struct AdminBootstrap {
    #[arg(long)]
    pub store: PathBuf,
    #[arg(long)]
    pub name: String,
    #[arg(long)]
    pub password_stdin: bool,
    #[arg(long)]
    pub json: bool,
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
pub enum DocsCommand {
    ValidateTopology,
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
pub enum QualityCommand {
    #[command(name = "check-lines")]
    Lines,
    #[command(name = "check-wording")]
    Wording,
    #[command(name = "check-retired-canon")]
    RetiredCanon,
    #[command(name = "check-placeholders")]
    ImplementationMarkers,
    #[command(name = "check-workspace")]
    Workspace,
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
pub enum StoreCommand {
    Stats(StorePath),
    Health(StorePath),
    RepairCheck(StorePath),
}

#[derive(Args)]
pub struct StorePath {
    #[arg(long)]
    pub store: PathBuf,
    #[arg(long)]
    pub json: bool,
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
pub enum FeedsCommand {
    Rebuild {
        #[arg(long)]
        store: PathBuf,
        #[arg(long)]
        json: bool,
    },
}
