/// Utility Module
///
/// This module contains three separate submodules: `time`, `random` and `misc`.
/// `time` module provides FPS and time related raylib functions.
/// `random` module gives an inner implemented random generator.
/// Finally, `misc` module gives miscellaneous functions
pub mod time {
    use crate::ffi;

    pub fn set_target_fps(fps: i32) {
        // SAFETY: ffi
        unsafe {
            ffi::SetTargetFPS(fps);
        }
    }

    pub fn get_fps() -> i32 {
        // SAFETY: ffi
        unsafe { ffi::GetFPS() }
    }

    pub fn get_frame_time() -> f64 {
        // SAFETY: ffi
        unsafe { ffi::GetFrameTime() as f64 }
    }

    pub fn get_time() -> f64 {
        // SAFETY: ffi
        unsafe { ffi::GetTime() }
    }
}

pub mod random {
    use crate::ffi;

    pub fn get_random_value(min: i32, max: i32) -> i32 {
        // SAFETY: ffi
        unsafe { ffi::GetRandomValue(min, max) }
    }

    pub fn set_random_seed(seed: u32) {
        // SAFETY: ffi
        unsafe { ffi::SetRandomSeed(seed) }
    }
}

pub mod misc {
    use crate::ffi;
    use std::ffi::CStr;

    pub fn take_screenshot(filename: &CStr) {
        // SAFETY: ffi
        unsafe {
            ffi::TakeScreenshot(filename.as_ptr());
        }
    }
}
