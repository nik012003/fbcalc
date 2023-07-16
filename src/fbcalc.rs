use crate::font::*;

pub struct FbCalc<'a> {
    pub width: usize,
    pub height: usize,
    pub font: Font,
    pub fb: &'a mut [bool],
}

impl FbCalc<'_> {
    pub fn render(&mut self) {
        self.render_glyph(self.font.x, 0, 0);
    }

    pub fn get(&mut self, i: usize, j: usize) -> &mut bool {
        &mut self.fb[(i * self.width) + j]
    }

    pub fn render_glyph(&mut self, glyph: Glyph, x: usize, y: usize) {
        for i in 0..GLYPH_HEIGHT {
            for j in 0..GLYPH_WIDTH {
                *self.get(x + i, y + j) = glyph[i][j];
            }
        }
    }
}
