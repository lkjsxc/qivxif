//! Embedded fallback visual assets for deterministic smoke and native
//! renderer paths.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, Clone)]
pub struct BlockPalette {
    fallback: Color,
}

impl Default for BlockPalette {
    fn default() -> Self {
        Self {
            fallback: Color::new(214, 80, 118),
        }
    }
}

impl BlockPalette {
    pub fn color(&self, block: u16) -> Color {
        match block {
            0 => Color::new(0, 0, 0),
            1 => Color::new(103, 105, 110),
            2 => Color::new(80, 156, 86),
            9 => Color::new(87, 125, 214),
            _ => self.fallback,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallback_palette_is_stable() {
        let palette = BlockPalette::default();
        assert_eq!(palette.color(2), Color::new(80, 156, 86));
        assert_eq!(palette.color(77), Color::new(214, 80, 118));
    }
}
