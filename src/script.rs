use std::cmp::Ordering;
use std::ops::Range;

use crate::tables::{SCRIPT_KEY, SCRIPT_VAL};

use harfbuzz::sys::{hb_script_t, HB_SCRIPT_COMMON, HB_SCRIPT_INHERITED, HB_SCRIPT_UNKNOWN};

/// Lookup the script property of a Codepoint.
///
/// The `hb_script_t` type is a big-endian encoding of the 4-byte string; this can also
/// be used for other purposes such as script matching during itemization.
///
/// Note that for unknown script, the unknown script value is returned ("Zzzz").
pub fn lookup_script(query: u32) -> hb_script_t {
    let pos = SCRIPT_KEY.binary_search_by(|&(s, e)| {
        if s > query {
            Ordering::Greater
        } else if e < query {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
    if let Ok(ix) = pos {
        SCRIPT_VAL[ix]
    } else {
        HB_SCRIPT_UNKNOWN
    }
}

pub struct Itemizer<'a> {
    text: &'a str,
    ix: usize,
}

impl<'a> Itemizer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text, ix: 0 }
    }
}

impl<'a> Iterator for Itemizer<'a> {
    type Item = (Range<usize>, hb_script_t);

    fn next(&mut self) -> Option<(Range<usize>, hb_script_t)> {
        let mut char_iter = self.text[self.ix..].chars();
        if let Some(cp) = char_iter.next() {
            let beg = self.ix;
            let mut current_script = lookup_script(cp.into());
            let mut len = cp.len_utf8();
            while let Some(cp) = char_iter.next() {
                let script = lookup_script(cp.into());
                if script != current_script {
                    if current_script == HB_SCRIPT_INHERITED || current_script == HB_SCRIPT_COMMON {
                        current_script = script;
                    } else if script != HB_SCRIPT_INHERITED && script != HB_SCRIPT_COMMON {
                        break;
                    }
                }
                len += cp.len_utf8();
            }
            if current_script == HB_SCRIPT_INHERITED {
                current_script = HB_SCRIPT_COMMON;
            }
            self.ix += len;
            Some((beg..self.ix, current_script))
        } else {
            None
        }
    }
}
