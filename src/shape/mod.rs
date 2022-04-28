//! A HarfBuzz shaping back-end.

mod hb_font;
mod hb_unicode_funcs;

use crate::layout::{Fragment, Glyph};
use crate::{FontRef, TextStyle};
use harfbuzz::{
    sys::{
        hb_buffer_get_glyph_infos, hb_buffer_get_glyph_positions, hb_glyph_info_get_glyph_flags,
        hb_script_t, hb_shape, HB_GLYPH_FLAG_UNSAFE_TO_BREAK,
    },
    Buffer,
};
use hb_font::HbFont;
use hb_unicode_funcs::InstallUnicodeFunc;
use pathfinder_geometry::vector::{vec2i, Vector2F};

pub(crate) fn shape(
    style: &TextStyle,
    font: &FontRef,
    script: hb_script_t,
    text: &str,
) -> Fragment {
    let mut b = Buffer::new();
    b.install_unicodd_funcs();
    b.add_str(text);
    b.set_script(script);
    b.guess_segment_properties();
    unsafe {
        let hb_font = HbFont::new(font);
        hb_shape(hb_font.hb_font, b.as_ptr(), std::ptr::null(), 0);

        let mut n_glyph = 0;
        let glyph_infos = hb_buffer_get_glyph_infos(b.as_ptr(), &mut n_glyph);
        trace!("number of glyphs: {}", n_glyph);

        let glyph_infos = std::slice::from_raw_parts(glyph_infos, n_glyph as usize);
        let mut n_glyph_pos = 0;
        let glyph_positions = hb_buffer_get_glyph_positions(b.as_ptr(), &mut n_glyph_pos);
        let glyph_positions = std::slice::from_raw_parts(glyph_positions, n_glyph_pos as usize);

        let mut total_adv = Vector2F::zero();
        let mut glyphs = Vec::new();
        // TODO: we might want to store this size-invariant.
        let scale = style.size / (font.font.metrics().units_per_em as f32);
        for (glyph, pos) in glyph_infos.iter().zip(glyph_positions.iter()) {
            let adv = vec2i(pos.x_advance, pos.y_advance).to_f32() * scale;
            let offset = vec2i(pos.x_offset, pos.y_offset).to_f32() * scale;
            let flags = hb_glyph_info_get_glyph_flags(glyph);
            let unsafe_to_break = flags & HB_GLYPH_FLAG_UNSAFE_TO_BREAK != 0;
            trace!(
                "{:?} {:?} {} {}",
                glyph.codepoint,
                (pos.x_offset, pos.y_offset),
                glyph.cluster,
                unsafe_to_break
            );

            let g = Glyph {
                cluster: glyph.cluster,
                advance: adv,
                glyph_id: glyph.codepoint,
                offset: total_adv + offset,
                unsafe_to_break,
            };
            total_adv += adv;
            glyphs.push(g);
        }

        Fragment {
            substr_len: text.len(),
            script,
            language: b.get_language(),
            direction: b.get_direction(),
            glyphs: glyphs,
            advance: total_adv,
            font: font.clone(),
        }
    }
}
