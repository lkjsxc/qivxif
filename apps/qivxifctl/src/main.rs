mod cli;
mod commands;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command, DocsCommand, FeedsCommand, QualityCommand, StoreCommand};

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Admin(command) => commands::admin(command)?,
        Command::Docs {
            command: DocsCommand::ValidateTopology,
        } => qivxif_quality::validate_docs_topology()?,
        Command::Quality { command } => quality(command)?,
        Command::Store { command } => store(command)?,
        Command::Feeds {
            command: FeedsCommand::Rebuild { store, json },
        } => commands::feeds_rebuild(store, json)?,
    }
    Ok(())
}

fn quality(command: QualityCommand) -> Result<()> {
    match command {
        QualityCommand::Lines => qivxif_quality::check_lines()?,
        QualityCommand::Wording => qivxif_quality::check_wording()?,
        QualityCommand::RetiredCanon => qivxif_quality::check_retired_canon()?,
        QualityCommand::PublicNames => qivxif_quality::check_public_names()?,
        QualityCommand::ImplementationMarkers => qivxif_quality::check_placeholders()?,
        QualityCommand::Workspace => qivxif_quality::check_workspace_matches_docs()?,
        QualityCommand::BrowserStorage => qivxif_quality::check_browser_storage_boundaries()?,
        QualityCommand::Routes => qivxif_quality::check_route_docs_match_api()?,
        QualityCommand::RedbTables => qivxif_quality::check_redb_tables_match_docs()?,
    }
    Ok(())
}

fn store(command: StoreCommand) -> Result<()> {
    match command {
        StoreCommand::Stats(args) => commands::store_stats(args)?,
        StoreCommand::Health(args) => commands::store_health(args)?,
        StoreCommand::RepairCheck(args) => commands::store_repair_check(args)?,
    }
    Ok(())
}
