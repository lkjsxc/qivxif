use crate::shade;
use qivxif_assets::{BlockPalette, Color};
use qivxif_protocol::BlockCell;

#[derive(Debug, Clone)]
pub struct RenderScene {
    pub cells: Vec<BlockCell>,
    pub quads: Vec<CellQuad>,
}

#[derive(Debug, Clone, Copy)]
pub struct CameraView {
    pub center_x: f32,
    pub center_z: f32,
    pub zoom: f32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameStats {
    pub gpu_frames: u64,
    pub quads: usize,
    pub cells: usize,
    pub nonzero_pixels: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct CellQuad {
    pub center: [f32; 2],
    pub size: f32,
    pub color: Color,
}

impl RenderScene {
    pub fn from_cells(cells: Vec<BlockCell>) -> Self {
        let palette = BlockPalette::default();
        let quads = cells
            .iter()
            .map(|cell| CellQuad {
                center: [cell.pos.x as f32, cell.pos.z as f32],
                size: (cell.pos.y.max(1) as f32).sqrt().max(1.0),
                color: shade(palette.color(cell.block), cell.pos.y),
            })
            .collect();
        Self { cells, quads }
    }
}

impl Default for CameraView {
    fn default() -> Self {
        Self {
            center_x: 0.0,
            center_z: 0.0,
            zoom: 16.0,
            width: 800,
            height: 600,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qivxif_core::BlockPos;

    #[test]
    fn render_scene_builds_quads_from_cells() {
        let scene = RenderScene::from_cells(vec![BlockCell {
            pos: BlockPos { x: 1, y: 2, z: 3 },
            block: 9,
        }]);
        assert_eq!(scene.quads.len(), 1);
        assert_eq!(scene.cells.len(), 1);
    }
}
