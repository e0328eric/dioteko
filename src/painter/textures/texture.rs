use std::cell::Cell;
use std::ptr::NonNull;

use crate::core::color::Color;
use crate::core::linalg::Vector2;
use crate::core::npatchinfo::NPatchInfo;
use crate::core::rectangle::Rectangle;
use crate::ffi;
use crate::painter::textures::image::Image;
use crate::painter::Painter;

// strong-weak tracker
pub(crate) struct TextureRc {
    strong: Cell<usize>,
    weak: Cell<usize>,
}

impl TextureRc {
    fn new() -> Self {
        Self {
            strong: Cell::new(1),
            weak: Cell::new(1),
        }
    }
}

trait TextureRcControll {
    fn get_strong(&self) -> &Cell<usize>;
    fn get_weak(&self) -> &Cell<usize>;

    #[inline]
    fn strong(&self) -> usize {
        self.get_strong().get()
    }

    #[inline]
    fn weak(&self) -> usize {
        self.get_weak().get()
    }

    #[inline]
    fn inc_strong(&self) {
        self.get_strong().set(self.strong() + 1);
    }

    #[inline]
    fn dec_strong(&self) {
        self.get_strong().set(self.strong() - 1);
    }

    #[inline]
    fn inc_weak(&self) {
        self.get_weak().set(self.weak() + 1);
    }

    #[inline]
    fn dec_weak(&self) {
        self.get_weak().set(self.weak() - 1);
    }
}

/// An owned Texture type
pub struct Texture {
    texture: ffi::Texture,
    rc_count: NonNull<TextureRc>,
}
pub type Texture2D = Texture;

// Implementation of Texture type
impl Texture {
    pub fn load(filename: impl AsRef<str>) -> Self {
        let filename = ffi::str_to_cstring(filename);

        // SAFETY: ffi
        Self {
            texture: unsafe { ffi::LoadTexture(filename.as_ptr()) },
            rc_count: Box::leak(Box::new(TextureRc::new())).into(),
        }
    }

    pub fn from_image(image: &Image) -> Self {
        // SAFETY: ffi
        // SAFETY: image.into_raw() is temporary, it is also safe
        Self {
            texture: unsafe { ffi::LoadTextureFromImage(image.take_raw()) },
            rc_count: Box::leak(Box::new(TextureRc::new())).into(),
        }
    }

    pub fn downgrade(val: &Self) -> WeakTexture {
        val.inc_weak();

        WeakTexture {
            texture: val.texture,
            rc_count: val.rc_count,
        }
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.texture.id
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.texture.width
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.texture.height
    }

    #[inline]
    pub fn mipmaps(&self) -> i32 {
        self.texture.mipmaps
    }

    #[inline]
    pub fn format(&self) -> i32 {
        self.texture.format
    }

    // This function marks as unsafe because the user can accidentally drop the second value.
    // In that case, converting ffi::Texture into original one is very unsafe
    #[allow(dead_code)]
    #[inline]
    pub(crate) unsafe fn into_raw(self) -> (ffi::Texture, NonNull<TextureRc>) {
        (self.texture, self.rc_count)
    }

    // This function marks as unsafe because rc_count may differ with the original data of the
    // texture's one
    #[allow(dead_code)]
    #[inline]
    pub(crate) unsafe fn from_raw(texture: ffi::Texture, rc_count: NonNull<TextureRc>) -> Self {
        Self { texture, rc_count }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.dec_strong();

        if self.strong() == 0 {
            // SAFETY: since we checked that there is no other owned inner texture,
            // dropping it is fine
            unsafe { ffi::UnloadTexture(self.texture) }

            self.dec_weak();
            if self.weak() == 0 {
                // SAFETY: Since that pointer was generated with Box::new, calling Box::from_raw
                // is safe
                drop(unsafe { Box::from_raw(self.rc_count.as_ptr()) })
            }
        }
    }
}

// A weak texture type
pub struct WeakTexture {
    texture: ffi::Texture,
    rc_count: NonNull<TextureRc>,
}
pub type WeakTexture2D = WeakTexture;

