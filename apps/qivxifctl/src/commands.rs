use crate::cli::{AdminCommand, AdminSubcommand, StorePath};
use anyhow::{Result, bail};
use qivxif_auth::hash_password;
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
            print_value(
                args.json,
                json!({ "status": "created", "user_id": user.id }),
            )?;
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
