use qivxif_core::ChunkCoord;
use qivxif_protocol::BlockCell;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct WorldCache {
    chunks: HashMap<ChunkCoord, Vec<BlockCell>>,
}

impl WorldCache {
    pub fn insert_chunk(&mut self, coord: ChunkCoord, cells: Vec<BlockCell>) {
        self.chunks.insert(coord, cells);
    }

    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn cell_count(&self) -> usize {
        self.chunks.values().map(Vec::len).sum()
    }

    pub fn cells(&self) -> Vec<BlockCell> {
        let mut cells: Vec<_> = self
            .chunks
            .values()
            .flat_map(|chunk| chunk.iter().cloned())
            .collect();
        cells.sort_by_key(|cell| (cell.pos.x, cell.pos.y, cell.pos.z, cell.block));
        cells
    }

    pub fn contains_cell(&self, cell: &BlockCell) -> bool {
        self.chunks
            .get(&qivxif_world::chunk_coord(cell.pos))
            .is_some_and(|chunk| chunk.contains(cell))
    }

    pub fn apply_cell(&mut self, cell: BlockCell) {
        let coord = qivxif_world::chunk_coord(cell.pos);
        let chunk = self.chunks.entry(coord).or_default();
        chunk.retain(|existing| existing.pos != cell.pos);
        if cell.block != qivxif_world::AIR {
            chunk.push(cell);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qivxif_core::BlockPos;

    #[test]
    fn cache_replaces_cells() {
        let mut cache = WorldCache::default();
        let pos = BlockPos { x: 1, y: 2, z: 3 };
        cache.apply_cell(BlockCell { pos, block: 1 });
        cache.apply_cell(BlockCell { pos, block: 2 });
        assert_eq!(cache.cells(), vec![BlockCell { pos, block: 2 }]);
        assert!(cache.contains_cell(&BlockCell { pos, block: 2 }));
        assert!(!cache.contains_cell(&BlockCell { pos, block: 1 }));
    }

    #[test]
    fn air_removes_cell() {
        let mut cache = WorldCache::default();
        let pos = BlockPos { x: 1, y: 2, z: 3 };
        cache.apply_cell(BlockCell { pos, block: 1 });
        cache.apply_cell(BlockCell {
            pos,
            block: qivxif_world::AIR,
        });
        assert!(cache.cells().is_empty());
        assert!(!cache.contains_cell(&BlockCell { pos, block: 1 }));
    }
}
