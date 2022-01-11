use crate::core::rectangle::Rectangle;
use crate::ffi;

#[derive(Debug, Clone, Copy)]
pub struct NPatchInfo {
    pub source: Rectangle,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub layout: i32,
}

impl From<ffi::NPatchInfo> for NPatchInfo {
    #[inline]
    fn from(info: ffi::NPatchInfo) -> Self {
        Self {
            source: info.source.into(),
            left: info.left,
            top: info.top,
            right: info.right,
            bottom: info.bottom,
            layout: info.layout,
        }
    }
}

impl From<NPatchInfo> for ffi::NPatchInfo {
    #[inline]
    fn from(info: NPatchInfo) -> Self {
        Self {
            source: info.source.into(),
            left: info.left,
            top: info.top,
            right: info.right,
            bottom: info.bottom,
            layout: info.layout,
        }
    }
}
