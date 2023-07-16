pub const GLYPH_WIDTH: usize = 6;
pub const GLYPH_HEIGHT: usize = 8;

pub type Glyph = [[bool; GLYPH_WIDTH]; GLYPH_HEIGHT];
pub struct Font {
    pub x: Glyph,
    pub y: Glyph,
    pub one: Glyph,
    pub two: Glyph,
    pub three: Glyph,
    pub four: Glyph,
    pub five: Glyph,
    pub six: Glyph,
    pub seven: Glyph,
    pub eight: Glyph,
    pub nine: Glyph,
    pub par_open: Glyph,
    pub par_close: Glyph,
}
