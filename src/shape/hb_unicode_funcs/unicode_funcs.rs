//! Rust-native Unicode functions for harfbuzz

use std::ffi::c_void;
use std::ptr::null_mut;

use harfbuzz_sys::{
    hb_bool_t, hb_codepoint_t, hb_script_t,
    hb_unicode_combining_class_t, hb_unicode_funcs_create,
    hb_unicode_funcs_set_combining_class_func, hb_unicode_funcs_set_compose_func,
    hb_unicode_funcs_set_decompose_func, hb_unicode_funcs_set_mirroring_func,
    hb_unicode_funcs_set_script_func, hb_unicode_funcs_t,
};

use unicode_normalization::char::{canonical_combining_class, compose};

use crate::tables::{
    CANONICAL_DECOMP_KEY, CANONICAL_DECOMP_VAL, MIRROR_KEY, MIRROR_VAL,
};

use crate::script;

pub fn hb_unicode_funcs() -> *mut hb_unicode_funcs_t {
    // TODO: probably want to lazy static initialize this
    unsafe {
        let funcs_ptr = hb_unicode_funcs_create(null_mut());
        hb_unicode_funcs_set_combining_class_func(
            funcs_ptr,
            Some(unicode_combining_class),
            null_mut(),
            None,
        );
        hb_unicode_funcs_set_compose_func(funcs_ptr, Some(unicode_compose), null_mut(), None);
        hb_unicode_funcs_set_decompose_func(funcs_ptr, Some(unicode_decompose), null_mut(), None);
        hb_unicode_funcs_set_mirroring_func(funcs_ptr, Some(unicode_mirror), null_mut(), None);
        hb_unicode_funcs_set_script_func(funcs_ptr, Some(unicode_script), null_mut(), None);
        // hb_buffer_set_unicode_funcs(buffer.as_ptr(), funcs_ptr);
        funcs_ptr
    }
}

// fn make_unicode_funcs() -> *mut hb_unicode_funcs_t {
//     unsafe {
//         let funcs_ptr = hb_unicode_funcs_create(null_mut());
//         funcs_ptr
//     }
// }

// pub fn install_unicode_funcs(buffer: &mut Buffer) {
//     // TODO: probably want to lazy static initialize this
//     let funcs_ptr = make_unicode_funcs();
//     unsafe {
//         hb_unicode_funcs_set_combining_class_func(
//             funcs_ptr,
//             Some(unicode_combining_class),
//             null_mut(),
//             None,
//         );
//         hb_unicode_funcs_set_compose_func(funcs_ptr, Some(unicode_compose), null_mut(), None);
//         hb_unicode_funcs_set_decompose_func(funcs_ptr, Some(unicode_decompose), null_mut(), None);
//         hb_unicode_funcs_set_mirroring_func(funcs_ptr, Some(unicode_mirror), null_mut(), None);
//         hb_unicode_funcs_set_script_func(funcs_ptr, Some(unicode_script), null_mut(), None);
//         hb_buffer_set_unicode_funcs(buffer.as_ptr(), funcs_ptr);
//     }
// }

unsafe extern "C" fn unicode_combining_class(
    _ufuncs: *mut hb_unicode_funcs_t,
    unicode: hb_codepoint_t,
    _user_data: *mut c_void,
) -> hb_unicode_combining_class_t {
    // Will HarfBuzz ever give us invalid Unicode? I think no, but might be worth checking.
    let c = std::char::from_u32(unicode).unwrap();
    let class = canonical_combining_class(c);
    class.into()
}

unsafe extern "C" fn unicode_compose(
    _ufuncs: *mut hb_unicode_funcs_t,
    a: hb_codepoint_t,
    b: hb_codepoint_t,
    ab: *mut hb_codepoint_t,
    _user_data: *mut c_void,
) -> hb_bool_t {
    let a = std::char::from_u32(a).unwrap();
    let b = std::char::from_u32(b).unwrap();
    if let Some(result) = compose(a, b) {
        *ab = result.into();
        true.into()
    } else {
        false.into()
    }
}

const HANGUL_SYL_BASE: u32 = 0xAC00;
const HANGUL_SYL_COUNT: u32 = 11172;
const HANGUL_L_BASE: u32 = 0x1100;
const HANGUL_V_BASE: u32 = 0x1161;
const HANGUL_T_BASE: u32 = 0x11A7;
const HANGUL_V_COUNT: u32 = 21;
const HANGUL_T_COUNT: u32 = 28;
const HANGUL_N_COUNT: u32 = HANGUL_V_COUNT * HANGUL_T_COUNT;

unsafe extern "C" fn unicode_decompose(
    _ufuncs: *mut hb_unicode_funcs_t,
    ab: hb_codepoint_t,
    a: *mut hb_codepoint_t,
    b: *mut hb_codepoint_t,
    _user_data: *mut c_void,
) -> hb_bool_t {
    if ab >= HANGUL_SYL_BASE && ab < HANGUL_SYL_BASE + HANGUL_SYL_COUNT {
        // Decompose Hangul algorithmically.
        let syl = ab - HANGUL_SYL_BASE;
        let t = syl % HANGUL_T_COUNT;
        if t == 0 {
            // Decompose to L, V
            *a = HANGUL_L_BASE + syl / HANGUL_N_COUNT;
            *b = HANGUL_V_BASE + (syl % HANGUL_N_COUNT) / HANGUL_T_COUNT;
        } else {
            // Decompose to LV, T
            *a = ab - t;
            *b = HANGUL_T_BASE + t;
        }
        return true.into();
    }
    if let Ok(ix) = CANONICAL_DECOMP_KEY.binary_search(&ab.into()) {
        let (a_result, b_result) = CANONICAL_DECOMP_VAL[ix];
        *a = a_result;
        *b = b_result;
        true.into()
    } else {
        false.into()
    }
}

unsafe extern "C" fn unicode_script(
    _ufuncs: *mut hb_unicode_funcs_t,
    unicode: hb_codepoint_t,
    _user_data: *mut c_void,
) -> hb_script_t {
    script::lookup_script(unicode)
}

unsafe extern "C" fn unicode_mirror(
    _ufuncs: *mut hb_unicode_funcs_t,
    unicode: hb_codepoint_t,
    _user_data: *mut c_void,
) -> hb_codepoint_t {
    if let Ok(ix) = MIRROR_KEY.binary_search(&unicode) {
        MIRROR_VAL[ix]
    } else {
        0
    }
}
