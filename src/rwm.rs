use crate::config::bindings;
use std::{process, process::Command};
use x11rb::{
    connect,
    connection::Connection,
    protocol::{Event, xproto::*},
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let bindings = bindings();

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
                            "increase width" => {
                                if let Some(window) = focused {
                                    let geom = conn.get_geometry(window)?.reply()?;
                                    resize_window(
                                        &conn,
                                        window,
                                        geom.width as u32 + 20,
                                        geom.height as u32,
                                    )?;
                                }
                            }
                            "decrease width" => {
                                if let Some(window) = focused {
                                    let geom = conn.get_geometry(window)?.reply()?;
                                    resize_window(
                                        &conn,
                                        window,
                                        geom.width as u32 - 20,
                                        geom.height as u32,
                                    )?;
                                }
                            }
                            "increase height" => {
                                if let Some(window) = focused {
                                    let geom = conn.get_geometry(window)?.reply()?;
                                    resize_window(
                                        &conn,
                                        window,
                                        geom.width as u32,
                                        geom.height as u32 + 20,
                                    )?;
                                }
                            }
                            "decrease height" => {
                                if let Some(window) = focused {
                                    let geom = conn.get_geometry(window)?.reply()?;
                                    resize_window(
                                        &conn,
                                        window,
                                        geom.width as u32,
                                        geom.height as u32 - 20,
                                    )?;
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
fn resize_window<C: Connection>(
    conn: &C,
    window: Window,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let values = ConfigureWindowAux::new().width(width).height(height);
    conn.configure_window(window, &values)?;

    conn.flush()?;
    Ok(())
}
