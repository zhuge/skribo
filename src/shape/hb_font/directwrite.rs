use harfbuzz::{
    sys::{
        directwrite::hb_directwrite_face_create, hb_font_create, hb_font_t, hb_glyph_position_t,
    },
    Blob,
};

use crate::FontRef;

pub struct HbFont {
    pub hb_font: *mut hb_font_t,
}

impl HbFont {
    pub fn new(font: &FontRef) -> HbFont {
        unsafe {
            let direct_write_font = font.font.native_font();
            let direct_write_face = direct_write_font.dwrite_font_face;
            let hb_face = hb_directwrite_face_create(direct_write_face.as_ptr());
            let hb_font = hb_font_create(hb_face);
            HbFont { hb_font }
        }
    }

    pub fn convert_hb_pos(&self, pos: &hb_glyph_position_t) -> hb_glyph_position_t {
        *pos
    }
}
