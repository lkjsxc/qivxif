use crate::{
    cli::StorePath,
    cli_admin::{AdminCommand, AdminSubcommand},
};
use anyhow::{Result, bail};
use qivxif_auth::{AuthRole, hash_password};
use qivxif_core::UserId;
use qivxif_store_redb::{StoreConfig, open_or_create};
use serde_json::json;
use std::{io::Read, path::PathBuf};

pub fn admin(command: AdminCommand) -> Result<()> {
    match command.command {
        AdminSubcommand::Bootstrap(args) => {
            if !args.password_stdin {
                bail!("admin bootstrap requires --password-stdin");
            }
            let mut password = String::new();
            std::io::stdin().read_to_string(&mut password)?;
            let store = open_or_create(StoreConfig::new(args.store))?;
            let user = store.create_admin_user(args.name, hash_password(password.trim_end())?)?;
            print_value(args.json, user_created_value("created", &user))?;
        }
        AdminSubcommand::CreateUser(args) => {
            if !args.password_stdin {
                bail!("admin create-user requires --password-stdin");
            }
            let mut password = String::new();
            std::io::stdin().read_to_string(&mut password)?;
            let roles = parse_roles(&args.roles)?;
            let store = open_or_create(StoreConfig::new(args.store))?;
            let user = store.create_user(args.name, hash_password(password.trim_end())?, roles)?;
            print_value(args.json, user_created_value("created", &user))?;
        }
        AdminSubcommand::IssueInvite(args) => {
            let _ = parse_role(&args.role)?;
            let store = open_or_create(StoreConfig::new(args.store))?;
            let issued = store.issue_invite(args.role, args.expires_in, args.max_uses)?;
            print_value(
                args.json,
                json!({ "status": "created", "invite": issued.record, "secret": issued.secret }),
            )?;
        }
        AdminSubcommand::ListInvites(args) => {
            let store = open_or_create(StoreConfig::new(args.store))?;
            print_value(args.json, json!({ "invites": store.list_invites()? }))?;
        }
        AdminSubcommand::RevokeInvite(args) => {
            let store = open_or_create(StoreConfig::new(args.store))?;
            print_value(
                args.json,
                json!({ "status": "revoked", "invite": store.revoke_invite(&args.invite_id)? }),
            )?;
        }
        AdminSubcommand::IssueKey(args) => {
            let store = open_or_create(StoreConfig::new(args.store))?;
            let user_id = resolve_user_id(&store, &args.user)?;
            let scopes = non_empty_scopes(args.scopes)?;
            let issued = store.issue_access_key(user_id, scopes, args.expires_in)?;
            print_value(
                args.json,
                json!({ "status": "created", "key": issued.record, "secret": issued.secret }),
            )?;
        }
        AdminSubcommand::ListKeys(args) => {
            let store = open_or_create(StoreConfig::new(args.store))?;
            let keys = maybe_filter_keys(&store, args.user.as_deref())?;
            print_value(args.json, json!({ "keys": keys }))?;
        }
        AdminSubcommand::RevokeKey(args) => {
            let store = open_or_create(StoreConfig::new(args.store))?;
            print_value(
                args.json,
                json!({ "status": "revoked", "key": store.revoke_access_key(&args.key_id)? }),
            )?;
        }
        AdminSubcommand::AuditKeys(args) => {
            let store = open_or_create(StoreConfig::new(args.store))?;
            print_value(args.json, json!({ "audit": store.key_audit()? }))?;
        }
    }
    Ok(())
}

pub fn store_stats(args: StorePath) -> Result<()> {
    let store = open_or_create(StoreConfig::new(args.store))?;
    print_value(args.json, serde_json::to_value(store.stats()?)?)
}

pub fn store_health(args: StorePath) -> Result<()> {
    let store = open_or_create(StoreConfig::new(args.store))?;
    print_value(args.json, serde_json::to_value(store.health()?)?)
}

pub fn store_repair_check(args: StorePath) -> Result<()> {
    let store = open_or_create(StoreConfig::new(args.store))?;
    print_value(args.json, serde_json::to_value(store.repair_check()?)?)
}

pub fn feeds_rebuild(store: PathBuf, json_output: bool) -> Result<()> {
    let store = open_or_create(StoreConfig::new(store))?;
    print_value(
        json_output,
        serde_json::to_value(store.rebuild_feed_indexes()?)?,
    )
}

fn resolve_user_id(store: &qivxif_store_redb::QivxifStore, value: &str) -> Result<UserId> {
    if let Ok(user_id) = value.parse::<UserId>() {
        if store.get_user(&user_id)?.is_some() {
            return Ok(user_id);
        }
    }
    store
        .find_user_by_name(value)?
        .map(|user| user.id)
        .ok_or_else(|| anyhow::anyhow!("user not found"))
}

fn non_empty_scopes(scopes: Vec<String>) -> Result<Vec<String>> {
    if scopes.is_empty() {
        bail!("admin issue-key requires at least one --scope");
    }
    Ok(scopes)
}

fn maybe_filter_keys(
    store: &qivxif_store_redb::QivxifStore,
    user: Option<&str>,
) -> Result<Vec<qivxif_store_redb::AccessKeyRecord>> {
    let keys = store.list_access_keys()?;
    let Some(user) = user else {
        return Ok(keys);
    };
    let user_id = resolve_user_id(store, user)?;
    Ok(keys
        .into_iter()
        .filter(|key| key.user_id == user_id)
        .collect())
}

fn print_value(json_output: bool, value: serde_json::Value) -> Result<()> {
    if json_output {
        println!("{}", serde_json::to_string(&value)?);
    } else {
        println!("{}", serde_json::to_string_pretty(&value)?);
    }
    Ok(())
}

fn user_created_value(status: &str, user: &qivxif_store_redb::StoredUser) -> serde_json::Value {
    json!({
        "status": status,
        "user_id": user.id,
        "actor_id": user.actor_id,
        "profile_node_id": user.profile_node_id,
        "name": user.name,
        "roles": user.roles,
    })
}

fn parse_roles(values: &[String]) -> Result<Vec<AuthRole>> {
    if values.is_empty() {
        return Ok(vec![AuthRole::Member]);
    }
    values.iter().map(|value| parse_role(value)).collect()
}

fn parse_role(value: &str) -> Result<AuthRole> {
    match value {
        "owner" => Ok(AuthRole::Owner),
        "admin" => Ok(AuthRole::Admin),
        "member" => Ok(AuthRole::Member),
        "guest" => Ok(AuthRole::Guest),
        "public" => bail!("public is a viewer role, not a durable account role"),
        _ => bail!("unknown role '{value}'"),
    }
}
