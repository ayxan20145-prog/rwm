//! # rwm Configuration
//!
//! This module contains configuration values for the window manager.
//!
//! It defines:
//! - Keyboard shortcuts
//! - Modifier keys
//! - Default applications
//! - Bar settings
//!
//! Keybindings are stored as [`KeyBinding`] structs and loaded by the
//! window manager during startup.

use x11rb::protocol::xproto::ModMask;
use crate::keys::*;

/// Represents a keyboard shortcut handled by the window manager.
///
/// A key binding consists of:
/// - Modifier keys (Super, Alt, Ctrl, Shift, etc.)
/// - An X11 keycode
/// - An action string interpreted by the key handler

pub struct KeyBinding {
    /// Modifier keys required for this shortcut.
    ///
    /// - [`ModMask::M4`] = Super/Windows key
    /// - [`ModMask::M1`] = Alt key
    pub modifiers: ModMask,

    /// X11 keycode for the key.
    pub key: u8,

    /// Action name executed by the window manager.
    pub action: &'static str,
}

/// Default modifier key used for window manager shortcuts.
pub const MOD: ModMask = ModMask::M4;

/// Default terminal emulator.
pub const TERM: &str = "kitty";

/// Default application launcher.
pub const LAUNCHER: &str = "rmenu";

/// Controls whether the status bar is enabled.
pub const BAR: bool = true;

/// Height of the bar
pub const BAR_HEIGHT: u32 = if BAR { 25 } else { 0 };

/// Returns all configured keyboard shortcuts.
///
/// This function defines every key combination supported by rwm,
/// including:
///
/// - Window manager controls
/// - Window movement
/// - Window resizing
/// - Fullscreen mode
/// - Workspace switching
/// - Focus movement
/// - Volume controls
/// - Application launching
pub fn bindings() -> Vec<KeyBinding> {
    vec![
        // =========================
        // Window manager controls
        // =========================

        // Exit the WM
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: E,
            action: "exit",
        },
        // Close the focused window
        KeyBinding {
            modifiers: MOD,
            key: Q,
            action: "close",
        },
        // Toggle bar
        KeyBinding {
            modifiers: MOD,
            key: B,
            action: "toggle bar",
        },
        // Toggle floating
        KeyBinding {
            modifiers: MOD,
            key: V,
            action: "toggle floating",
        },
        // =========================
        // Moving windows
        // =========================

        // Vim style movement (h/j/k/l)
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: H,
            action: "move left",
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: J,
            action: "move down",
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: K,
            action: "move up",
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: L,
            action: "move right",
        },
        // Arrow key movement
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: LEFT,
            action: "move left",
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: DOWN,
            action: "move down",
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: UP,
            action: "move up",
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: RIGHT,
            action: "move right",
        },
        // =========================
        // Window resizing
        // =========================

        // Change window width
        KeyBinding {
            modifiers: MOD,
            key: EQUAL,
            action: "increase width",
        },
        KeyBinding {
            modifiers: MOD,
            key: MINUS,
            action: "decrease width",
        },
        // Change window height
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: EQUAL,
            action: "increase height",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: MINUS,
            action: "decrease height",
        },
        // Toggle full screen mode
        KeyBinding {
            modifiers: MOD,
            key: F,
            action: "fullscreen",
        },
        // =========================
        // Workspaces
        // =========================

        // Switch workspace 1-9
        KeyBinding {
            modifiers: MOD,
            key: KEY_1,
            action: "workspace 1",
        },
        KeyBinding {
            modifiers: MOD,
            key: KEY_2,
            action: "workspace 2",
        },
        KeyBinding {
            modifiers: MOD,
            key: KEY_3,
            action: "workspace 3",
        },
        KeyBinding {
            modifiers: MOD,
            key: KEY_4,
            action: "workspace 4",
        },
        KeyBinding {
            modifiers: MOD,
            key: KEY_5,
            action: "workspace 5",
        },
        KeyBinding {
            modifiers: MOD,
            key: KEY_6,
            action: "workspace 6",
        },
        KeyBinding {
            modifiers: MOD,
            key: KEY_7,
            action: "workspace 7",
        },
        KeyBinding {
            modifiers: MOD,
            key: KEY_8,
            action: "workspace 8",
        },
        KeyBinding {
            modifiers: MOD,
            key: KEY_9,
            action: "workspace 9",
        },
        // Move current window to workspace 1-9
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_1,
            action: "move to workspace 1",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_2,
            action: "move to workspace 2",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_3,
            action: "move to workspace 3",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_4,
            action: "move to workspace 4",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_5,
            action: "move to workspace 5",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_6,
            action: "move to workspace 6",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_7,
            action: "move to workspace 7",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_8,
            action: "move to workspace 8",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_9,
            action: "move to workspace 9",
        },
        // =========================
        // Focus
        // =========================

        // Vim style movement (h/j/k/l)
        KeyBinding {
            modifiers: MOD,
            key: H,
            action: "focus left",
        },
        KeyBinding {
            modifiers: MOD,
            key: L,
            action: "focus right",
        },
        // Arrow key movement
        KeyBinding {
            modifiers: MOD,
            key: LEFT,
            action: "focus left",
        },
        KeyBinding {
            modifiers: MOD,
            key: RIGHT,
            action: "focus right",
        },
        // =========================
        // Volume controls
        // =========================

        // Increase volume
        KeyBinding {
            modifiers: ModMask::default(),
            key: VOLUME_DOWN,
            action: "volume down",
        },
        // Decrease volume
        KeyBinding {
            modifiers: ModMask::default(),
            key: VOLUME_UP,
            action: "volume up",
        },
        // Mute
        KeyBinding {
            modifiers: ModMask::default(),
            key: MUTE,
            action: "mute",
        },
        // =========================
        // Applications
        // =========================

        // Open terminal
        KeyBinding {
            modifiers: MOD,
            key: ENTER,
            action: TERM,
        },
        // Open application launcher
        KeyBinding {
            modifiers: MOD,
            key: D,
            action: LAUNCHER,
        },
    ]
}
