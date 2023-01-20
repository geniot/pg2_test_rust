use bitflags::bitflags;

pub enum Key {
    Backspace, Tab, Return, Escape, Space, Exclaim, Quotedbl, Hash, Dollar, Percent, Ampersand,
    Quote, LeftParen, RightParen, Asterisk, Plus, Comma, Minus, Period, Slash, Num0, Num1, Num2,
    Num3, Num4, Num5, Num6, Num7, Num8, Num9, Colon, Semicolon, Less, Equals, Greater, Question,
    At, LeftBracket, Backslash, RightBracket, Caret, Underscore, Backquote, A, B, C, D, E, F, G, H,
    I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Delete, CapsLock, F1, F2, F3, F4, F5, F6,
    F7, F8, F9, F10, F11, F12, PrintScreen, ScrollLock, Pause, Insert, Home, PageUp, End, PageDown,
    Right, Left, Down, Up, NumLock, KpDivide, KpMultiply, KpMinus, KpPlus, KpEnter, Kp1, Kp2, Kp3,
    Kp4, Kp5, Kp6, Kp7, Kp8, Kp9, Kp0, KpPeriod, KpEquals, KpComma, LCtrl, LShift, LAlt, LGui,
    RCtrl, RShift, RAlt, RGui, Unhandled
}

pub struct KeyEvent {
    /// Specific key for this event.
    pub key: Key,
    /// Key modifiers being held upon press, e.g. Shift or Ctrl, etc.
    pub keymod: KeyMod,
    /// Whether this is a key-repeat event.
    pub repeat: bool,
}

bitflags! {
    pub struct KeyMod: u16 {
        /// No key modifier.
        const NONE = 0x0000;
        /// Left Shift or Right Shift.
        const SHIFT = 0x0001;
        /// Left Control or Right Control.
        const CTRL = 0x0040;
        /// Left Alt/Option or Right Alt/Option.
        const ALT = 0x0100;
        /// Left GUI or Right GUI (e.g. Windows or Command keys).
        const GUI = 0x0400;
    }
}

impl KeyEvent {
    pub(crate) const fn new(key: Key, keymod: KeyMod, repeat: bool) -> Self {
        Self {
            key,
            keymod,
            repeat,
        }
    }
}