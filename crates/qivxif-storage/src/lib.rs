use qivxif_core::{BlockPos, ChunkCoord};
use qivxif_protocol::BlockCell;
use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use std::{fs, path::Path};
use thiserror::Error;

const DB_FILE: &str = "world.redb";
const EDITS: TableDefinition<&str, &[u8]> = TableDefinition::new("section_edits");

#[derive(Debug, Error)]
pub enum StoreError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("redb: {0}")]
    Redb(String),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub struct WorldStore {
    db: Database,
}

impl WorldStore {
    pub fn open(root: &Path) -> Result<Self, StoreError> {
        fs::create_dir_all(root)?;
        let db = Database::create(root.join(DB_FILE)).map_err(redb_error)?;
        let write = db.begin_write().map_err(redb_error)?;
        write.open_table(EDITS).map_err(redb_error)?;
        write.commit().map_err(redb_error)?;
        Ok(Self { db })
    }

    pub fn put_block(&self, cell: &BlockCell) -> Result<(), StoreError> {
        let write = self.db.begin_write().map_err(redb_error)?;
        {
            let mut table = write.open_table(EDITS).map_err(redb_error)?;
            let bytes = serde_json::to_vec(cell)?;
            table
                .insert(block_key(cell.pos).as_str(), bytes.as_slice())
                .map_err(redb_error)?;
        }
        write.commit().map_err(redb_error)?;
        Ok(())
    }

    pub fn load_chunk(&self, coord: ChunkCoord) -> Result<Vec<BlockCell>, StoreError> {
        let read = self.db.begin_read().map_err(redb_error)?;
        let table = read.open_table(EDITS).map_err(redb_error)?;
        let mut edits = Vec::new();
        for entry in table.iter().map_err(redb_error)? {
            let (_, value) = entry.map_err(redb_error)?;
            let cell: BlockCell = serde_json::from_slice(value.value())?;
            if qivxif_world::in_chunk(cell.pos, coord) {
                edits.push(cell);
            }
        }
        Ok(edits)
    }
}

fn block_key(pos: BlockPos) -> String {
    format!("block/{}/{}/{}", pos.x, pos.y, pos.z)
}

fn redb_error(error: impl std::fmt::Display) -> StoreError {
    StoreError::Redb(error.to_string())
}
