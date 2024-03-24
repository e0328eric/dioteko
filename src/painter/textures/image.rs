use std::ffi::CStr;

use crate::ffi;
use crate::painter::textures::texture::Texture;

#[repr(transparent)]
pub struct Image {
    image: ffi::Image,
}

impl Image {
    pub fn load_image(filename: &CStr) -> Self {
        // SAFETY: ffi
        // SAFETY: Since ffi::LoadImage makes a temporary ffi::Image and
        // ffi::Image has no destructor, making Image with from_raw satisfies
        // all conditions of safety.
        unsafe { Image::from_raw(ffi::LoadImage(filename.as_ptr())) }
    }

    pub fn load_image_raw(
        filename: &CStr,
        width: i32,
        height: i32,
        format: i32,
        header_size: i32,
    ) -> Self {
        // SAFETY: ffi
        // SAFETY: Since ffi::LoadImageRaw makes a temporary ffi::Image and
        // ffi::Image has no destructor, making Image with from_raw satisfies
        // all conditions of safety.
        unsafe {
            Image::from_raw(ffi::LoadImageRaw(
                filename.as_ptr(),
                width,
                height,
                format,
                header_size,
            ))
        }
    }

    pub fn load_image_from_memory(filename: &CStr, file_data: &[u8]) -> Self {
        // SAFETY: ffi
        // SAFETY: Since ffi::LoadImageFromMemory makes a temporary ffi::Image and
        // ffi::Image has no destructor, making Image with from_raw satisfies
        // all conditions of safety.
        unsafe {
            Image::from_raw(ffi::LoadImageFromMemory(
                filename.as_ptr(),
                file_data.as_ptr(),
                file_data.len() as i32,
            ))
        }
    }

    pub fn load_image_from_texture(texture: Texture) -> Self {
        // SAFETY: ffi
        // SAFETY: Since ffi::LoadImageFromTexture makes a temporary ffi::Image and
        // ffi::Image has no destructor, making Image with from_raw satisfies
        // all conditions of safety.
        // SAFETY: since we do not convert back to the Texture, ignoring the second
        // value of the into_raw is fine
        unsafe { Image::from_raw(ffi::LoadImageFromTexture(texture.into_raw().0)) }
    }

    pub fn load_image_from_screen() -> Self {
        // SAFETY: ffi
        // SAFETY: Since ffi::LoadImageFromScreen makes a temporary ffi::Image and
        // ffi::Image has no destructor, making Image with from_raw satisfies
        // all conditions of safety.
        unsafe { Image::from_raw(ffi::LoadImageFromScreen()) }
    }

    /// This function is unsafe because it takes a raw pointer as its parameter
    pub unsafe fn load_image_anim(filename: &CStr, frames: *mut i32) -> Self {
        // SAFETY: ffi
        // SAFETY: Since ffi::LoadImageAnim makes a temporary ffi::Image and
        // ffi::Image has no destructor, making Image with from_raw satisfies
        // all conditions of safety.
        Image::from_raw(ffi::LoadImageAnim(filename.as_ptr(), frames))
    }

    /// Convert ffi::Image structure into Rust's one.
    /// This function is unsafe because Image has a destructor which does not exist in C.
    /// If there is one ffi::Image that makes two distinct Image, it can be happen that
    /// the destructor of Image is called more than twice, cause a double free.
    ///
    /// ## Safety
    /// - There is no manually called 'UnloadImage' with ffi::Image.
    /// - `from_raw` must call once for one ffi::Image.
    #[inline]
    pub(crate) unsafe fn from_raw(image: ffi::Image) -> Self {
        Self { image }
    }

    /// Takes the inner value
    /// This is unsafe because after making ffi::Image, user can never drop the
    /// ffi::Image.
    ///
    /// ## Safety
    /// - After calling this, one should manually drop the output or convert back to
    ///   the Rust's Image.
    #[inline]
    pub(crate) unsafe fn take_raw(&self) -> ffi::Image {
        self.image
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        // SAFETY: ffi
        // SAFETY: ffi::ImageCopy makes a deep copy of ffi::Image.
        // In addition, the obtained ffi::Image is temporary value
        // for which no destructor of ffi::Image is called.
        unsafe { Image::from_raw(ffi::ImageCopy(self.image)) }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        // SAFETY: ffi
        // SAFETY: Since this is a destructor, into_raw is fine
        unsafe { ffi::UnloadImage(self.image) }
    }
}
