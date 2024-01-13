pub mod shapes;
pub mod text;
pub mod textures;

use std::marker::PhantomData;

use crate::core::color::Color;
use crate::core::linalg::Vector2;
use crate::ffi;

pub struct Painter {
    _marker: PhantomData<*mut u8>, // to make Painter !Sync and !Send
}

/// basic drawings
impl Painter {
    pub fn new() -> Self {
        // SAFETY: ffi
        unsafe {
            ffi::BeginDrawing();
        }

        Self {
            _marker: PhantomData,
        }
    }

    pub fn clear_background(&self, color: Color) {
        // SAFETY: ffi
        unsafe { ffi::ClearBackground(color.into()) }
    }

    // Drawing Circle
    pub fn draw_circle_v(&self, center: Vector2, radius: f32, color: Color) {
        // SAFETY: ffi
        unsafe { ffi::DrawCircleV(center.into(), radius, color.into()) }
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
