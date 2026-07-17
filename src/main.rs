use std::{process, process::Command};
use x11rb::{
    connect,
    connection::Connection,
    protocol::{Event, xproto::*},
};

struct KeyBinding {
    modifiers: ModMask,
    key: u8,
    action: &'static str,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bindings = [
        KeyBinding {
            modifiers: ModMask::M4 | ModMask::SHIFT,
            key: 26, // e
            action: "exit",
        },
        KeyBinding {
            modifiers: ModMask::M4,
            key: 24, // q
            action: "close",
        },
        KeyBinding {
            modifiers: ModMask::M4,
            key: 36, // Return
            action: "kitty",
        },
        KeyBinding {
            modifiers: ModMask::M4,
            key: 40, // d
            action: "dmenu_run",
        },
    ];

    let (conn, screen_num) = connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    conn.change_window_attributes(
        screen.root,
        &ChangeWindowAttributesAux::new().event_mask(
            EventMask::SUBSTRUCTURE_REDIRECT
                | EventMask::SUBSTRUCTURE_NOTIFY
                | EventMask::BUTTON_PRESS
                | EventMask::POINTER_MOTION
                | EventMask::KEY_PRESS,
        ),
    )?;

    for binding in &bindings {
        conn.grab_key(
            true,
            screen.root,
            binding.modifiers,
            binding.key,
            GrabMode::ASYNC,
            GrabMode::ASYNC,
        )?;
    }

    conn.flush()?;

    loop {
        let event = conn.wait_for_event()?;

        match event {
            Event::MapRequest(e) => {
                println!("Map {}", e.window);

                conn.map_window(e.window)?;
                conn.flush()?;
            }
            Event::DestroyNotify(e) => {
                println!("Destroyed {}", e.window);
            }
            Event::KeyPress(e) => {
                for binding in &bindings {
                    if e.detail == binding.key && modifiers_match(e.state, binding.modifiers) {
                        match binding.action {
                            "exit" => process::exit(0),
                            "close" => {
                                let focused = conn.get_input_focus()?.reply()?.focus;
                                conn.kill_client(focused)?;
                                conn.flush()?;
                            }
                            cmd => {
                                Command::new(cmd).spawn()?;
                            }
                        }
                    }
                }
                conn.flush()?;
            }

            _ => {}
        }
    }
}
fn modifiers_match(event: KeyButMask, binding: ModMask) -> bool {
    event.contains(KeyButMask::MOD4) == binding.contains(ModMask::M4)
        && event.contains(KeyButMask::SHIFT) == binding.contains(ModMask::SHIFT)
        && event.contains(KeyButMask::CONTROL) == binding.contains(ModMask::CONTROL)
        && event.contains(KeyButMask::MOD1) == binding.contains(ModMask::M1)
}
