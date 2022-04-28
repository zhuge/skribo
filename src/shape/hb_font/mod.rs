#[cfg(all(
    any(target_os = "macos", target_os = "ios"),
    not(feature = "loader-freetype-default")
))]
mod coretext;

#[cfg(all(target_family = "windows", not(feature = "loader-freetype-default")))]
mod directwrite;

#[cfg(any(
    not(any(target_os = "macos", target_os = "ios", target_family = "windows")),
    feature = "loader-freetype-default"
))]
mod freetype;

#[cfg(all(
    any(target_os = "macos", target_os = "ios"),
    not(feature = "loader-freetype-default")
))]
pub use coretext::HbFont;

#[cfg(all(target_family = "windows", not(feature = "loader-freetype-default")))]
pub use directwrite::HbFont;

#[cfg(any(
    not(any(target_os = "macos", target_os = "ios", target_family = "windows")),
    feature = "loader-freetype-default"
))]
pub use self::freetype::HbFont;

use harfbuzz::sys::{hb_font_destroy, hb_font_reference};

impl Clone for HbFont {
    fn clone(&self) -> HbFont {
        unsafe {
            HbFont {
                hb_font: hb_font_reference(self.hb_font),
            }
        }
    }
}

impl Drop for HbFont {
    fn drop(&mut self) {
        unsafe {
            hb_font_destroy(self.hb_font);
        }
    }
}
