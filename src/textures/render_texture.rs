use std::cell::Cell;
use std::ptr::NonNull;

use crate::ffi;

// strong-weak tracker
pub(crate) struct RenderTextureRc {
    strong: Cell<usize>,
    weak: Cell<usize>,
}

impl RenderTextureRc {
    fn new() -> Self {
        Self {
            strong: Cell::new(1),
            weak: Cell::new(1),
        }
    }
}

trait RenderTextureRcControll {
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
pub struct RenderTexture {
    render_texture: ffi::RenderTexture,
    rc_count: NonNull<RenderTextureRc>,
}
pub type RenderTexture2D = RenderTexture;

// Implementation of Texture type
impl RenderTexture {
    pub fn load(width: i32, height: i32) -> Self {
        // SAFETY: ffi
        Self {
            render_texture: unsafe { ffi::LoadRenderTexture(width, height) },
            rc_count: Box::leak(Box::new(RenderTextureRc::new())).into(),
        }
    }

    pub fn downgrade(val: &Self) -> WeakRenderTexture {
        val.inc_weak();

        WeakRenderTexture {
            render_texture: val.render_texture,
            rc_count: val.rc_count,
        }
    }

    // This function marks as unsafe because the user can accidentally drop the second value.
    // In that case, converting ffi::Texture into original one is very unsafe
    #[allow(dead_code)]
    #[inline]
    pub(crate) unsafe fn into_raw(self) -> (ffi::RenderTexture, NonNull<RenderTextureRc>) {
        (self.render_texture, self.rc_count)
    }

    // This function marks as unsafe because rc_count may differ with the original data of the
    // texture's one
    #[allow(dead_code)]
    #[inline]
    pub(crate) unsafe fn from_raw(
        render_texture: ffi::RenderTexture,
        rc_count: NonNull<RenderTextureRc>,
    ) -> Self {
        Self {
            render_texture,
            rc_count,
        }
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        self.dec_strong();

        if self.strong() == 0 {
            // SAFETY: since we checked that there is no other owned inner render texture,
            // dropping it is fine
            unsafe { ffi::UnloadRenderTexture(self.render_texture) }

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
pub struct WeakRenderTexture {
    render_texture: ffi::RenderTexture,
    rc_count: NonNull<RenderTextureRc>,
}
pub type WeakRenderTexture2D = WeakRenderTexture;

impl WeakRenderTexture {
    pub fn upgrade(&self) -> Option<RenderTexture> {
        if self.strong() != 0 {
            Some(RenderTexture {
                render_texture: self.render_texture,
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
    pub(crate) unsafe fn into_raw(self) -> (ffi::RenderTexture, NonNull<RenderTextureRc>) {
        (self.render_texture, self.rc_count)
    }

    // This function marks as unsafe because rc_count may differ with the original data of the
    // render texture's one
    #[allow(dead_code)]
    #[inline]
    pub(crate) unsafe fn from_raw(
        render_texture: ffi::RenderTexture,
        rc_count: NonNull<RenderTextureRc>,
    ) -> Self {
        Self {
            render_texture,
            rc_count,
        }
    }
}

impl Clone for WeakRenderTexture {
    fn clone(&self) -> Self {
        self.inc_weak();
        Self {
            render_texture: self.render_texture,
            rc_count: self.rc_count,
        }
    }
}

impl Drop for WeakRenderTexture {
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
impl RenderTextureRcControll for RenderTexture {
    fn get_strong(&self) -> &Cell<usize> {
        // SAFETY: reference is alive as long as the texture is alive
        unsafe { &self.rc_count.as_ref().strong }
    }

    fn get_weak(&self) -> &Cell<usize> {
        // SAFETY: reference is alive as long as the texture is alive
        unsafe { &self.rc_count.as_ref().weak }
    }
}

impl RenderTextureRcControll for WeakRenderTexture {
    fn get_strong(&self) -> &Cell<usize> {
        // SAFETY: reference is alive as long as the weak_texture is alive
        unsafe { &self.rc_count.as_ref().strong }
    }

    fn get_weak(&self) -> &Cell<usize> {
        // SAFETY: reference is alive as long as the weak_texture is alive
        unsafe { &self.rc_count.as_ref().weak }
    }
}
