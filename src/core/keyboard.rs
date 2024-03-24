use crate::ffi;

// Key enum implementation
//
// Actual Key enum is defined with the impl_raylib_enum macro

impl Key {
    pub fn is_pressed(self) -> bool {
        // SAFETY: ffi
        unsafe { to_bool!(ffi::IsKeyPressed(self.into())) }
    }

    pub fn is_down(self) -> bool {
        // SAFETY: ffi
        unsafe { to_bool!(ffi::IsKeyDown(self.into())) }
    }

    pub fn is_released(self) -> bool {
        // SAFETY: ffi
        unsafe { to_bool!(ffi::IsKeyReleased(self.into())) }
    }

    pub fn is_up(self) -> bool {
        // SAFETY: ffi
        unsafe { to_bool!(ffi::IsKeyUp(self.into())) }
    }

    pub fn get_key_pressed() -> Self {
        // SAFETY: ffi
        unsafe { ffi::GetKeyPressed().into() }
    }

    pub fn get_char_pressed() -> Option<char> {
        // SAFETY: ffi
        unsafe { char::from_u32(ffi::GetCharPressed() as u32) }
    }
}

impl_raylib_enum![
    Key =>
    (Null        , KeyboardKey_KEY_NULL),
    (Apostrophe  , KeyboardKey_KEY_APOSTROPHE),
    (Comma       , KeyboardKey_KEY_COMMA),
    (Minus       , KeyboardKey_KEY_MINUS),
    (Period      , KeyboardKey_KEY_PERIOD),
    (Slash       , KeyboardKey_KEY_SLASH),
    (Zero        , KeyboardKey_KEY_ZERO),
    (One         , KeyboardKey_KEY_ONE),
    (Two         , KeyboardKey_KEY_TWO),
    (Three       , KeyboardKey_KEY_THREE),
    (Four        , KeyboardKey_KEY_FOUR),
    (Five        , KeyboardKey_KEY_FIVE),
    (Six         , KeyboardKey_KEY_SIX),
    (Seven       , KeyboardKey_KEY_SEVEN),
    (Eight       , KeyboardKey_KEY_EIGHT),
    (Nine        , KeyboardKey_KEY_NINE),
    (Semicolon   , KeyboardKey_KEY_SEMICOLON),
    (Equal       , KeyboardKey_KEY_EQUAL),
    (A           , KeyboardKey_KEY_A),
    (B           , KeyboardKey_KEY_B),
    (C           , KeyboardKey_KEY_C),
    (D           , KeyboardKey_KEY_D),
    (E           , KeyboardKey_KEY_E),
    (F           , KeyboardKey_KEY_F),
    (G           , KeyboardKey_KEY_G),
    (H           , KeyboardKey_KEY_H),
    (I           , KeyboardKey_KEY_I),
    (J           , KeyboardKey_KEY_J),
    (K           , KeyboardKey_KEY_K),
    (L           , KeyboardKey_KEY_L),
    (M           , KeyboardKey_KEY_M),
    (N           , KeyboardKey_KEY_N),
    (O           , KeyboardKey_KEY_O),
    (P           , KeyboardKey_KEY_P),
    (Q           , KeyboardKey_KEY_Q),
    (R           , KeyboardKey_KEY_R),
    (S           , KeyboardKey_KEY_S),
    (T           , KeyboardKey_KEY_T),
    (U           , KeyboardKey_KEY_U),
    (V           , KeyboardKey_KEY_V),
    (W           , KeyboardKey_KEY_W),
    (X           , KeyboardKey_KEY_X),
    (Y           , KeyboardKey_KEY_Y),
    (Z           , KeyboardKey_KEY_Z),
    (LeftBracket , KeyboardKey_KEY_LEFT_BRACKET),
    (Backslash   , KeyboardKey_KEY_BACKSLASH),
    (RightBracket, KeyboardKey_KEY_RIGHT_BRACKET),
    (Grave       , KeyboardKey_KEY_GRAVE),
    (Space       , KeyboardKey_KEY_SPACE),
    (Escape      , KeyboardKey_KEY_ESCAPE),
    (Enter       , KeyboardKey_KEY_ENTER),
    (Tab         , KeyboardKey_KEY_TAB),
    (Backspace   , KeyboardKey_KEY_BACKSPACE),
    (Insert      , KeyboardKey_KEY_INSERT),
    (Delete      , KeyboardKey_KEY_DELETE),
    (Right       , KeyboardKey_KEY_RIGHT),
    (Left        , KeyboardKey_KEY_LEFT),
    (Down        , KeyboardKey_KEY_DOWN),
    (Up          , KeyboardKey_KEY_UP),
    (PageUp      , KeyboardKey_KEY_PAGE_UP),
    (PageDown    , KeyboardKey_KEY_PAGE_DOWN),
    (Home        , KeyboardKey_KEY_HOME),
    (End         , KeyboardKey_KEY_END),
    (CapsLock    , KeyboardKey_KEY_CAPS_LOCK),
    (ScrollLock  , KeyboardKey_KEY_SCROLL_LOCK),
    (NumLock     , KeyboardKey_KEY_NUM_LOCK),
    (PrintScreen , KeyboardKey_KEY_PRINT_SCREEN),
    (Pause       , KeyboardKey_KEY_PAUSE),
    (F1          , KeyboardKey_KEY_F1),
    (F2          , KeyboardKey_KEY_F2),
    (F3          , KeyboardKey_KEY_F3),
    (F4          , KeyboardKey_KEY_F4),
    (F5          , KeyboardKey_KEY_F5),
    (F6          , KeyboardKey_KEY_F6),
    (F7          , KeyboardKey_KEY_F7),
    (F8          , KeyboardKey_KEY_F8),
    (F9          , KeyboardKey_KEY_F9),
    (F10         , KeyboardKey_KEY_F10),
    (F11         , KeyboardKey_KEY_F11),
    (F12         , KeyboardKey_KEY_F12),
    (LeftShift   , KeyboardKey_KEY_LEFT_SHIFT),
    (LeftControl , KeyboardKey_KEY_LEFT_CONTROL),
    (LeftAlt     , KeyboardKey_KEY_LEFT_ALT),
    (LeftSuper   , KeyboardKey_KEY_LEFT_SUPER),
    (Rightshift  , KeyboardKey_KEY_RIGHT_SHIFT),
    (Rightcontrol, KeyboardKey_KEY_RIGHT_CONTROL),
    (Rightalt    , KeyboardKey_KEY_RIGHT_ALT),
    (Rightsuper  , KeyboardKey_KEY_RIGHT_SUPER),
    (KbMenu      , KeyboardKey_KEY_KB_MENU),
    (Kp0         , KeyboardKey_KEY_KP_0),
    (Kp1         , KeyboardKey_KEY_KP_1),
    (Kp2         , KeyboardKey_KEY_KP_2),
    (Kp3         , KeyboardKey_KEY_KP_3),
    (Kp4         , KeyboardKey_KEY_KP_4),
    (Kp5         , KeyboardKey_KEY_KP_5),
    (Kp6         , KeyboardKey_KEY_KP_6),
    (Kp7         , KeyboardKey_KEY_KP_7),
    (Kp8         , KeyboardKey_KEY_KP_8),
    (Kp9         , KeyboardKey_KEY_KP_9),
    (KpDecimal   , KeyboardKey_KEY_KP_DECIMAL),
    (KpDivide    , KeyboardKey_KEY_KP_DIVIDE),
    (KpMultiply  , KeyboardKey_KEY_KP_MULTIPLY),
    (KpSubtract  , KeyboardKey_KEY_KP_SUBTRACT),
    (KpAdd       , KeyboardKey_KEY_KP_ADD),
    (KpEnter     , KeyboardKey_KEY_KP_ENTER),
    (KpEqual     , KeyboardKey_KEY_KP_EQUAL),
    (Back        , KeyboardKey_KEY_BACK),
    (VolumeUp    , KeyboardKey_KEY_VOLUME_UP),
    (VolumeDown  , KeyboardKey_KEY_VOLUME_DOWN),
];
