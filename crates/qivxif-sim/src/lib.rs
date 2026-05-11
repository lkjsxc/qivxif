use bevy_ecs::prelude::*;
use qivxif_core::{BlockPos, ChunkCoord};
use qivxif_protocol::BlockCell;
use qivxif_storage::{StoreError, WorldStore};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

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
    ecs: World,
}

impl Region {
    fn new(seed: u64, store: Arc<WorldStore>) -> Self {
        let mut ecs = World::new();
        ecs.insert_resource(RegionStats::default());
        Self { seed, store, ecs }
    }

    fn chunk(&self, coord: ChunkCoord) -> Result<Vec<BlockCell>, SimError> {
        let edits = self.store.load_chunk(coord)?;
        Ok(qivxif_world::chunk_cells(coord, self.seed, &edits))
    }

    fn place_block(&mut self, pos: BlockPos, block: u16) -> Result<BlockCell, SimError> {
        let cell = BlockCell { pos, block };
        self.store.put_block(&cell)?;
        self.ecs.resource_mut::<RegionStats>().mutations += 1;
        Ok(cell)
    }
}

#[derive(Default, Resource)]
struct RegionStats {
    mutations: u64,
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
                let _ = reply.send(Ok(()));
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum SimError {
    #[error("region actor closed")]
    Closed,
    #[error(transparent)]
    Store(#[from] StoreError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn actor_serializes_mutation() {
        let root = tempfile::tempdir().unwrap();
        let store = Arc::new(WorldStore::open(root.path(), 3).unwrap());
        let region = RegionHandle::spawn(3, store);
        let pos = BlockPos { x: 1, y: 3, z: 1 };
        region.place_block(pos, 8).await.unwrap();
        let cells = region.chunk(ChunkCoord { x: 0, z: 0 }).await.unwrap();
        assert!(cells.contains(&BlockCell { pos, block: 8 }));
    }
}
