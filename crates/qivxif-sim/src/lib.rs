use bevy_ecs::prelude::*;
use qivxif_core::{BlockPos, ChunkCoord};
use qivxif_protocol::BlockCell;
use qivxif_storage::{StoreError, WorldStore};
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

#[cfg(test)]
mod region_tests;

#[derive(Clone)]
pub struct RegionHandle {
    tx: mpsc::Sender<RegionCmd>,
}

impl RegionHandle {
    pub fn spawn(seed: u64, store: Arc<WorldStore>) -> Self {
        let (tx, rx) = mpsc::channel(64);
        tokio::spawn(run_region(rx, Region::new(seed, store)));
        Self { tx }
    }

    pub async fn chunk(&self, coord: ChunkCoord) -> Result<Vec<BlockCell>, SimError> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(RegionCmd::Chunk { coord, reply })
            .await
            .map_err(|_| SimError::Closed)?;
        rx.await.map_err(|_| SimError::Closed)?
    }

    pub async fn place_block(&self, pos: BlockPos, block: u16) -> Result<BlockCell, SimError> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(RegionCmd::PlaceBlock { pos, block, reply })
            .await
            .map_err(|_| SimError::Closed)?;
        rx.await.map_err(|_| SimError::Closed)?
    }

    pub async fn flush(&self) -> Result<(), SimError> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(RegionCmd::Flush { reply })
            .await
            .map_err(|_| SimError::Closed)?;
        rx.await.map_err(|_| SimError::Closed)?
    }
}

enum RegionCmd {
    Chunk {
        coord: ChunkCoord,
        reply: oneshot::Sender<Result<Vec<BlockCell>, SimError>>,
    },
    PlaceBlock {
        pos: BlockPos,
        block: u16,
        reply: oneshot::Sender<Result<BlockCell, SimError>>,
    },
    Flush {
        reply: oneshot::Sender<Result<(), SimError>>,
    },
}

struct Region {
    seed: u64,
    store: Arc<WorldStore>,
    dirty: HashMap<ChunkCoord, Vec<BlockCell>>,
    ecs: World,
}

impl Region {
    fn new(seed: u64, store: Arc<WorldStore>) -> Self {
        let mut ecs = World::new();
        ecs.insert_resource(RegionStats::default());
        Self {
            seed,
            store,
            dirty: HashMap::new(),
            ecs,
        }
    }

    fn chunk(&self, coord: ChunkCoord) -> Result<Vec<BlockCell>, SimError> {
        let mut edits = self.store.load_chunk(coord)?;
        if let Some(dirty) = self.dirty.get(&coord) {
            for cell in dirty {
                replace_cell(&mut edits, cell.clone());
            }
        }
        Ok(qivxif_world::chunk_cells(coord, self.seed, &edits))
    }

    fn place_block(&mut self, pos: BlockPos, block: u16) -> Result<BlockCell, SimError> {
        validate_pos(pos)?;
        let cell = BlockCell { pos, block };
        let coord = qivxif_world::chunk_coord(pos);
        if !self.dirty.contains_key(&coord) {
            self.dirty.insert(coord, self.store.load_chunk(coord)?);
        }
        replace_cell(
            self.dirty.get_mut(&coord).expect("dirty chunk exists"),
            cell.clone(),
        );
        self.ecs.resource_mut::<RegionStats>().mutations += 1;
        Ok(cell)
    }

    fn flush(&mut self) -> Result<(), SimError> {
        for (coord, cells) in self.dirty.drain() {
            self.store.put_chunk(coord, &cells)?;
        }
        self.ecs.resource_mut::<RegionStats>().flushes += 1;
        Ok(())
    }
}

#[derive(Default, Resource)]
struct RegionStats {
    mutations: u64,
    flushes: u64,
}

async fn run_region(mut rx: mpsc::Receiver<RegionCmd>, mut region: Region) {
    while let Some(cmd) = rx.recv().await {
        match cmd {
            RegionCmd::Chunk { coord, reply } => {
                let _ = reply.send(region.chunk(coord));
            }
            RegionCmd::PlaceBlock { pos, block, reply } => {
                let _ = reply.send(region.place_block(pos, block));
            }
            RegionCmd::Flush { reply } => {
                let _ = reply.send(region.flush());
            }
        }
    }
}

fn validate_pos(pos: BlockPos) -> Result<(), SimError> {
    if (0..qivxif_world::CHUNK_EDGE).contains(&pos.y) {
        Ok(())
    } else {
        Err(SimError::InvalidBlockPos(pos))
    }
}

fn replace_cell(cells: &mut Vec<BlockCell>, edit: BlockCell) {
    cells.retain(|cell| cell.pos != edit.pos);
    cells.push(edit);
}

#[derive(Debug, Error)]
pub enum SimError {
    #[error("region actor closed")]
    Closed,
    #[error("invalid block position: {0:?}")]
    InvalidBlockPos(BlockPos),
    #[error(transparent)]
    Store(#[from] StoreError),
}
