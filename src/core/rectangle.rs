use crate::ffi;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl From<ffi::Rectangle> for Rectangle {
    #[inline]
    fn from(rec: ffi::Rectangle) -> Self {
        Self {
            x: rec.x,
            y: rec.y,
            width: rec.width,
            height: rec.height,
        }
    }
}

impl From<Rectangle> for ffi::Rectangle {
    #[inline]
    fn from(rec: Rectangle) -> Self {
        Self {
            x: rec.x,
            y: rec.y,
            width: rec.width,
            height: rec.height,
        }
    }
}
