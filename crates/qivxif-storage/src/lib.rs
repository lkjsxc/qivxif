mod archives;
mod database;
mod edit_overlays;
mod errors;
mod metadata;
mod tables;

pub use archives::ArchiveStore;
pub use errors::StoreError;

use qivxif_core::{ChunkCoord, WorldMeta};
use qivxif_protocol::BlockCell;
use redb::Database;
use std::path::Path;

pub struct WorldStore {
    db: Database,
    meta: WorldMeta,
}

impl WorldStore {
    pub fn open(root: &Path, seed: u64) -> Result<Self, StoreError> {
        let db = database::open(root)?;
        let meta = metadata::load_or_create(&db, seed)?;
        Ok(Self { db, meta })
    }

    pub fn meta(&self) -> &WorldMeta {
        &self.meta
    }

    pub fn put_block(&self, cell: &BlockCell) -> Result<(), StoreError> {
        let coord = qivxif_world::chunk_coord(cell.pos);
        let mut cells = self.load_chunk(coord)?;
        edit_overlays::replace_cell(&mut cells, cell.clone());
        self.put_chunk(coord, &cells)
    }

    pub fn load_chunk(&self, coord: ChunkCoord) -> Result<Vec<BlockCell>, StoreError> {
        edit_overlays::load_chunk_overlay(&self.db, coord)
    }

    pub fn put_chunk(&self, coord: ChunkCoord, cells: &[BlockCell]) -> Result<(), StoreError> {
        edit_overlays::put_chunk_overlay(&self.db, coord, cells)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qivxif_core::BlockPos;

    #[test]
    fn stores_edits_by_chunk_overlay() {
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
    fn stores_air_as_removal_override() {
        let root = tempfile::tempdir().unwrap();
        let store = WorldStore::open(root.path(), 55).unwrap();
        let cell = BlockCell {
            pos: BlockPos { x: 1, y: 0, z: 1 },
            block: qivxif_world::AIR,
        };
        store.put_block(&cell).unwrap();
        assert_eq!(
            store.load_chunk(ChunkCoord { x: 0, z: 0 }).unwrap(),
            vec![cell]
        );
    }

    #[test]
    fn negative_chunk_key_matches_docs() {
        assert_eq!(
            edit_overlays::chunk_overlay_key(ChunkCoord { x: -1, z: -2 }),
            "section/-1/-2"
        );
    }

    #[test]
    fn stores_negative_chunk_overlay() {
        let root = tempfile::tempdir().unwrap();
        let store = WorldStore::open(root.path(), 55).unwrap();
        let cell = BlockCell {
            pos: BlockPos { x: -1, y: 0, z: -1 },
            block: qivxif_world::AIR,
        };
        store.put_block(&cell).unwrap();
        assert_eq!(
            store.load_chunk(ChunkCoord { x: -1, z: -1 }).unwrap(),
            vec![cell]
        );
    }

    #[test]
    fn repeated_mutation_replaces_previous_value() {
        let root = tempfile::tempdir().unwrap();
        let store = WorldStore::open(root.path(), 55).unwrap();
        let pos = BlockPos { x: 2, y: 0, z: 2 };
        store.put_block(&BlockCell { pos, block: 4 }).unwrap();
        store.put_block(&BlockCell { pos, block: 7 }).unwrap();
        assert_eq!(
            store.load_chunk(ChunkCoord { x: 0, z: 0 }).unwrap(),
            vec![BlockCell { pos, block: 7 }]
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
