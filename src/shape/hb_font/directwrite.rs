use freetype::freetype::FT_Done_Face;
use harfbuzz::sys::hb_ft_font_create_referenced;

use crate::FontRef;

pub(crate) struct HbFont {
    hb_font: *mut hb_font_t,
}

impl HbFont {
    pub fn new(font: &FontRef) -> HbFace {
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
}
