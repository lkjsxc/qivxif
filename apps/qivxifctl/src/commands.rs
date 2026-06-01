use crate::cli::{AdminCommand, AdminSubcommand, StorePath};
use anyhow::{Result, bail};
use qivxif_auth::{AuthRole, hash_password};
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
