use freetype::freetype::{FT_Done_Face, FT_Set_Char_Size};
use harfbuzz::sys::{hb_font_t, hb_ft_font_create_referenced, hb_glyph_position_t};

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

    pub fn convert_hb_pos(&self, pos: &hb_glyph_position_t) -> hb_glyph_position_t {
        hb_glyph_position_t {
            x_advance: pos.x_advance >> 6,
            y_advance: pos.y_advance >> 6,
            x_offset: pos.x_offset >> 6,
            y_offset: pos.y_offset >> 6,
            var: pos.var,
        }
    }
}