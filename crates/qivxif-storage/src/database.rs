use crate::{
    errors::{StoreError, redb_error},
    tables::{DB_FILE, META, SECTIONS},
};
use redb::{Database, Durability};
use std::{fs, path::Path};

pub(crate) fn open(root: &Path) -> Result<Database, StoreError> {
    fs::create_dir_all(root)?;
    let db = Database::create(root.join(DB_FILE)).map_err(redb_error)?;
    init_tables(&db)?;
    Ok(db)
}

fn init_tables(db: &Database) -> Result<(), StoreError> {
    let mut write = db.begin_write().map_err(redb_error)?;
    write
        .set_durability(Durability::Immediate)
        .map_err(redb_error)?;
    write.open_table(META).map_err(redb_error)?;
    write.open_table(SECTIONS).map_err(redb_error)?;
    write.commit().map_err(redb_error)?;
    Ok(())
}
