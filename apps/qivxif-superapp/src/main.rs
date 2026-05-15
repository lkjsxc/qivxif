use anyhow::Result;
use clap::{Parser, Subcommand};
use qivxif_action::{AppCommand, CommandEnvelope, CommandSource};
use qivxif_browser::BrowserPolicy;
use qivxif_editor_buffer::{TextEdit, TextRange};
use qivxif_explorer::ExplorerModel;
use qivxif_persistence::{JsonStore, TomlStore};
use qivxif_platform::StatePaths;
use qivxif_shell::{ShellModel, run_native};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Parser)]
#[command(name = "qivxif-superapp", about = "Rust-native tile workspace")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
enum Command {
    Run,
    Smoke,
}

fn main() -> Result<()> {
    match Cli::parse().command.unwrap_or(Command::Smoke) {
        Command::Run => run_app(),
        Command::Smoke => smoke(),
    }
}

fn run_app() -> Result<()> {
    let paths = StatePaths::resolve()?;
    let shell = new_shell(&paths);
    JsonStore::new(paths.workspace_json).save(&RunSnapshot::from_shell(&shell))?;
    run_native().map_err(|error| anyhow::anyhow!("native window failed: {error}"))
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

    JsonStore::new(paths.workspace_json.clone()).save(&RunSnapshot::from_shell(&shell))?;
    let saved: RunSnapshot = JsonStore::new(paths.workspace_json).load()?;
    TomlStore::new(paths.settings_toml).save(&AppSettings::default())?;
    println!(
        "qivxif smoke ok panes={} buffers={} markdown={} explorer={}",
        saved.panes, saved.buffers, saved.markdown_blocks, saved.explorer_entries
    );
    Ok(())
}

fn new_shell(paths: &StatePaths) -> ShellModel {
    let policy = BrowserPolicy::locked_down(paths.root.join("downloads"));
    ShellModel::new(policy)
}

#[derive(Debug, Serialize, Deserialize)]
struct RunSnapshot {
    panes: usize,
    buffers: usize,
    events: usize,
    markdown_blocks: usize,
    explorer_entries: usize,
    browser_open: bool,
}

impl RunSnapshot {
    fn from_shell(shell: &ShellModel) -> Self {
        Self {
            panes: shell.session.panes.len(),
            buffers: shell.buffers.len(),
            events: shell.events.len(),
            markdown_blocks: shell.markdown.blocks.len(),
            explorer_entries: shell.explorer.entries.len(),
            browser_open: shell.browser_state.is_some(),
        }
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
