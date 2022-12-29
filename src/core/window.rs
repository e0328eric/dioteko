use std::cell::Cell;
use std::ffi::CString;

use crate::core::keyboard::Key;
use crate::error::{self, DiotekoErr};
use crate::ffi;
use crate::painter::textures::image;

pub struct WindowBuilder<'s> {
    width: usize,
    height: usize,
    title: &'s str,
    icon: Option<&'s image::Image>,
    exit_key: Option<Key>,
    #[cfg(target_os = "windows")]
    config_flags: i32,
    #[cfg(not(target_os = "windows"))]
    config_flags: u32,
}

#[allow(dead_code)]
pub struct Window {
    width: usize,
    height: usize,
    title: CString,
    #[cfg(target_os = "windows")]
    config_flags: Cell<i32>,
    #[cfg(not(target_os = "windows"))]
    config_flags: Cell<u32>,
}

impl<'s> WindowBuilder<'s> {
    pub fn new(width: usize, height: usize, title: &'s str) -> Self {
        Self {
            width,
            height,
            title,
            icon: None,
            exit_key: None,
            config_flags: 0,
        }
    }

    /// Initialize Window and OpenGL context with given configs
    pub fn build(&self) -> error::Result<Window> {
        let title = ffi::str_to_cstring(self.title);

        // SAFETY: ffi
        unsafe {
            // Initialize Window
            ffi::InitWindow(self.width as i32, self.height as i32, title.as_ptr());

            // Sets Window State
            #[cfg(target_os = "windows")]
            ffi::SetWindowState(self.config_flags as u32);
            #[cfg(not(target_os = "windows"))]
            ffi::SetWindowState(self.config_flags);

            // Check the window is well-initialized
            if !to_bool!(ffi::IsWindowReady()) {
                return Err(DiotekoErr::WindowInitFailedErr);
            }

            // Sets window icon
            if let Some(icon) = self.icon {
                // SAFETY: icon.take_raw() makes a temporary data
                ffi::SetWindowIcon(icon.take_raw())
            }

            // Sets exit key
            if let Some(exit_key) = self.exit_key {
                ffi::SetExitKey(exit_key.into());
            }
        }

        Ok(Window {
            width: self.width,
            height: self.height,
            title,
            config_flags: Cell::new(self.config_flags),
        })
    }

    #[inline]
    pub fn set_window_icon(mut self, icon: &'s image::Image) -> Self {
        self.icon = Some(icon);
        self
    }

    #[inline]
    pub fn set_exit_key(mut self, key: Key) -> Self {
        self.exit_key = Some(key);
        self
    }

    #[inline]
    pub fn with_vsync_hint(mut self) -> Self {
        self.config_flags |= ffi::ConfigFlags_FLAG_VSYNC_HINT;
        self
    }

    #[inline]
    pub fn with_fullscreen(mut self) -> Self {
        self.config_flags |= ffi::ConfigFlags_FLAG_FULLSCREEN_MODE;
        self
    }

    #[inline]
    pub fn with_resizable(mut self) -> Self {
        self.config_flags |= ffi::ConfigFlags_FLAG_WINDOW_RESIZABLE;
        self
    }

    #[inline]
    pub fn with_undecorated(mut self) -> Self {
        self.config_flags |= ffi::ConfigFlags_FLAG_WINDOW_UNDECORATED;
        self
    }

    #[inline]
    pub fn with_hidden(mut self) -> Self {
        self.config_flags |= ffi::ConfigFlags_FLAG_WINDOW_HIDDEN;
        self
    }

    #[inline]
    pub fn with_minimized(mut self) -> Self {
        self.config_flags |= ffi::ConfigFlags_FLAG_WINDOW_MINIMIZED;
        self
    }

    #[inline]
    pub fn with_maximized(mut self) -> Self {
        self.config_flags |= ffi::ConfigFlags_FLAG_WINDOW_MAXIMIZED;
        self
    }

    #[inline]
    pub fn with_unfocused(mut self) -> Self {
        self.config_flags |= ffi::ConfigFlags_FLAG_WINDOW_UNFOCUSED;
        self
    }

    #[inline]
    pub fn with_topmost(mut self) -> Self {
        self.config_flags |= ffi::ConfigFlags_FLAG_WINDOW_TOPMOST;
        self
    }

    #[inline]
    pub fn with_always_run(mut self) -> Self {
        self.config_flags |= ffi::ConfigFlags_FLAG_WINDOW_ALWAYS_RUN;
        self
    }

