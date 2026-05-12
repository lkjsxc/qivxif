use clap::{Args, Parser, Subcommand};
use qivxif_client_core::TlsMode;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "qivxif-client-desktop")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
pub enum Command {
    Run(NativeArgs),
    E2e(E2eArgs),
    SmokeFrame(SmokeFrameArgs),
}

#[derive(Args, Debug, Clone)]
pub struct NativeArgs {
    #[arg(long)]
    pub addr: String,
    #[arg(long, default_value = "localhost")]
    pub server_name: String,
    #[arg(long, default_value = "verified")]
    pub tls: TlsMode,
    #[arg(long, default_value = "desktop")]
    pub player: String,
    #[arg(long, default_value_t = 1)]
    pub radius: i32,
}

#[derive(Args, Debug, Clone)]
pub struct E2eArgs {
    #[command(flatten)]
    pub native: NativeArgs,
    #[arg(long, default_value = "/tmp/qivxif-desktop-e2e.ppm")]
    pub frame_output: PathBuf,
    #[arg(long, default_value = "/tmp/qivxif-desktop-e2e.json")]
    pub summary_output: PathBuf,
}

#[derive(Args, Debug, Clone)]
pub struct SmokeFrameArgs {
    #[arg(long)]
    pub addr: String,
    #[arg(long, default_value = "localhost")]
    pub server_name: String,
    #[arg(long, default_value = "verified")]
    pub tls: TlsMode,
    #[arg(long, default_value = "desktop-smoke")]
    pub player: String,
    #[arg(long, default_value = "/tmp/qivxif-desktop-smoke.ppm")]
    pub output: PathBuf,
    #[arg(long, default_value_t = 1)]
    pub radius: i32,
    #[arg(long, default_value_t = 128)]
    pub size: usize,
}
