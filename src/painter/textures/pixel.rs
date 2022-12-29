use std::ffi::c_void;

use crate::core::color::Color;
use crate::ffi;

pub unsafe fn get_pixel_color(src_ptr: *mut c_void, format: i32) -> Color {
    ffi::GetPixelColor(src_ptr, format).into()
}

pub unsafe fn set_pixel_color(dst_ptr: *mut c_void, color: Color, format: i32) {
    ffi::SetPixelColor(dst_ptr, color.into(), format)
}

pub fn get_pixel_data_size(width: i32, height: i32, format: i32) -> i32 {
    // SAFETY: ffi
    unsafe { ffi::GetPixelDataSize(width, height, format) }
}
