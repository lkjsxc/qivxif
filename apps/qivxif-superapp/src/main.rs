#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use anyhow::Result;
use clap::{Parser, Subcommand};
use qivxif_action::{AppCommand, CommandEnvelope, CommandSource};
use qivxif_browser::BrowserPolicy;
use qivxif_editor_buffer::{TextEdit, TextRange};
use qivxif_explorer::ExplorerModel;
use qivxif_persistence::{JsonStore, TomlStore};
use qivxif_platform::StatePaths;
use qivxif_shell::{NativeRunConfig, ShellModel, ShellSnapshot, run_native_with_model};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Parser)]
#[command(name = "qivxif-superapp", about = "Rust-native tile super app")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
enum Command {
    Run,
    Smoke,
    SmokeNative,
}

fn main() -> Result<()> {
    match Cli::parse().command.unwrap_or(Command::Run) {
        Command::Run => run_app(),
        Command::Smoke => smoke(),
        Command::SmokeNative => smoke_native(),
    }
}

fn run_app() -> Result<()> {
    let paths = StatePaths::resolve()?;
    let shell = load_shell(&paths);
    ShellSnapshot::from_shell(&shell).save(paths.workspace_json.clone())?;
    let config = NativeRunConfig {
        state_path: Some(paths.workspace_json),
        ..NativeRunConfig::default()
    };
    run_native_with_model(shell, config)
        .map_err(|error| anyhow::anyhow!("native window failed: {error}"))
}

fn smoke_native() -> Result<()> {
    let paths = StatePaths::resolve()?;
    let shell = new_shell(&paths);
    let config = NativeRunConfig {
        close_after_frames: Some(1),
        ..NativeRunConfig::default()
    };
    run_native_with_model(shell, config).map_err(|error| anyhow::anyhow!("{error}"))
}

fn smoke() -> Result<()> {
    let paths = StatePaths::resolve()?;
    fs::create_dir_all(&paths.root)?;
    let sample = paths.root.join("sample.md");
    fs::write(&sample, "# qivxif\n\nsource")?;

    let mut shell = new_shell(&paths);
    shell.open_file(sample.clone())?;
    shell
        .buffers
        .last_mut()
        .expect("opened buffer")
        .apply(TextEdit::replace(TextRange::new(10, 16), "edited"))?;
    shell.save_focused()?;
    shell.render_markdown(&fs::read_to_string(&sample)?);
    shell.dispatch(CommandEnvelope::new(
        CommandSource::Startup,
        AppCommand::ToggleExplorer,
    ));
    shell.dispatch(CommandEnvelope::new(
        CommandSource::Startup,
        AppCommand::OpenBrowser("https://example.com".to_owned()),
    ));
    shell.explorer = ExplorerModel::with_root(paths.root.clone());
    shell.explorer.refresh_root(0)?;

    ShellSnapshot::from_shell(&shell).save(paths.workspace_json.clone())?;
    let saved = ShellSnapshot::load(paths.workspace_json)?;
    let summary = saved.summary();
    TomlStore::new(paths.settings_toml).save(&AppSettings::default())?;
    println!(
        "qivxif smoke ok panes={} buffers={} markdown={} explorer={}",
        summary.panes, summary.buffers, summary.markdown_blocks, summary.explorer_entries
    );
    Ok(())
}

fn new_shell(paths: &StatePaths) -> ShellModel {
    let policy = BrowserPolicy::locked_down(paths.root.join("downloads"));
    ShellModel::new(policy)
}

fn load_shell(paths: &StatePaths) -> ShellModel {
    let policy = BrowserPolicy::locked_down(paths.root.join("downloads"));
    match JsonStore::new(paths.workspace_json.clone()).load::<ShellSnapshot>() {
        Ok(snapshot) => snapshot.into_shell(policy),
        Err(_) => ShellModel::new(policy),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AppSettings {
    theme: String,
    font_size: u16,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "dark".to_owned(),
            font_size: 14,
        }
    }
}
