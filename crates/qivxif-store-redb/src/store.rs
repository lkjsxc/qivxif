use crate::{
    StoreConfig, StoreError, StoreResult,
    codec::{decode, encode},
    records::{StoredSession, StoredUser},
    tables,
};
use qivxif_auth::{AuthRole, PasswordHashString};
use qivxif_core::{ActorId, SessionId, UserId};
use redb::{Database, ReadableTable, TableDefinition};
use serde::Serialize;
use std::{collections::BTreeMap, fs, path::PathBuf, sync::Arc};

#[derive(Clone)]
pub struct QivxifStore {
    database: Arc<Database>,
    path: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StoreStats {
    pub path: PathBuf,
    pub table_counts: BTreeMap<String, u64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StoreHealth {
    pub ok: bool,
    pub table_count: usize,
}

pub fn open_or_create(config: StoreConfig) -> StoreResult<QivxifStore> {
    if let Some(parent) = config.database_file.parent() {
        fs::create_dir_all(parent)?;
    }
    let database = if config.database_file.exists() {
        Database::open(&config.database_file)?
    } else {
        Database::create(&config.database_file)?
    };
    let store = QivxifStore {
        database: Arc::new(database),
        path: config.database_file,
    };
    store.initialize()?;
    Ok(store)
}

impl QivxifStore {
    fn initialize(&self) -> StoreResult<()> {
        let tx = self.database.begin_write()?;
        for (_, table) in tables::ALL {
            tx.open_table(*table)?;
        }
        {
            let mut meta = tx.open_table(tables::META)?;
            meta.insert("schema_contract", encode(&"qivxif-web-server")?.as_slice())?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn stats(&self) -> StoreResult<StoreStats> {
        let tx = self.database.begin_read()?;
        let mut table_counts = BTreeMap::new();
        for (name, table) in tables::ALL {
            table_counts.insert((*name).to_owned(), count_table(&tx, *table)?);
        }
        Ok(StoreStats {
            path: self.path.clone(),
            table_counts,
        })
    }

    pub fn health(&self) -> StoreResult<StoreHealth> {
        Ok(StoreHealth {
            ok: self.stats()?.table_counts.len() == tables::ALL.len(),
            table_count: tables::ALL.len(),
        })
    }

    pub fn create_admin_user(
        &self,
        name: String,
        password_hash: PasswordHashString,
    ) -> StoreResult<StoredUser> {
        let tx = self.database.begin_write()?;
        let user = StoredUser {
            id: UserId::generate(),
            actor_id: ActorId::generate(),
            name,
            password_hash,
            roles: vec![AuthRole::Owner, AuthRole::Admin],
        };
        {
            let mut names = tx.open_table(tables::USER_NAMES)?;
            if names.iter()?.next().is_some() {
                return Err(StoreError::AdminExists);
            }
            if names.get(user.name.as_str())?.is_some() {
                return Err(StoreError::DuplicateUserName);
            }
            let mut users = tx.open_table(tables::USERS)?;
            users.insert(user.id.as_str(), encode(&user)?.as_slice())?;
            names.insert(user.name.as_str(), encode(&user.id)?.as_slice())?;
        }
        tx.commit()?;
        Ok(user)
    }

    pub fn find_user_by_name(&self, name: &str) -> StoreResult<Option<StoredUser>> {
        let tx = self.database.begin_read()?;
        let names = tx.open_table(tables::USER_NAMES)?;
        let Some(user_id_bytes) = names.get(name)? else {
            return Ok(None);
        };
        let user_id: UserId = decode(user_id_bytes.value())?;
        self.get_user(&user_id)
    }

    pub fn get_user(&self, user_id: &UserId) -> StoreResult<Option<StoredUser>> {
        let tx = self.database.begin_read()?;
        let users = tx.open_table(tables::USERS)?;
        users
            .get(user_id.as_str())?
            .map(|bytes| decode(bytes.value()))
            .transpose()
    }

    pub fn create_session(&self, session: StoredSession) -> StoreResult<StoredSession> {
        let tx = self.database.begin_write()?;
        {
            let mut sessions = tx.open_table(tables::SESSIONS)?;
            sessions.insert(session.id.as_str(), encode(&session)?.as_slice())?;
        }
        tx.commit()?;
        Ok(session)
    }

    pub fn get_session(&self, session_id: &SessionId) -> StoreResult<Option<StoredSession>> {
        let tx = self.database.begin_read()?;
        let sessions = tx.open_table(tables::SESSIONS)?;
        sessions
            .get(session_id.as_str())?
            .map(|bytes| decode(bytes.value()))
            .transpose()
    }

    pub fn delete_session(&self, session_id: &SessionId) -> StoreResult<()> {
        let tx = self.database.begin_write()?;
        {
            let mut sessions = tx.open_table(tables::SESSIONS)?;
            sessions.remove(session_id.as_str())?;
        }
        tx.commit()?;
        Ok(())
    }
}

fn count_table(
    tx: &redb::ReadTransaction,
    table: TableDefinition<&str, &[u8]>,
) -> StoreResult<u64> {
    let table = tx.open_table(table)?;
    let mut count = 0;
    for item in table.iter()? {
        item?;
        count += 1;
    }
    Ok(count)
}
