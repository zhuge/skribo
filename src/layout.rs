//! Retained layout that supports substring queries.

use std::ops::Range;

use harfbuzz::sys::hb_script_t;
use harfbuzz::{Direction, Language};

use pathfinder_geometry::vector::Vector2F;

use crate::script::Itemizer as ScriptItemizer;
use crate::shape::shape;
use crate::{FontCollection, FontRef, TextStyle};

#[allow(unused)]
pub struct Layout<S: AsRef<str>> {
    text: S,
    style: TextStyle,
    fragments: Vec<Fragment>,
}

#[allow(unused)]
pub struct Fragment {
    text_range: Range<usize>,
    script: hb_script_t,
    language: Language,
    direction: Direction,
    advance: Vector2F,
    glyphs: Vec<Glyph>,
    font: FontRef,
}

#[allow(unused)]
pub struct Glyph {
    pub cluster: u32,
    pub glyph_id: u32,
    pub offset: Vector2F,
    pub advance: Vector2F,
    pub unsafe_to_break: bool,
}

impl<S: AsRef<str>> Layout<S> {
    pub fn create(text: S, style: TextStyle, collection: &FontCollection) -> Layout<S> {
        let mut fragments = Vec::new();

        let tx = &text.as_ref()[..];
        for (script_range, script) in ScriptItemizer::new(tx) {
            let substr = &tx[script_range.clone()];
            let start = script_range.start;
            for (range, font) in collection.itemize(substr) {
                let (direction, language, glyphs) =
                    shape(&style, font, script, &substr[range.clone()]);

                let total_adv_x: f32 = glyphs.iter().map(|g| g.advance.x()).sum();
                let total_adv_y: f32 = glyphs.iter().map(|g| g.advance.y()).sum();
                fragments.push(Fragment {
                    text_range: (start + range.clone().start)..(start + range.end),
                    script,
                    language,
                    direction,
                    advance: Vector2F::new(total_adv_x, total_adv_y),
                    glyphs,
                    font: font.clone(),
                });
            }
        }

        Layout {
            text,
            style,
            fragments,
        }
    }

    /// Iterate through all glyphs in the layout.
    pub fn fragments(&self) -> FragmentIter {
        FragmentIter {
            offset: Vector2F::zero(),
            fragments: &self.fragments,
            ix: 0,
        }
    }
}

#[allow(unused)]
impl Fragment {
    pub fn font(&self) -> &FontRef {
        &self.font
    }

    pub fn script(&self) -> hb_script_t {
        self.script
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn advance(&self) -> Vector2F {
        self.advance
    }

    pub fn text_range(&self) -> Range<usize> {
        self.text_range.clone()
    }

    pub fn glyphs(&self) -> GlyphIter {
        GlyphIter {
            glyphs: &self.glyphs,
            offset: Vector2F::zero(),
            ix: 0,
        }
    }
}

pub struct FragmentIter<'a> {
    fragments: &'a [Fragment],
    offset: Vector2F,
    ix: usize,
}

impl<'a> Iterator for FragmentIter<'a> {
    type Item = (&'a Fragment, Vector2F);

    fn next(&mut self) -> Option<(&'a Fragment, Vector2F)> {
        if self.ix == self.fragments.len() {
            None
        } else {
            let fragment = &self.fragments[self.ix];
            self.ix += 1;
            let offset = self.offset;
            self.offset += fragment.advance;
            Some((fragment, offset))
        }
    }
}

pub struct GlyphIter<'a> {
    glyphs: &'a [Glyph],
    offset: Vector2F,
    ix: usize,
}

impl<'a> Iterator for GlyphIter<'a> {
    type Item = (&'a Glyph, Vector2F);

    fn next(&mut self) -> Option<(&'a Glyph, Vector2F)> {
        if self.ix == self.glyphs.len() {
            None
        } else {
            let glyph = &self.glyphs[self.ix];
            self.ix += 1;
            Some((&glyph, self.offset + glyph.offset))
        }
    }
}
