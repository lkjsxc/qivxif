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
use qivxif_shell::{
    AppSettings, NativeRunConfig, ShellModel, ShellSnapshot, run_native_with_model,
};
use std::{env, fs, path::PathBuf};

#[derive(Parser)]
#[command(name = "qivxif-superapp", about = "Rust-native tile super app")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
enum Command {
    Run { path: Option<PathBuf> },
    Smoke,
    SmokeNative,
}

fn main() -> Result<()> {
    init_tracing();
    match Cli::parse().command.unwrap_or(Command::Run { path: None }) {
        Command::Run { path } => run_app(path),
        Command::Smoke => smoke(),
        Command::SmokeNative => smoke_native(),
    }
}

fn run_app(path: Option<PathBuf>) -> Result<()> {
    let paths = StatePaths::resolve()?;
    let mut shell = load_shell(&paths);
    apply_start_path(&mut shell, path)?;
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
    let paths = smoke_paths()?;
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
    let saved = ShellSnapshot::load(paths.workspace_json.clone())?;
    let summary = saved.summary();
    TomlStore::new(paths.settings_toml.clone()).save(&AppSettings::default())?;
    println!(
        "qivxif smoke ok panes={} buffers={} markdown={} explorer={} state={} settings={}",
        summary.panes,
        summary.buffers,
        summary.markdown_blocks,
        summary.explorer_entries,
        paths.workspace_json.display(),
        paths.settings_toml.display()
    );
    Ok(())
}

fn new_shell(paths: &StatePaths) -> ShellModel {
    let policy = BrowserPolicy::locked_down(paths.root.join("downloads"));
    ShellModel::new(policy)
}

fn smoke_paths() -> Result<StatePaths> {
    if env::var_os("QIVXIF_STATE_DIR").is_some() {
        return Ok(StatePaths::resolve()?);
    }
    let root = env::temp_dir().join("qivxif-smoke-state");
    Ok(StatePaths {
        settings_toml: root.join("settings.toml"),
        workspace_json: root.join("workspace.json"),
        recovery_dir: root.join("recovery"),
        root,
    })
}

fn load_shell(paths: &StatePaths) -> ShellModel {
    let policy = BrowserPolicy::locked_down(paths.root.join("downloads"));
    let mut shell = match JsonStore::new(paths.workspace_json.clone()).load::<ShellSnapshot>() {
        Ok(snapshot) => snapshot.into_shell(policy),
        Err(_) => ShellModel::new(policy),
    };
    if let Ok(settings) = TomlStore::new(paths.settings_toml.clone()).load::<AppSettings>() {
        shell.session.settings = settings;
    }
    shell
}

fn apply_start_path(shell: &mut ShellModel, path: Option<PathBuf>) -> Result<()> {
    let Some(path) = path else {
        return Ok(());
    };
    if path.is_dir() {
        shell.explorer = ExplorerModel::with_root(path);
        shell.explorer.refresh_root(0)?;
    } else {
        shell.open_file(path)?;
    }
    Ok(())
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();
}
