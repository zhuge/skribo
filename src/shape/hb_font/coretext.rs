use core_foundation::base::TCFType;
use harfbuzz::sys::{coretext::hb_coretext_font_create, hb_font_t, hb_glyph_position_t};

use crate::FontRef;

pub struct HbFont {
    pub hb_font: *mut hb_font_t,
}

impl HbFont {
    pub fn new(font: &FontRef) -> HbFont {
        unsafe {
            let core_text_font = font.font.native_font();
            let hb_font = hb_coretext_font_create(core_text_font.as_concrete_TypeRef() as *mut _);
            HbFont { hb_font }
        }
    }

    pub fn convert_hb_pos(&self, pos: &hb_glyph_position_t) -> hb_glyph_position_t {
        *pos
    }
}
