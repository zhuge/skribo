#[macro_use]
extern crate log;

use font_kit::loaders::default::Font;

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
