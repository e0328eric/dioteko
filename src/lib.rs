pub mod core;
pub mod error;
pub mod prelude;
pub mod shapes;
pub mod textures;

pub type Result<T> = crate::error::Result<T>;

mod ffi;
