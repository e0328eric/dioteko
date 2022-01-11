use crate::core::color::Color;
use crate::ffi;

pub struct Painter;

impl Painter {
    pub fn begin() -> Self {
        // SAFETY: ffi
        unsafe {
            ffi::BeginDrawing();
        }

        Self
    }

    pub fn clear_background(&self, color: Color) {
        // SAFETY: ffi
        unsafe { ffi::ClearBackground(color.into()) }
    }
}

impl Drop for Painter {
    fn drop(&mut self) {
        // SAFETY: ffi
        unsafe {
            ffi::EndDrawing();
        }
    }
}
