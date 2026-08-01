//! X11 keycode definitions.
//!
//! These are Linux evdev/X11 keycodes.
//! Use these constants instead of raw numbers in keybindings.

#![allow(unused)]

// =====================
// Numbers
// =====================

pub const KEY_1: u8 = 10;
pub const KEY_2: u8 = 11;
pub const KEY_3: u8 = 12;
pub const KEY_4: u8 = 13;
pub const KEY_5: u8 = 14;
pub const KEY_6: u8 = 15;
pub const KEY_7: u8 = 16;
pub const KEY_8: u8 = 17;
pub const KEY_9: u8 = 18;
pub const KEY_0: u8 = 19;

// =====================
// Letters
// =====================

pub const Q: u8 = 24;
pub const W: u8 = 25;
pub const E: u8 = 26;
pub const R: u8 = 27;
pub const T: u8 = 28;
pub const Y: u8 = 29;
pub const U: u8 = 30;
pub const I: u8 = 31;
pub const O: u8 = 32;
pub const P: u8 = 33;

pub const A: u8 = 38;
pub const S: u8 = 39;
pub const D: u8 = 40;
pub const F: u8 = 41;
pub const G: u8 = 42;
pub const H: u8 = 43;
pub const J: u8 = 44;
pub const K: u8 = 45;
pub const L: u8 = 46;

pub const Z: u8 = 52;
pub const X: u8 = 53;
pub const C: u8 = 54;
pub const V: u8 = 55;
pub const B: u8 = 56;
pub const N: u8 = 57;
pub const M: u8 = 58;

// =====================
// Symbols
// =====================

pub const GRAVE: u8 = 49;
pub const MINUS: u8 = 20;
pub const EQUAL: u8 = 21;

pub const LEFT_BRACKET: u8 = 34;
pub const RIGHT_BRACKET: u8 = 35;

pub const SEMICOLON: u8 = 47;
pub const APOSTROPHE: u8 = 48;

pub const COMMA: u8 = 59;
pub const PERIOD: u8 = 60;
pub const SLASH: u8 = 61;

pub const BACKSLASH: u8 = 51;

// =====================
// Control keys
// =====================

pub const ESC: u8 = 9;
pub const TAB: u8 = 23;
pub const CAPS_LOCK: u8 = 66;

pub const ENTER: u8 = 36;
pub const SPACE: u8 = 65;
pub const BACKSPACE: u8 = 22;

// =====================
// Modifiers
// =====================

pub const LEFT_SHIFT: u8 = 50;
pub const RIGHT_SHIFT: u8 = 62;

pub const LEFT_CTRL: u8 = 37;
pub const RIGHT_CTRL: u8 = 105;

pub const LEFT_ALT: u8 = 64;
pub const RIGHT_ALT: u8 = 108;

pub const LEFT_SUPER: u8 = 133;
pub const RIGHT_SUPER: u8 = 134;

// =====================
// Function keys
// =====================

pub const F1: u8 = 67;
pub const F2: u8 = 68;
pub const F3: u8 = 69;
pub const F4: u8 = 70;
pub const F5: u8 = 71;
pub const F6: u8 = 72;
pub const F7: u8 = 73;
pub const F8: u8 = 74;
pub const F9: u8 = 75;
pub const F10: u8 = 76;
pub const F11: u8 = 95;
pub const F12: u8 = 96;

// =====================
// Navigation
// =====================

pub const HOME: u8 = 110;
pub const END: u8 = 115;

pub const INSERT: u8 = 118;
pub const DELETE: u8 = 119;

pub const PAGE_UP: u8 = 112;
pub const PAGE_DOWN: u8 = 117;

// =====================
// Arrow keys
// =====================

pub const LEFT: u8 = 113;
pub const DOWN: u8 = 116;
pub const UP: u8 = 111;
pub const RIGHT: u8 = 114;

// =====================
// Numpad
// =====================

pub const NUM_LOCK: u8 = 77;

pub const KP_DIVIDE: u8 = 106;
pub const KP_MULTIPLY: u8 = 63;
pub const KP_MINUS: u8 = 82;
pub const KP_PLUS: u8 = 86;
pub const KP_ENTER: u8 = 104;

pub const KP_0: u8 = 90;
pub const KP_1: u8 = 87;
pub const KP_2: u8 = 88;
pub const KP_3: u8 = 89;
pub const KP_4: u8 = 83;
pub const KP_5: u8 = 84;
pub const KP_6: u8 = 85;
pub const KP_7: u8 = 79;
pub const KP_8: u8 = 80;
pub const KP_9: u8 = 81;

// =====================
// Media keys
// =====================

pub const VOLUME_DOWN: u8 = 122;
pub const VOLUME_UP: u8 = 123;
pub const MUTE: u8 = 121;

pub const PLAY_PAUSE: u8 = 172;
pub const STOP: u8 = 174;
pub const NEXT: u8 = 171;
pub const PREVIOUS: u8 = 173;

// =====================
// System keys
// =====================

pub const PRINT: u8 = 107;
pub const PAUSE: u8 = 127;
pub const MENU: u8 = 135;
