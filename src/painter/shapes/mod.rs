use crate::core::color::*;
use crate::core::linalg::*;
use crate::ffi;

pub fn draw_circle_v(center: Vector2, radius: f32, color: Color) {
    // SAFETY: ffi
    unsafe { ffi::DrawCircleV(center.into(), radius, color.into()) }
}
