use qivxif_core::{BlockPos, ChunkCoord};
use qivxif_protocol::BlockCell;
use qivxif_storage::{StoreError, WorldStore};
use std::sync::Arc;
use thiserror::Error;

pub struct Region {
    seed: u64,
    store: Arc<WorldStore>,
}

impl Region {
    pub fn new(seed: u64, store: Arc<WorldStore>) -> Self {
        Self { seed, store }
    }

    pub fn chunk(&self, coord: ChunkCoord) -> Result<Vec<BlockCell>, SimError> {
        let edits = self.store.load_chunk(coord)?;
        Ok(qivxif_world::chunk_cells(coord, self.seed, &edits))
    }

    pub fn place_block(&self, pos: BlockPos, block: u16) -> Result<BlockCell, SimError> {
        let cell = BlockCell { pos, block };
        self.store.put_block(&cell)?;
        Ok(cell)
    }
}

#[derive(Debug, Error)]
pub enum SimError {
    #[error(transparent)]
    Store(#[from] StoreError),
}
