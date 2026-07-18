use x11rb::protocol::xproto::ModMask;

pub struct KeyBinding {
    pub modifiers: ModMask,
    pub key: u8,
    pub action: &'static str,
}

const MOD: ModMask = ModMask::M4;
/*
Choose your modkey
   M4: Super
   M1: Alt
*/

pub fn bindings() -> Vec<KeyBinding> {
    // You can check the keycode in xev
    vec![
        KeyBinding {
            modifiers: MOD | ModMask::SHIFT,
            key: 26, // e
            action: "exit",
        },
        KeyBinding {
            modifiers: MOD,
            key: 24, // q
            action: "close",
        },
        // Moving windows
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
        // Resizing windows
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
        KeyBinding {
            modifiers: MOD,
            key: 41, // f
            action: "fullscreen",
        },
        // Workspaces
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
        // Other
        KeyBinding {
            modifiers: MOD,
            key: 36, // Return
            action: "kitty",
        },
        KeyBinding {
            modifiers: MOD,
            key: 40, // d
            action: "dmenu_run",
        },
    ]
}
