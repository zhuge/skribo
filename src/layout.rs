//! Retained layout that supports substring queries.

use harfbuzz::sys::hb_script_t;
use harfbuzz::{Direction, Language};

use pathfinder_geometry::vector::Vector2F;

use crate::script::ScriptIter;
use crate::shape::shape;
use crate::{FontCollection, FontRef, TextStyle};

#[allow(unused)]
pub struct Layout<S: AsRef<str>> {
    text: S,
    style: TextStyle,
    fragments: Vec<Fragment>,
}

#[allow(unused)]
pub(crate) struct Fragment {
    // Length of substring covered by this fragment.
    pub(crate) substr_len: usize,
    pub(crate) script: hb_script_t,
    pub(crate) language: Language,
    pub(crate) direction: Direction,
    pub(crate) advance: Vector2F,
    pub(crate) glyphs: Vec<Glyph>,
    pub(crate) font: FontRef,
}

// This should probably be renamed "glyph".
//
// Discussion topic: this is so similar to hb_glyph_info_t, maybe we
// should just use that.
#[allow(unused)]
pub struct Glyph {
    pub cluster: u32,
    pub glyph_id: u32,
    pub offset: Vector2F,
    pub advance: Vector2F,
    pub unsafe_to_break: bool,
}

pub struct LayoutIter<'a> {
    fragments: &'a [Fragment],
    offset: Vector2F,
    fragment_ix: usize,
}

pub struct LayoutItem<'a> {
    pub offset: Vector2F,
    fragment: &'a Fragment,
}

pub struct FragmentIter<'a> {
    offset: Vector2F,
    fragment: &'a Fragment,
    glyph_ix: usize,
}

pub struct FragmentItem<'a> {
    pub offset: Vector2F,
    pub glyph: &'a Glyph,
}

impl<S: AsRef<str>> Layout<S> {
    pub fn create(text: S, style: &TextStyle, collection: &FontCollection) -> Layout<S> {
        let mut fragments = Vec::new();
        for (script, substr) in ScriptIter::new(&text.as_ref()[..]) {
            for (range, font) in collection.itemize(substr) {
                let fragment = shape(style, font, script, &substr[range]);
                fragments.push(fragment);
            }
        }

        Layout {
            text,
            // Does this clone mean we should take style arg by-move?
            style: style.clone(),
            fragments,
        }
    }

    /// Iterate through all glyphs in the layout.
    pub fn fragments(&self) -> LayoutIter {
        LayoutIter {
            offset: Vector2F::zero(),
            fragments: &self.fragments,
            fragment_ix: 0,
        }
    }
}

impl<'a> Iterator for LayoutIter<'a> {
    type Item = LayoutItem<'a>;

    fn next(&mut self) -> Option<LayoutItem<'a>> {
        if self.fragment_ix == self.fragments.len() {
            None
        } else {
            let fragment = &self.fragments[self.fragment_ix];
            self.fragment_ix += 1;
            let offset = self.offset;
            self.offset += fragment.advance;
            Some(LayoutItem { offset, fragment })
        }
    }
}

impl<'a> LayoutItem<'a> {
    pub fn font(&self) -> &FontRef {
        &self.fragment.font
    }

    pub fn script(&self) -> hb_script_t {
        self.fragment.script
    }

    pub fn direction(&self) -> Direction {
        self.fragment.direction
    }

    pub fn language(&self) -> Language {
        self.fragment.language
    }

    pub fn glyphs(&self) -> FragmentIter<'a> {
        FragmentIter {
            offset: self.offset,
            fragment: self.fragment,
            glyph_ix: 0,
        }
    }
}

impl<'a> Iterator for FragmentIter<'a> {
    type Item = FragmentItem<'a>;

    fn next(&mut self) -> Option<FragmentItem<'a>> {
        if self.glyph_ix == self.fragment.glyphs.len() {
            None
        } else {
            let glyph = &self.fragment.glyphs[self.glyph_ix];
            self.glyph_ix += 1;
            Some(FragmentItem {
                glyph,
                offset: self.offset + glyph.offset,
            })
        }
    }
}
