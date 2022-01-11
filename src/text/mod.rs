pub mod font;

use std::ffi::CString;

use crate::core::color::Color;
use crate::ffi;

pub fn draw_text(text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
    let text = CString::new(text).expect("[INTERNAL ERROR]: Cannot cast str into cstring");

    // SAFETY: ffi
    unsafe { ffi::DrawText(text.as_ptr(), pos_x, pos_y, font_size, color.into()) }
}
