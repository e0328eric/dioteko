use crate::ffi;

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
