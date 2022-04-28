mod unicode_funcs;

use harfbuzz::{sys::hb_buffer_set_unicode_funcs, Buffer};

pub trait InstallUnicodeFunc {
    fn install_unicodd_funcs(&self);
}

impl InstallUnicodeFunc for Buffer {
    fn install_unicodd_funcs(&self) {
        let funcs = unicode_funcs::hb_unicode_funcs();
        unsafe {
            hb_buffer_set_unicode_funcs(self.as_ptr(), funcs);
        }
    }
}
