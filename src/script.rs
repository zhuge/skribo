use std::cmp::Ordering;

use crate::tables::{SCRIPT_KEY, SCRIPT_VAL};

use harfbuzz_sys::{hb_script_t, HB_SCRIPT_COMMON, HB_SCRIPT_INHERITED, HB_SCRIPT_UNKNOWN};

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

pub struct ScriptIter<'a> {
    text: &'a str,
}

impl<'a> ScriptIter<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl<'a> Iterator for ScriptIter<'a> {
    type Item = (hb_script_t, &'a str);

    fn next(&mut self) -> Option<(hb_script_t, &'a str)> {
        let mut char_iter = self.text.chars();
        if let Some(cp) = char_iter.next() {
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

            let substr = &self.text[..len];
            self.text = &self.text[len..];
            Some((current_script, substr))
        } else {
            None
        }
    }
}
