use qivxif_assets::{BlockPalette, Color};
use qivxif_protocol::BlockCell;

#[derive(Debug, Clone)]
pub struct SmokeFrame {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl SmokeFrame {
    pub fn render(cells: &[BlockCell], width: usize, height: usize) -> Self {
        let mut frame = Self::blank(width, height);
        if cells.is_empty() || width == 0 || height == 0 {
            return frame;
        }
        let bounds = Bounds::from_cells(cells);
        let palette = BlockPalette::default();
        for cell in cells {
            let color = shade(palette.color(cell.block), cell.pos.y);
            let x = project(cell.pos.x, bounds.min_x, bounds.max_x, width);
            let y = project(cell.pos.z, bounds.min_z, bounds.max_z, height);
            frame.plot(x, height.saturating_sub(1).saturating_sub(y), color);
        }
        frame
    }

    pub fn to_ppm(&self) -> Vec<u8> {
        let mut out = format!("P3\n{} {}\n255\n", self.width, self.height);
        for pixel in &self.pixels {
            out.push_str(&format!("{} {} {}\n", pixel.r, pixel.g, pixel.b));
        }
        out.into_bytes()
    }

    pub fn is_nonblank(&self) -> bool {
        self.pixels
            .iter()
            .any(|pixel| *pixel != Color::new(0, 0, 0))
    }

    fn blank(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0, 0, 0); width.saturating_mul(height)],
        }
    }

    fn plot(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }
}

struct Bounds {
    min_x: i32,
    max_x: i32,
    min_z: i32,
    max_z: i32,
}

impl Bounds {
    fn from_cells(cells: &[BlockCell]) -> Self {
        let first = cells[0].pos;
        cells.iter().fold(
            Self {
                min_x: first.x,
                max_x: first.x,
                min_z: first.z,
                max_z: first.z,
            },
            |mut bounds, cell| {
                bounds.min_x = bounds.min_x.min(cell.pos.x);
                bounds.max_x = bounds.max_x.max(cell.pos.x);
                bounds.min_z = bounds.min_z.min(cell.pos.z);
                bounds.max_z = bounds.max_z.max(cell.pos.z);
                bounds
            },
        )
    }
}

fn project(value: i32, min: i32, max: i32, size: usize) -> usize {
    if size <= 1 || max == min {
        return 0;
    }
    let span = (max - min) as i64;
    let offset = (value - min) as i64;
    ((offset * (size as i64 - 1)) / span) as usize
}

fn shade(color: Color, y: i32) -> Color {
    let lift = y.clamp(0, 12) as u8 * 4;
    Color::new(
        color.r.saturating_add(lift),
        color.g.saturating_add(lift),
        color.b.saturating_add(lift),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use qivxif_core::BlockPos;

    #[test]
    fn smoke_frame_is_nonblank_for_cells() {
        let cells = vec![BlockCell {
            pos: BlockPos { x: 1, y: 2, z: 3 },
            block: 2,
        }];
        assert!(SmokeFrame::render(&cells, 16, 16).is_nonblank());
    }
}
