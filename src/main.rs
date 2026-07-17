use std::{process, process::Command};
use x11rb::{
    connect,
    connection::Connection,
    protocol::{Event, xproto::*},
};

struct KeyBinding {
    key: u8,
    action: &'static str,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bindings = [
        KeyBinding {
            key: 24, // q
            action: "exit",
        },
        KeyBinding {
            key: 36, // Return
            action: "kitty",
        },
        KeyBinding {
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
            ModMask::M4,
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
            }
            Event::DestroyNotify(e) => {
                println!("Destroyed {}", e.window);
            }
            Event::KeyPress(e) => {
                if e.state.contains(KeyButMask::MOD4) {
                    for binding in &bindings {
                        if e.detail == binding.key {
                            if binding.action == "exit" {
                                process::exit(0);
                            } else {
                                Command::new(binding.action)
                                    .spawn()
                                    .expect("Failed to execute command");
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
