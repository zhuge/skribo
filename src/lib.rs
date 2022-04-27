#[macro_use]
extern crate log;

use font_kit::loaders::default::Font;
use pathfinder_geometry::vector::Vector2F;

mod collection;
mod hb_layout;
mod layout;
mod tables;
mod unicode_funcs;

pub use crate::collection::{FontCollection, FontFamily, FontRef};
pub use layout::Layout;

#[derive(Clone)]
pub struct TextStyle {
    // This should be either horiz and vert, or a 2x2 matrix
    pub size: f32,
}

// TODO: remove this (in favor of GlyphInfo as a public API)
#[derive(Debug)]
pub struct Glyph {
    pub font: FontRef,
    pub glyph_id: u32,
    pub offset: Vector2F,
    // TODO: more fields for advance, clusters, etc.
}
