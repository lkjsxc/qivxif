use qivxif_core::{BlockPos, ChunkCoord};
use qivxif_protocol::BlockCell;

pub const CHUNK_EDGE: i32 = 8;
pub const AIR: u16 = 0;
pub const STONE: u16 = 1;
pub const GRASS: u16 = 2;

pub fn generated_block(pos: BlockPos, seed: u64) -> u16 {
    let surface = ((pos.x as i64 * 17 + pos.z as i64 * 31 + seed as i64) & 1) as i32;
    if pos.y < surface {
        STONE
    } else if pos.y == surface {
        GRASS
    } else {
        AIR
    }
}

pub fn chunk_cells(coord: ChunkCoord, seed: u64, edits: &[BlockCell]) -> Vec<BlockCell> {
    let mut cells = Vec::new();
    for x in 0..CHUNK_EDGE {
        for y in 0..CHUNK_EDGE {
            for z in 0..CHUNK_EDGE {
                let pos = BlockPos {
                    x: coord.x * CHUNK_EDGE + x,
                    y,
                    z: coord.z * CHUNK_EDGE + z,
                };
                push_visible(&mut cells, pos, generated_block(pos, seed));
            }
        }
    }
    for edit in edits {
        if in_chunk(edit.pos, coord) {
            replace_cell(&mut cells, edit.clone());
        }
    }
    cells
}

fn push_visible(cells: &mut Vec<BlockCell>, pos: BlockPos, block: u16) {
    if block != AIR {
        cells.push(BlockCell { pos, block });
    }
}

fn replace_cell(cells: &mut Vec<BlockCell>, edit: BlockCell) {
    cells.retain(|cell| cell.pos != edit.pos);
    if edit.block != AIR {
        cells.push(edit);
    }
}

pub fn in_chunk(pos: BlockPos, coord: ChunkCoord) -> bool {
    pos.x.div_euclid(CHUNK_EDGE) == coord.x && pos.z.div_euclid(CHUNK_EDGE) == coord.z
}

pub fn chunk_coord(pos: BlockPos) -> ChunkCoord {
    ChunkCoord {
        x: pos.x.div_euclid(CHUNK_EDGE),
        z: pos.z.div_euclid(CHUNK_EDGE),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_chunk_is_stable() {
        let coord = ChunkCoord { x: 0, z: 0 };
        assert_eq!(chunk_cells(coord, 7, &[]), chunk_cells(coord, 7, &[]));
    }

    #[test]
    fn negative_positions_map_to_negative_chunks() {
        assert_eq!(
            chunk_coord(BlockPos { x: -1, y: 0, z: -1 }),
            ChunkCoord { x: -1, z: -1 }
        );
    }

    #[test]
    fn air_edit_removes_generated_cell() {
        let pos = BlockPos { x: 0, y: 0, z: 0 };
        let cells = chunk_cells(
            ChunkCoord { x: 0, z: 0 },
            0,
            &[BlockCell { pos, block: AIR }],
        );
        assert!(!cells.iter().any(|cell| cell.pos == pos));
    }

    #[test]
    fn overlay_replaces_generated_cell() {
        let pos = BlockPos { x: 0, y: 0, z: 0 };
        let cells = chunk_cells(
            ChunkCoord { x: 0, z: 0 },
            0,
            &[BlockCell { pos, block: STONE }],
        );
        assert_eq!(cells.iter().filter(|cell| cell.pos == pos).count(), 1);
        assert_eq!(
            cells.iter().find(|cell| cell.pos == pos).unwrap().block,
            STONE
        );
    }
}
