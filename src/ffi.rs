use std::ffi::CString;

pub(crate) use dioteko_raylib_sys::ffi::*;

pub(crate) fn str_to_cstring(s: impl AsRef<str>) -> CString {
    CString::new(s.as_ref()).expect("[INTERNAL ERROR]: Cannot cast str into cstring")
}