impl WeakTexture {
    pub fn upgrade(&self) -> Option<Texture> {
        if self.strong() != 0 {
            Some(Texture {
                texture: self.texture,
                rc_count: self.rc_count,
            })
        } else {
            None
        }
    }

    // This function marks as unsafe because the user can accidentally drop the second value.
    // In that case, converting ffi::Texture into original one is very unsafe
    #[allow(dead_code)]
    #[inline]
    pub(crate) unsafe fn into_raw(self) -> (ffi::Texture, NonNull<TextureRc>) {
        (self.texture, self.rc_count)
    }

    // This function marks as unsafe because rc_count may differ with the original data of the
    // texture's one
    #[allow(dead_code)]
    #[inline]
    pub(crate) unsafe fn from_raw(texture: ffi::Texture, rc_count: NonNull<TextureRc>) -> Self {
        Self { texture, rc_count }
    }
}

impl Clone for WeakTexture {
    fn clone(&self) -> Self {
        self.inc_weak();
        Self {
            texture: self.texture,
            rc_count: self.rc_count,
        }
    }
}

impl Drop for WeakTexture {
    fn drop(&mut self) {
        self.dec_weak();

        if self.weak() == 0 {
            // SAFETY: Since that pointer was generated with Box::new, calling Box::from_raw
            // is safe
            drop(unsafe { Box::from_raw(self.rc_count.as_ptr()) })
        }
    }
}

// Implement TextureRcControll for Texture and WeakTexture
impl TextureRcControll for Texture {
    fn get_strong(&self) -> &Cell<usize> {
        // SAFETY: reference is alive as long as the texture is alive
        unsafe { &self.rc_count.as_ref().strong }
    }

    fn get_weak(&self) -> &Cell<usize> {
        // SAFETY: reference is alive as long as the texture is alive
        unsafe { &self.rc_count.as_ref().weak }
    }
}

impl TextureRcControll for WeakTexture {
    fn get_strong(&self) -> &Cell<usize> {
        // SAFETY: reference is alive as long as the weak_texture is alive
        unsafe { &self.rc_count.as_ref().strong }
    }

    fn get_weak(&self) -> &Cell<usize> {
        // SAFETY: reference is alive as long as the weak_texture is alive
        unsafe { &self.rc_count.as_ref().weak }
    }
}

/// drawing related to texture
impl Painter {
    pub fn draw_texture(&self, texture: Texture, pos_x: i32, pos_y: i32, tint: Color) {
        // SAFETY: ffi
        unsafe { ffi::DrawTexture(texture.texture, pos_x, pos_y, tint.into()) }
    }

    pub fn draw_texture_v(&self, texture: Texture, position: Vector2, tint: Color) {
        // SAFETY: ffi
        unsafe { ffi::DrawTextureV(texture.texture, position.into(), tint.into()) }
    }

    pub fn draw_texture_ex(
        &self,
        texture: Texture,
        position: Vector2,
        rotation: f32,
        scale: f32,
        tint: Color,
    ) {
        // SAFETY: ffi
        unsafe {
            ffi::DrawTextureEx(
                texture.texture,
                position.into(),
                rotation,
                scale,
                tint.into(),
            )
        }
    }

    pub fn draw_texture_rec(
        &self,
        texture: Texture,
        source: Rectangle,
        position: Vector2,
        tint: Color,
    ) {
        // SAFETY: ffi
        unsafe { ffi::DrawTextureRec(texture.texture, source.into(), position.into(), tint.into()) }
    }

    pub fn draw_texture_pro(
        &self,
        texture: Texture,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        // SAFETY: ffi
        unsafe {
            ffi::DrawTexturePro(
                texture.texture,
                source.into(),
                dest.into(),
                origin.into(),
                rotation,
                tint.into(),
            )
        }
    }

    pub fn draw_texture_npatch(
        &self,
        texture: Texture,
        npatch_info: NPatchInfo,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        // SAFETY: ffi
        unsafe {
            ffi::DrawTextureNPatch(
                texture.texture,
                npatch_info.into(),
                dest.into(),
                origin.into(),
                rotation,
                tint.into(),
            )
        }
    }
}
