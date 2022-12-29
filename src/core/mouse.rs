use crate::core::linalg::Vector2;
use crate::ffi;

pub fn get_mouse_x() -> i32 {
    // SAFETY: ffi
    unsafe { ffi::GetMouseX() }
}

pub fn get_mouse_y() -> i32 {
    // SAFETY: ffi
    unsafe { ffi::GetMouseY() }
}

pub fn get_mouse_position() -> Vector2 {
    // SAFETY: ffi
    unsafe { ffi::GetMousePosition() }.into()
}

pub fn get_mouse_delta() -> Vector2 {
    // SAFETY: ffi
    unsafe { ffi::GetMouseDelta() }.into()
}

pub fn set_mouse_position(x: i32, y: i32) {
    // SAFETY: ffi
    unsafe { ffi::SetMousePosition(x, y) }
}

pub fn set_mouse_offset(offset_x: i32, offset_y: i32) {
    // SAFETY: ffi
    unsafe { ffi::SetMouseOffset(offset_x, offset_y) }
}

pub fn set_mouse_scale(scale_x: f32, scale_y: f32) {
    // SAFETY: ffi
    unsafe { ffi::SetMouseScale(scale_x, scale_y) }
}

pub fn get_mouse_wheel_move() -> f32 {
    // SAFETY: ffi
    unsafe { ffi::GetMouseWheelMove() }
}

impl_raylib_enum![
    MouseButton =>
    (Left, MouseButton_MOUSE_BUTTON_LEFT),
    (Right, MouseButton_MOUSE_BUTTON_RIGHT),
    (Middle, MouseButton_MOUSE_BUTTON_MIDDLE),
    (Side, MouseButton_MOUSE_BUTTON_SIDE),
    (Extra, MouseButton_MOUSE_BUTTON_EXTRA),
    (Forward, MouseButton_MOUSE_BUTTON_FORWARD),
    (Back, MouseButton_MOUSE_BUTTON_BACK),
];

impl MouseButton {
    pub fn is_pressed(self) -> bool {
        // SAFETY: ffi
        unsafe { to_bool!(ffi::IsMouseButtonPressed(self.into())) }
    }

    pub fn is_released(self) -> bool {
        // SAFETY: ffi
        unsafe { to_bool!(ffi::IsMouseButtonReleased(self.into())) }
    }

    pub fn is_up(self) -> bool {
        // SAFETY: ffi
        unsafe { to_bool!(ffi::IsMouseButtonUp(self.into())) }
    }

    pub fn is_down(self) -> bool {
        // SAFETY: ffi
        unsafe { to_bool!(ffi::IsMouseButtonDown(self.into())) }
    }
}

impl_raylib_enum![
    MouseCursor =>
    (Default, MouseCursor_MOUSE_CURSOR_DEFAULT),
    (Arrow, MouseCursor_MOUSE_CURSOR_ARROW),
    (Ibeam, MouseCursor_MOUSE_CURSOR_IBEAM),
    (CrossHair, MouseCursor_MOUSE_CURSOR_CROSSHAIR),
    (PointingHand, MouseCursor_MOUSE_CURSOR_POINTING_HAND),
    (ResizeEW, MouseCursor_MOUSE_CURSOR_RESIZE_EW),
    (ResizeNS, MouseCursor_MOUSE_CURSOR_RESIZE_NS),
    (ResizeNWSE, MouseCursor_MOUSE_CURSOR_RESIZE_NWSE),
    (ResizeNESW, MouseCursor_MOUSE_CURSOR_RESIZE_NESW),
    (ResizeAll, MouseCursor_MOUSE_CURSOR_RESIZE_ALL),
    (NotAllowed, MouseCursor_MOUSE_CURSOR_NOT_ALLOWED),
];
