use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Args)]
pub struct AdminCommand {
    #[command(subcommand)]
    pub command: AdminSubcommand,
}

#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
pub enum AdminSubcommand {
    Bootstrap(AdminBootstrap),
    CreateUser(AdminCreateUser),
    IssueInvite(AdminIssueInvite),
    ListInvites(AdminListInvites),
    RevokeInvite(AdminRevokeInvite),
    IssueKey(AdminIssueKey),
    ListKeys(AdminListKeys),
    RevokeKey(AdminRevokeKey),
    AuditKeys(AdminAuditKeys),
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

#[derive(Args)]
pub struct AdminCreateUser {
    #[arg(long)]
    pub store: PathBuf,
    #[arg(long)]
    pub name: String,
    #[arg(long)]
    pub password_stdin: bool,
    #[arg(long = "role")]
    pub roles: Vec<String>,
    #[arg(long)]
    pub json: bool,
}

#[derive(Args)]
pub struct AdminIssueInvite {
    #[arg(long)]
    pub store: PathBuf,
    #[arg(long, default_value = "member")]
    pub role: String,
    #[arg(long)]
    pub expires_in: Option<String>,
    #[arg(long, default_value_t = 1)]
    pub max_uses: u64,
    #[arg(long)]
    pub json: bool,
}

#[derive(Args)]
pub struct AdminListInvites {
    #[arg(long)]
    pub store: PathBuf,
    #[arg(long)]
    pub json: bool,
}

#[derive(Args)]
pub struct AdminRevokeInvite {
    #[arg(long)]
    pub store: PathBuf,
    #[arg(long)]
    pub invite_id: String,
    #[arg(long)]
    pub json: bool,
}

#[derive(Args)]
pub struct AdminIssueKey {
    #[arg(long)]
    pub store: PathBuf,
    #[arg(long)]
    pub user: String,
    #[arg(long = "scope")]
    pub scopes: Vec<String>,
    #[arg(long)]
    pub expires_in: Option<String>,
    #[arg(long)]
    pub json: bool,
}

#[derive(Args)]
pub struct AdminListKeys {
    #[arg(long)]
    pub store: PathBuf,
    #[arg(long)]
    pub user: Option<String>,
    #[arg(long)]
    pub json: bool,
}

#[derive(Args)]
pub struct AdminRevokeKey {
    #[arg(long)]
    pub store: PathBuf,
    #[arg(long)]
    pub key_id: String,
    #[arg(long)]
    pub json: bool,
}

#[derive(Args)]
pub struct AdminAuditKeys {
    #[arg(long)]
    pub store: PathBuf,
    #[arg(long)]
    pub json: bool,
}
