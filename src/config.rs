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
use crate::{keys::*, actions::{Action, Direction}};

/// Represents a keyboard shortcut handled by the window manager.
///
/// A key binding consists of:
/// - Modifier keys (Super, Alt, Ctrl, Shift, etc.)
/// - An X11 keycode
/// - An action enum interpreted by the key handler

pub struct KeyBinding {
    /// Modifier keys required for this shortcut.
    ///
    /// - [`ModMask::M4`] = Super/Windows key
    /// - [`ModMask::M1`] = Alt key
    pub modifiers: ModMask,

    /// X11 keycode for the key.
    pub key: u8,

    /// Action name executed by the window manager.
    pub action: Action,
}

/// Default modifier key used for window manager shortcuts.
pub const MOD: ModMask = ModMask::M4;

/// Default terminal emulator.
pub const TERM_CMD: &str = "kitty";

/// Default application launcher.
pub const LAUNCHER_CMD: &str = "rmenu";

/// Controls whether the status bar is enabled.
pub const BAR: bool = true;

/// Height of the bar
pub const BAR_HEIGHT: u32 = if BAR { 25 } else { 0 };

/// Step size for moving floating windows (pixels per key press).
pub const MOVE_STEP: i32 = 20;

/// Step size for resizing floating windows (pixels per key press).
pub const RESIZE_STEP: u16 = 20;

/// Increase volume by 5% command
pub const VOLUME_UP_CMD: &str = "wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%+";

/// Decrease volume by 5% command
pub const VOLUME_DOWN_CMD: &str = "wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%-";

/// Mute volume command
pub const VOLUME_MUTE_CMD: &str = "wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle";

/// Percentage of screen width for the master column (0–100). Default is 60%.
pub const MASTER_RATIO: u32 = 60;

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
            action: Action::Exit,
        },
        // Close the focused window
        KeyBinding {
            modifiers: MOD,
            key: Q,
            action: Action::Close,
        },
        // Toggle bar
        KeyBinding {
            modifiers: MOD,
            key: B,
            action: Action::ToggleBar,
        },
        // Toggle floating
        KeyBinding {
            modifiers: MOD,
            key: V,
            action: Action::ToggleFloating,
        },
        // =========================
        // Moving windows
        // =========================

        // Vim style movement (h/j/k/l)
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: H,
            action: Action::Move(Direction::Left, MOVE_STEP),
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: J,
            action: Action::Move(Direction::Down, MOVE_STEP),
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: K,
            action: Action::Move(Direction::Up, MOVE_STEP),
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: L,
            action: Action::Move(Direction::Right, MOVE_STEP),
        },
        // Arrow key movement
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: LEFT,
            action: Action::Move(Direction::Left, MOVE_STEP),
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: DOWN,
            action: Action::Move(Direction::Down, MOVE_STEP),
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: UP,
            action: Action::Move(Direction::Up, MOVE_STEP),
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: RIGHT,
            action: Action::Move(Direction::Right, MOVE_STEP),
        },
        // =========================
        // Window resizing
        // =========================

        // Change window width
        KeyBinding {
            modifiers: MOD,
            key: EQUAL,
            action: Action::Resize(Direction::Right, RESIZE_STEP),
        },
        KeyBinding {
            modifiers: MOD,
            key: MINUS,
            action: Action::Resize(Direction::Left, RESIZE_STEP),
        },
        // Change window height
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: EQUAL,
            action: Action::Resize(Direction::Down, RESIZE_STEP),
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: MINUS,
            action: Action::Resize(Direction::Up, RESIZE_STEP),
        },
        // Toggle full screen mode
        KeyBinding {
            modifiers: MOD,
            key: F,
            action: Action::Fullscreen,
        },
        // =========================
        // Workspaces
        // =========================

        // Switch workspace 1-9
        KeyBinding {
            modifiers: MOD,
            key: KEY_1,
            action: Action::Workspace(1),
        },
        KeyBinding {
            modifiers: MOD,
            key: KEY_2,
            action: Action::Workspace(2),
        },
        KeyBinding {
            modifiers: MOD,
            key: KEY_3,
            action: Action::Workspace(3),
        },
        KeyBinding {
            modifiers: MOD,
            key: KEY_4,
            action: Action::Workspace(4),
        },
        KeyBinding {
            modifiers: MOD,
            key: KEY_5,
            action: Action::Workspace(5),
        },
        KeyBinding {
            modifiers: MOD,
            key: KEY_6,
            action: Action::Workspace(6),
        },
        KeyBinding {
            modifiers: MOD,
            key: KEY_7,
            action: Action::Workspace(7),
        },
        KeyBinding {
            modifiers: MOD,
            key: KEY_8,
            action: Action::Workspace(8),
        },
        KeyBinding {
            modifiers: MOD,
            key: KEY_9,
            action: Action::Workspace(9),
        },
        // Move current window to workspace 1-9
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_1,
            action: Action::MoveToWorkspace(1),
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_2,
            action: Action::MoveToWorkspace(2),
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_3,
            action: Action::MoveToWorkspace(3),
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_4,
            action: Action::MoveToWorkspace(4),
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_5,
            action: Action::MoveToWorkspace(5),
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_6,
            action: Action::MoveToWorkspace(6),
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_7,
            action: Action::MoveToWorkspace(7),
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_8,
            action: Action::MoveToWorkspace(8),
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: KEY_9,
            action: Action::MoveToWorkspace(9),
        },
        // =========================
        // Focus
        // =========================

        // Vim style movement (h/j/k/l)
        KeyBinding {
            modifiers: MOD,
            key: H,
            action: Action::Focus(Direction::Left),
        },
        KeyBinding {
            modifiers: MOD,
            key: L,
            action: Action::Focus(Direction::Right),
        },
        // Arrow key movement
        KeyBinding {
            modifiers: MOD,
            key: LEFT,
            action: Action::Focus(Direction::Left),
        },
        KeyBinding {
            modifiers: MOD,
            key: RIGHT,
            action: Action::Focus(Direction::Right),
        },
        // =========================
        // Volume controls
        // =========================

        // Increase volume
        KeyBinding {
            modifiers: ModMask::default(),
            key: VOLUME_DOWN,
            action: Action::VolumeDown,
        },
        // Decrease volume
        KeyBinding {
            modifiers: ModMask::default(),
            key: VOLUME_UP,
            action: Action::VolumeUp,
        },
        // Mute
        KeyBinding {
            modifiers: ModMask::default(),
            key: MUTE,
            action: Action::Mute,
        },
        // =========================
        // Applications
        // =========================

        // Open terminal
        KeyBinding {
            modifiers: MOD,
            key: ENTER,
            action: Action::Run(TERM_CMD),
        },
        // Open application launcher
        KeyBinding {
            modifiers: MOD,
            key: D,
            action: Action::Run(LAUNCHER_CMD),
        },
    ]
}
