use crate::core::linalg::{Vector3, Vector4};
use crate::ffi;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl From<ffi::Color> for Color {
    #[inline]
    fn from(color: ffi::Color) -> Self {
        Self(color.r, color.g, color.b, color.a)
    }
}

impl From<Color> for ffi::Color {
    #[inline]
    fn from(color: Color) -> ffi::Color {
        ffi::Color {
            r: color.0,
            g: color.1,
            b: color.2,
            a: color.3,
        }
    }
}

impl Color {
    pub fn fade(self, alpha: f32) -> Self {
        // SAFETY: ffi
        unsafe { ffi::Fade(self.into(), alpha) }.into()
    }

    pub fn to_int(self) -> i32 {
        // SAFETY: ffi
        unsafe { ffi::ColorToInt(self.into()) }
    }

    pub fn normalize(self) -> Vector4 {
        // SAFETY: ffi
        unsafe { ffi::ColorNormalize(self.into()) }.into()
    }

    pub fn from_normalized(normalized: Vector4) -> Self {
        // SAFETY: ffi
        unsafe { ffi::ColorFromNormalized(normalized.into()) }.into()
    }

    pub fn to_hsv(self) -> Vector3 {
        // SAFETY: ffi
        unsafe { ffi::ColorToHSV(self.into()) }.into()
    }

    pub fn from_hsv(hue: f32, saturation: f32, value: f32) -> Self {
        // SAFETY: ffi
        unsafe { ffi::ColorFromHSV(hue, saturation, value) }.into()
    }

    pub fn alpha(self, alpha: f32) -> Self {
        // SAFETY: ffi
        unsafe { ffi::ColorAlpha(self.into(), alpha) }.into()
    }

    pub fn alpha_blend(dst: Self, src: Self, tint: Self) -> Self {
        // SAFETY: ffi
        unsafe { ffi::ColorAlphaBlend(dst.into(), src.into(), tint.into()) }.into()
    }

    pub fn get_color(hex_value: u32) -> Self {
        // SAFETY: ffi
        unsafe { ffi::GetColor(hex_value) }.into()
    }
}

// Predefined Colors
pub const LIGHTGRAY: Color = Color(200, 200, 200, 255); // Light Gray
pub const GRAY: Color = Color(130, 130, 130, 255); // Gray
pub const DARKGRAY: Color = Color(80, 80, 80, 255); // Dark Gray
pub const YELLOW: Color = Color(253, 249, 0, 255); // Yellow
pub const GOLD: Color = Color(255, 203, 0, 255); // Gold
pub const ORANGE: Color = Color(255, 161, 0, 255); // Orange
pub const PINK: Color = Color(255, 109, 194, 255); // Pink
pub const RED: Color = Color(230, 41, 55, 255); // Red
pub const MAROON: Color = Color(190, 33, 55, 255); // Maroon
pub const GREEN: Color = Color(0, 228, 48, 255); // Green
pub const LIME: Color = Color(0, 158, 47, 255); // Lime
pub const DARKGREEN: Color = Color(0, 117, 44, 255); // Dark Green
pub const SKYBLUE: Color = Color(102, 191, 255, 255); // Sky Blue
pub const BLUE: Color = Color(0, 121, 241, 255); // Blue
pub const DARKBLUE: Color = Color(0, 82, 172, 255); // Dark Blue
pub const PURPLE: Color = Color(200, 122, 255, 255); // Purple
pub const VIOLET: Color = Color(135, 60, 190, 255); // Violet
pub const DARKPURPLE: Color = Color(112, 31, 126, 255); // Dark Purple
pub const BEIGE: Color = Color(211, 176, 131, 255); // Beige
pub const BROWN: Color = Color(127, 106, 79, 255); // Brown
pub const DARKBROWN: Color = Color(76, 63, 47, 255); // Dark Brown

pub const WHITE: Color = Color(255, 255, 255, 255); // White
pub const BLACK: Color = Color(0, 0, 0, 255); // Black
pub const BLANK: Color = Color(0, 0, 0, 0); // Transparent
pub const MAGENTA: Color = Color(255, 0, 255, 255); // Magenta
pub const RAYWHITE: Color = Color(245, 245, 245, 255); // Ray White
