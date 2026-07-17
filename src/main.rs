use std::{process, process::Command};
use x11rb::{
    connect,
    connection::Connection,
    protocol::{Event, xproto::*},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("started");
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

    let keys = [24, 36];

    for key in keys {
        conn.grab_key(
            false,
            screen.root,
            ModMask::M4,
            key,
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
                if e.detail == 24 && e.state.contains(KeyButMask::MOD4) {
                    process::exit(0);
                } else if e.detail == 36 && e.state.contains(KeyButMask::MOD4) {
                    Command::new("kitty").spawn().expect("failed");
                }
                conn.flush()?;
            }

            _ => {}
        }
    }
}
