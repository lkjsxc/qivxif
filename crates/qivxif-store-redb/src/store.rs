use crate::{StoreConfig, StoreResult, codec::encode, tables};
use redb::{Database, ReadableTable, TableDefinition};
use serde::Serialize;
use std::{collections::BTreeMap, fs, path::PathBuf, sync::Arc};

#[derive(Clone)]
pub struct QivxifStore {
    pub(crate) database: Arc<Database>,
    pub(crate) path: PathBuf,
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
