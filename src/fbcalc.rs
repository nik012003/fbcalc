use alloc::vec::Vec;

use crate::font::*;

pub struct FbCalc {
    font: Font,
    pub fb: Vec<Vec<bool>>,
}

impl FbCalc {
    pub fn new(width: usize, height: usize, font: Font) -> FbCalc {
        FbCalc {
            font,
            fb: vec![vec![false; width]; height],
        }
    }

    pub fn render(&mut self) {
        self.render_glyph(self.font.x, 0, 0);
    }

    pub fn render_glyph(&mut self, glyph: Glyph, x: usize, y: usize) {
        for i in 0..GLYPH_HEIGHT {
            for j in 0..GLYPH_WIDTH {
                self.fb[y + i][x + j] = glyph[i][j];
            }
        }
    }
}
