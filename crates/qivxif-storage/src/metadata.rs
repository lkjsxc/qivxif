use crate::{
    errors::{StoreError, redb_error},
    tables::{META, META_WORLD},
};
use qivxif_core::WorldMeta;
use redb::{Database, Durability, ReadableDatabase};

pub(crate) fn load_or_create(db: &Database, seed: u64) -> Result<WorldMeta, StoreError> {
    if let Some(meta) = load(db)? {
        return Ok(meta);
    }
    let meta = WorldMeta::new(seed);
    let mut write = db.begin_write().map_err(redb_error)?;
    write
        .set_durability(Durability::Immediate)
        .map_err(redb_error)?;
    {
        let mut table = write.open_table(META).map_err(redb_error)?;
        let bytes = postcard::to_stdvec(&meta)?;
        table
            .insert(META_WORLD, bytes.as_slice())
            .map_err(redb_error)?;
    }
    write.commit().map_err(redb_error)?;
    Ok(meta)
}

fn load(db: &Database) -> Result<Option<WorldMeta>, StoreError> {
    let read = db.begin_read().map_err(redb_error)?;
    let table = read.open_table(META).map_err(redb_error)?;
    match table.get(META_WORLD).map_err(redb_error)? {
        Some(value) => Ok(Some(postcard::from_bytes(value.value())?)),
        None => Ok(None),
    }
}
