use harfbuzz::{Blob, sys::{hb_font_t, hb_face_create, hb_font_create, hb_glyph_position_t}};

use crate::FontRef;

pub struct HbFont {
    pub hb_font: *mut hb_font_t,
}

impl HbFont {
    pub fn new(font: &FontRef) -> HbFont {
        unsafe {
            let data = font.font.copy_font_data().expect("font data unavailable");
            let blob = Blob::new_from_arc_vec(data);
            unsafe {
                let hb_face = hb_face_create(blob.as_raw(), 0);
                let hb_font = hb_font_create(hb_face);
                HbFont { hb_font }
            }
        }
    }

    pub fn convert_hb_pos(&self, pos: &hb_glyph_position_t) -> hb_glyph_position_t {
        *pos
    }
}
