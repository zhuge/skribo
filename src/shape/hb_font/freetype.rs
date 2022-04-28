use freetype::freetype::FT_Done_Face;
use harfbuzz::sys::{hb_font_t, hb_ft_font_create_referenced};

use crate::FontRef;

pub struct HbFont {
    pub hb_font: *mut hb_font_t,
}

impl HbFont {
    pub fn new(font: &FontRef) -> HbFont {
        unsafe {
            let ft_face = font.font.native_font();
            let hb_font = hb_ft_font_create_referenced(ft_face);
            FT_Done_Face(ft_face);
            HbFont { hb_font }
        }
    }
}
