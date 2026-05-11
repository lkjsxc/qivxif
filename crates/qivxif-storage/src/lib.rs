use qivxif_core::{BlockPos, ChunkCoord, WorldMeta};
use qivxif_protocol::BlockCell;
use redb::{Database, ReadableDatabase, TableDefinition};
use std::{fs, path::Path};
use thiserror::Error;

const DB_FILE: &str = "world.redb";
const META: TableDefinition<&str, &[u8]> = TableDefinition::new("meta");
const SECTIONS: TableDefinition<&str, &[u8]> = TableDefinition::new("sections");
const META_WORLD: &str = "world";

#[derive(Debug, Error)]
pub enum StoreError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("redb: {0}")]
    Redb(String),
    #[error(transparent)]
    Codec(#[from] postcard::Error),
}

pub struct WorldStore {
    db: Database,
    meta: WorldMeta,
}

impl WorldStore {
    pub fn open(root: &Path, seed: u64) -> Result<Self, StoreError> {
        fs::create_dir_all(root)?;
        let db = Database::create(root.join(DB_FILE)).map_err(redb_error)?;
        init_tables(&db)?;
        let meta = load_or_create_meta(&db, seed)?;
        Ok(Self { db, meta })
    }

    pub fn meta(&self) -> &WorldMeta {
        &self.meta
    }

    pub fn put_block(&self, cell: &BlockCell) -> Result<(), StoreError> {
        let coord = block_chunk(cell.pos);
        let mut cells = self.load_chunk(coord)?;
        replace_cell(&mut cells, cell.clone());
        self.put_chunk(coord, &cells)
    }

    pub fn load_chunk(&self, coord: ChunkCoord) -> Result<Vec<BlockCell>, StoreError> {
        let read = self.db.begin_read().map_err(redb_error)?;
        let table = read.open_table(SECTIONS).map_err(redb_error)?;
        match table.get(section_key(coord).as_str()).map_err(redb_error)? {
            Some(value) => Ok(postcard::from_bytes(value.value())?),
            None => Ok(Vec::new()),
        }
    }

    fn put_chunk(&self, coord: ChunkCoord, cells: &[BlockCell]) -> Result<(), StoreError> {
        let write = self.db.begin_write().map_err(redb_error)?;
        {
            let mut table = write.open_table(SECTIONS).map_err(redb_error)?;
            let bytes = postcard::to_stdvec(cells)?;
            table
                .insert(section_key(coord).as_str(), bytes.as_slice())
                .map_err(redb_error)?;
        }
        write.commit().map_err(redb_error)?;
        Ok(())
    }
}

fn init_tables(db: &Database) -> Result<(), StoreError> {
    let write = db.begin_write().map_err(redb_error)?;
    write.open_table(META).map_err(redb_error)?;
    write.open_table(SECTIONS).map_err(redb_error)?;
    write.commit().map_err(redb_error)?;
    Ok(())
}

fn load_or_create_meta(db: &Database, seed: u64) -> Result<WorldMeta, StoreError> {
    if let Some(meta) = load_meta(db)? {
        return Ok(meta);
    }
    let meta = WorldMeta::new(seed);
    let write = db.begin_write().map_err(redb_error)?;
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

fn load_meta(db: &Database) -> Result<Option<WorldMeta>, StoreError> {
    let read = db.begin_read().map_err(redb_error)?;
    let table = read.open_table(META).map_err(redb_error)?;
    match table.get(META_WORLD).map_err(redb_error)? {
        Some(value) => Ok(Some(postcard::from_bytes(value.value())?)),
        None => Ok(None),
    }
}

fn replace_cell(cells: &mut Vec<BlockCell>, edit: BlockCell) {
    cells.retain(|cell| cell.pos != edit.pos);
    if edit.block != qivxif_world::AIR {
        cells.push(edit);
    }
}

fn block_chunk(pos: BlockPos) -> ChunkCoord {
    ChunkCoord {
        x: pos.x.div_euclid(qivxif_world::CHUNK_EDGE),
        z: pos.z.div_euclid(qivxif_world::CHUNK_EDGE),
    }
}

fn section_key(coord: ChunkCoord) -> String {
    format!("section/{}/{}", coord.x, coord.z)
}

fn redb_error(error: impl std::fmt::Display) -> StoreError {
    StoreError::Redb(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_edits_by_section() {
        let root = tempfile::tempdir().unwrap();
        let store = WorldStore::open(root.path(), 55).unwrap();
        let cell = BlockCell {
            pos: BlockPos { x: 9, y: 2, z: 1 },
            block: 4,
        };
        store.put_block(&cell).unwrap();
        assert_eq!(
            store.load_chunk(ChunkCoord { x: 1, z: 0 }).unwrap(),
            vec![cell]
        );
    }

    #[test]
    fn reloads_existing_meta() {
        let root = tempfile::tempdir().unwrap();
        assert_eq!(
            WorldStore::open(root.path(), 7).unwrap().meta().world_seed,
            7
        );
        assert_eq!(
            WorldStore::open(root.path(), 9).unwrap().meta().world_seed,
            7
        );
    }
}