    #[inline]
    pub fn with_transparent(mut self) -> Self {
        self.config_flags |= ffi::ConfigFlags_FLAG_WINDOW_TRANSPARENT;
        self
    }

    #[inline]
    pub fn with_high_dpi(mut self) -> Self {
        self.config_flags |= ffi::ConfigFlags_FLAG_WINDOW_HIGHDPI;
        self
    }

    #[inline]
    pub fn with_msaa_4x(mut self) -> Self {
        self.config_flags |= ffi::ConfigFlags_FLAG_MSAA_4X_HINT;
        self
    }

    #[inline]
    pub fn with_interlaced_hint(mut self) -> Self {
        self.config_flags |= ffi::ConfigFlags_FLAG_INTERLACED_HINT;
        self
    }
}

impl Window {
    #[inline]
    pub fn get_width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn should_close(&self) -> bool {
        // SAFETY: ffi
        unsafe { to_bool!(ffi::WindowShouldClose()) }
    }

    pub fn is_window_ready(&self) -> bool {
        // SAFETY: ffi
        unsafe { to_bool!(ffi::IsWindowReady()) }
    }

    pub fn is_window_fullscreen(&self) -> bool {
        // SAFETY: ffi
        unsafe { to_bool!(ffi::IsWindowFullscreen()) }
    }

    pub fn is_window_hidden(&self) -> bool {
        // SAFETY: ffi
        unsafe { to_bool!(ffi::IsWindowHidden()) }
    }

    pub fn is_window_minimized(&self) -> bool {
        // SAFETY: ffi
        unsafe { to_bool!(ffi::IsWindowMinimized()) }
    }

    pub fn is_window_maximized(&self) -> bool {
        // SAFETY: ffi
        unsafe { to_bool!(ffi::IsWindowMaximized()) }
    }

    pub fn is_window_focused(&self) -> bool {
        // SAFETY: ffi
        unsafe { to_bool!(ffi::IsWindowFocused()) }
    }

    pub fn is_window_resized(&self) -> bool {
        // SAFETY: ffi
        unsafe { to_bool!(ffi::IsWindowResized()) }
    }

    pub fn toogle_fullscreen(&self) {
        // Updates config_flag
        let mut config_flags = self.config_flags.get();
        config_flags |= ffi::ConfigFlags_FLAG_FULLSCREEN_MODE;
        self.config_flags.set(config_flags);

        // SAFETY: ffi
        unsafe {
            #[cfg(target_os = "windows")]
            ffi::SetWindowState(config_flags as u32);
            #[cfg(not(target_os = "windows"))]
            ffi::SetWindowState(config_flags);
        }
    }

    pub fn maximize_window(&self) {
        // Updates config_flag
        let mut config_flags = self.config_flags.get();
        config_flags |= ffi::ConfigFlags_FLAG_WINDOW_MAXIMIZED;
        self.config_flags.set(config_flags);

        // SAFETY: ffi
        unsafe {
            #[cfg(target_os = "windows")]
            ffi::SetWindowState(config_flags as u32);
            #[cfg(not(target_os = "windows"))]
            ffi::SetWindowState(config_flags);
        }
    }

    pub fn minimize_window(&self) {
        // Updates config_flag
        let mut config_flags = self.config_flags.get();
        config_flags |= ffi::ConfigFlags_FLAG_WINDOW_MINIMIZED;
        self.config_flags.set(config_flags);

        // SAFETY: ffi
        unsafe {
            #[cfg(target_os = "windows")]
            ffi::SetWindowState(config_flags as u32);
            #[cfg(not(target_os = "windows"))]
            ffi::SetWindowState(config_flags);
        }
    }

    pub fn restore_window(&self) {
        // Updates config_flag
        let mut config_flags = self.config_flags.get();
        config_flags &=
            !(ffi::ConfigFlags_FLAG_WINDOW_MAXIMIZED | ffi::ConfigFlags_FLAG_WINDOW_MINIMIZED);
        self.config_flags.set(config_flags);

        // SAFETY: ffi
        unsafe {
            #[cfg(target_os = "windows")]
            ffi::SetWindowState(config_flags as u32);
            #[cfg(not(target_os = "windows"))]
            ffi::SetWindowState(config_flags);
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        // SAFETY: ffi
        unsafe {
            ffi::CloseWindow();
        }
    }
}
