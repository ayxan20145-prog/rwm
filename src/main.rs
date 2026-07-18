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

const MOD: ModMask = ModMask::M4;
/* Choose your modkey
   M4: Super
   M1: Alt
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // You can check the keycode in xev
    let bindings = [
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
                | EventMask::KEY_PRESS
                | EventMask::FOCUS_CHANGE,
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

    let mut windows: Vec<Window> = Vec::new();
    let mut focused: Option<Window> = None;

    loop {
        let event = conn.wait_for_event()?;

        match event {
            Event::MapRequest(e) => {
                windows.push(e.window);
                focused = Some(e.window);

                conn.map_window(e.window)?;
                conn.flush()?;
            }
            Event::DestroyNotify(e) => {
                windows.retain(|&w| w != e.window);

                if focused == Some(e.window) {
                    focused = windows.last().copied();
                }
            }
            Event::FocusIn(e) => {
                if windows.contains(&e.event) {
                    focused = Some(e.event);
                }
            }
            Event::KeyPress(e) => {
                for binding in &bindings {
                    if e.detail == binding.key && modifiers_match(e.state, binding.modifiers) {
                        match binding.action {
                            "exit" => process::exit(0),
                            "close" => {
                                if let Some(window) = focused {
                                    conn.kill_client(window)?;
                                    conn.flush()?;
                                }
                            }
                            "move left" => {
                                if let Some(window) = focused {
                                    let geom = conn.get_geometry(window)?.reply()?;
                                    move_window(&conn, window, geom.x as i32 - 20, geom.y as i32)?;
                                }
                            }
                            "move down" => {
                                if let Some(window) = focused {
                                    let geom = conn.get_geometry(window)?.reply()?;
                                    move_window(&conn, window, geom.x as i32, geom.y as i32 + 20)?;
                                }
                            }
                            "move up" => {
                                if let Some(window) = focused {
                                    let geom = conn.get_geometry(window)?.reply()?;
                                    move_window(&conn, window, geom.x as i32, geom.y as i32 - 20)?;
                                }
                            }
                            "move right" => {
                                if let Some(window) = focused {
                                    let geom = conn.get_geometry(window)?.reply()?;
                                    move_window(&conn, window, geom.x as i32 + 20, geom.y as i32)?;
                                }
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
fn move_window<C: Connection>(
    conn: &C,
    window: Window,
    x: i32,
    y: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    conn.configure_window(window, &ConfigureWindowAux::new().x(x).y(y))?;

    conn.flush()?;
    Ok(())
}
