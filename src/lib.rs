pub mod core;
pub mod error;
pub mod prelude;
pub mod painter;

pub type Result<T> = crate::error::Result<T>;

mod ffi;
