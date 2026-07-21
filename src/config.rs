use x11rb::protocol::xproto::ModMask;

pub struct KeyBinding {
    // Modifier keys (Super, Alt, Ctrl, Shift, etc.)
    pub modifiers: ModMask,

    // X11 Keycode (you can find it with xev)
    pub key: u8,

    // Action that the WM will execute
    pub action: &'static str,
}

// Choose your modifier key for the WM
//  M4: Super/Windows
//  M1: Alt
const MOD: ModMask = ModMask::M4;

// Choose your terminal emulator
const TERM: &str = "kitty";

// Choose your application launcher
const LAUNCHER: &str = "dmenu_run";

pub const BAR: bool = true;

pub fn bindings() -> Vec<KeyBinding> {
    vec![
        // =========================
        // Window manager controls
        // =========================

        // Exit the WM
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: 26, // e
            action: "exit",
        },
        // Close the focused window
        KeyBinding {
            modifiers: MOD,
            key: 24, // q
            action: "close",
        },
        // Toggle bar
        KeyBinding {
            modifiers: MOD,
            key: 56, // b
            action: "toggle bar",
        },
        // =========================
        // Moving windows
        // =========================

        // Vim style movement (h/j/k/l)
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: 43, // h
            action: "move left",
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: 44, // j
            action: "move down",
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: 45, // k
            action: "move up",
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: 46, // l
            action: "move right",
        },
        // Arrow key movement
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: 113, // left
            action: "move left",
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: 116, // down
            action: "move down",
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: 111, // up
            action: "move up",
        },
        KeyBinding {
            modifiers: MOD | ModMask::CONTROL,
            key: 114, // right
            action: "move right",
        },
        // =========================
        // Window resizing
        // =========================

        // Change window width
        KeyBinding {
            modifiers: MOD,
            key: 21, // =
            action: "increase width",
        },
        KeyBinding {
            modifiers: MOD,
            key: 20, // -
            action: "decrease width",
        },
        // Change window height
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: 21, // +
            action: "increase height",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: 20, // _
            action: "decrease height",
        },
        // Toggle full screen mode
        KeyBinding {
            modifiers: MOD,
            key: 41, // f
            action: "fullscreen",
        },
        // =========================
        // Workspaces
        // =========================

        // Switch workspace 1-9
        KeyBinding {
            modifiers: MOD,
            key: 10, // 1
            action: "workspace 1",
        },
        KeyBinding {
            modifiers: MOD,
            key: 11, // 2
            action: "workspace 2",
        },
        KeyBinding {
            modifiers: MOD,
            key: 12, // 3
            action: "workspace 3",
        },
        KeyBinding {
            modifiers: MOD,
            key: 13, // 4
            action: "workspace 4",
        },
        KeyBinding {
            modifiers: MOD,
            key: 14, // 5
            action: "workspace 5",
        },
        KeyBinding {
            modifiers: MOD,
            key: 15, // 6
            action: "workspace 6",
        },
        KeyBinding {
            modifiers: MOD,
            key: 16, // 7
            action: "workspace 7",
        },
        KeyBinding {
            modifiers: MOD,
            key: 17, // 8
            action: "workspace 8",
        },
        KeyBinding {
            modifiers: MOD,
            key: 18, // 9
            action: "workspace 9",
        },
        // Move current window to workspace 1-9
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: 10, // 1
            action: "move to workspace 1",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: 11, // 2
            action: "move to workspace 2",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: 12, // 3
            action: "move to workspace 3",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: 13, // 4
            action: "move to workspace 4",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: 14, // 5
            action: "move to workspace 5",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: 15, // 6
            action: "move to workspace 6",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: 16, // 7
            action: "move to workspace 7",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: 17, // 8
            action: "move to workspace 8",
        },
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: 18, // 9
            action: "move to workspace 9",
        },
        // =========================
        // Focus
        // =========================

        // Vim style movement (h/j/k/l)
        KeyBinding {
            modifiers: MOD,
            key: 43, // h
            action: "focus left",
        },
        KeyBinding {
            modifiers: MOD,
            key: 46, // l
            action: "focus right",
        },
        // Arrow key movement
        KeyBinding {
            modifiers: MOD,
            key: 113, // left
            action: "focus left",
        },
        KeyBinding {
            modifiers: MOD,
            key: 114, // right
            action: "focus right",
        },
        // =========================
        // Applications
        // =========================

        // Open terminal
        KeyBinding {
            modifiers: MOD,
            key: 36, // Return
            action: TERM,
        },
        // Open application launcher
        KeyBinding {
            modifiers: MOD,
            key: 40, // d
            action: LAUNCHER,
        },
    ]
}
