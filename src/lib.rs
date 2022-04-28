#[macro_use]
extern crate log;

use font_kit::loaders::default::Font;

mod tables;
mod collection;
mod shape;
mod layout;
mod script;

pub use crate::collection::{FontCollection, FontFamily, FontRef};
pub use layout::Layout;

#[derive(Clone)]
pub struct TextStyle {
    // This should be either horiz and vert, or a 2x2 matrix
    pub size: f32,
}
