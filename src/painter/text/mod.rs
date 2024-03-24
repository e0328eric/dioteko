pub mod font;

use std::ffi::CStr;

use super::Painter;
use crate::core::color::Color;
use crate::ffi;

impl Painter {
    pub fn draw_text(&self, text: &CStr, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
        // SAFETY: ffi
        unsafe { ffi::DrawText(text.as_ptr(), pos_x, pos_y, font_size, color.into()) }
    }
}
