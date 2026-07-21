use crate::{
    bar,
    config::{BAR, bindings},
};
use std::{process, process::Command};
use x11rb::{
    connect,
    connection::Connection,
    protocol::{Event, xproto::*},
};

struct Workspace {
    windows: Vec<Window>,
}

struct FullscreenState {
    window: Window,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    fullscreen: bool,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let bindings = bindings();
    let mut show_bar = BAR;

    let (conn, screen_num) = connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let font = conn.generate_id()?;
    conn.open_font(font, b"cursor")?;

    let cursor = conn.generate_id()?;
    conn.create_glyph_cursor(cursor, font, font, 68, 69, 0, 0, 0, 0xffff, 0xffff, 0xffff)?;

    conn.change_window_attributes(
        screen.root,
        &ChangeWindowAttributesAux::new().cursor(cursor),
    )?;

    conn.close_font(font)?;
    conn.flush()?;

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

    let bar = bar::create_bar(&conn, screen)?;

    let mut focused: Option<Window> = None;

    let mut workspaces = vec![
        Workspace {
            windows: Vec::new(),
        },
        Workspace {
            windows: Vec::new(),
        },
        Workspace {
            windows: Vec::new(),
        },
        Workspace {
            windows: Vec::new(),
        },
        Workspace {
            windows: Vec::new(),
        },
        Workspace {
            windows: Vec::new(),
        },
        Workspace {
            windows: Vec::new(),
        },
        Workspace {
            windows: Vec::new(),
        },
        Workspace {
            windows: Vec::new(),
        },
    ];

    let mut current = 0;

    let mut fullscreen_states: Vec<FullscreenState> = Vec::new();

    loop {
        let event = conn.wait_for_event()?;

        match event {
            Event::MapRequest(e) => {
                workspaces[current].windows.push(e.window);
                focused = Some(e.window);

                conn.map_window(e.window)?;
                conn.set_input_focus(InputFocus::POINTER_ROOT, e.window, x11rb::CURRENT_TIME)?;
                conn.flush()?;
            }
            Event::DestroyNotify(e) => {
                for workspace in &mut workspaces {
                    workspace.windows.retain(|&w| w != e.window);
                }

                if focused == Some(e.window) {
                    focused = workspaces[current].windows.last().copied();
                }
            }
            Event::FocusIn(e) => {
                if workspaces[current].windows.contains(&e.event) {
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
                            "fullscreen" => {
                                if let Some(window) = focused {
                                    fullscreen(&conn, &mut fullscreen_states, window, screen)?;
                                }
                            }
                            "workspace 1" => {
                                switch_workspace(
                                    &conn,
                                    &mut workspaces,
                                    &mut current,
                                    0,
                                    &mut focused,
                                )?;
                            }
                            "workspace 2" => {
                                switch_workspace(
                                    &conn,
                                    &mut workspaces,
                                    &mut current,
                                    1,
                                    &mut focused,
                                )?;
                            }
                            "workspace 3" => {
                                switch_workspace(
                                    &conn,
                                    &mut workspaces,
                                    &mut current,
                                    2,
                                    &mut focused,
                                )?;
                            }
                            "workspace 4" => {
                                switch_workspace(
                                    &conn,
                                    &mut workspaces,
                                    &mut current,
                                    3,
                                    &mut focused,
                                )?;
                            }
                            "workspace 5" => {
                                switch_workspace(
                                    &conn,
                                    &mut workspaces,
                                    &mut current,
                                    4,
                                    &mut focused,
                                )?;
                            }
                            "workspace 6" => {
                                switch_workspace(
                                    &conn,
                                    &mut workspaces,
                                    &mut current,
                                    5,
                                    &mut focused,
                                )?;
                            }
                            "workspace 7" => {
                                switch_workspace(
                                    &conn,
                                    &mut workspaces,
                                    &mut current,
                                    6,
                                    &mut focused,
                                )?;
                            }
                            "workspace 8" => {
                                switch_workspace(
                                    &conn,
                                    &mut workspaces,
                                    &mut current,
                                    7,
                                    &mut focused,
                                )?;
                            }
                            "workspace 9" => {
                                switch_workspace(
                                    &conn,
                                    &mut workspaces,
                                    &mut current,
                                    8,
                                    &mut focused,
                                )?;
                            }
                            "move to workspace 1" => {
                                if let Some(window) = focused {
                                    move_to_workspace(&conn, &mut workspaces, current, 0, window)?;
                                }
                            }
                            "move to workspace 2" => {
                                if let Some(window) = focused {
                                    move_to_workspace(&conn, &mut workspaces, current, 1, window)?;
                                }
                            }
                            "move to workspace 3" => {
                                if let Some(window) = focused {
                                    move_to_workspace(&conn, &mut workspaces, current, 2, window)?;
                                }
                            }
                            "move to workspace 4" => {
                                if let Some(window) = focused {
                                    move_to_workspace(&conn, &mut workspaces, current, 3, window)?;
                                }
                            }
                            "move to workspace 5" => {
                                if let Some(window) = focused {
                                    move_to_workspace(&conn, &mut workspaces, current, 4, window)?;
                                }
                            }
                            "move to workspace 6" => {
                                if let Some(window) = focused {
                                    move_to_workspace(&conn, &mut workspaces, current, 5, window)?;
                                }
                            }
                            "move to workspace 7" => {
                                if let Some(window) = focused {
                                    move_to_workspace(&conn, &mut workspaces, current, 6, window)?;
                                }
                            }
                            "move to workspace 8" => {
                                if let Some(window) = focused {
                                    move_to_workspace(&conn, &mut workspaces, current, 7, window)?;
                                }
                            }
                            "move to workspace 9" => {
                                if let Some(window) = focused {
                                    move_to_workspace(&conn, &mut workspaces, current, 8, window)?;
                                }
                            }
                            "focus left" => {
                                focus_prev(&conn, &workspaces[current], &mut focused)?;
                            }
                            "focus right" => {
                                focus_next(&conn, &workspaces[current], &mut focused)?;
                            }
                            "toggle bar" => {
                                show_bar = !show_bar;

                                if show_bar {
                                    conn.map_window(bar.window)?;
                                } else {
                                    conn.unmap_window(bar.window)?;
                                }

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
        if show_bar {
            bar::draw(&conn, &bar, current)?;
            conn.flush()?;
        } else {
            conn.unmap_window(bar.window)?;
            conn.flush()?;
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
fn fullscreen<C: Connection>(
    conn: &C,
    states: &mut Vec<FullscreenState>,
    window: Window,
    screen: &Screen,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(state) = states.iter_mut().find(|s| s.window == window) {
        if state.fullscreen {
            conn.configure_window(
                window,
                &ConfigureWindowAux::new()
                    .x(state.x)
                    .y(state.y)
                    .width(state.width)
                    .height(state.height),
            )?;

            state.fullscreen = false;
        } else {
            conn.configure_window(
                window,
                &ConfigureWindowAux::new()
                    .x(0)
                    .y(0)
                    .width(screen.width_in_pixels as u32)
                    .height(screen.height_in_pixels as u32),
            )?;

            state.fullscreen = true;
        }
    } else {
        let geom = conn.get_geometry(window)?.reply()?;

        states.push(FullscreenState {
            window,
            x: geom.x.into(),
            y: geom.y.into(),
            width: geom.width.into(),
            height: geom.height.into(),
            fullscreen: true,
        });

        conn.configure_window(
            window,
            &ConfigureWindowAux::new()
                .x(0)
                .y(0)
                .width(screen.width_in_pixels as u32)
                .height(screen.height_in_pixels as u32),
        )?;
    }

    conn.flush()?;
    Ok(())
}
fn switch_workspace<C: Connection>(
    conn: &C,
    workspaces: &mut [Workspace],
    current: &mut usize,
    new: usize,
    focused: &mut Option<Window>,
) -> Result<(), Box<dyn std::error::Error>> {
    if *current == new {
        return Ok(());
    }

    for &window in &workspaces[*current].windows {
        conn.unmap_window(window)?;
    }

    *current = new;

    for &window in &workspaces[*current].windows {
        conn.map_window(window)?;
    }

    *focused = workspaces[*current].windows.last().copied();

    conn.flush()?;
    Ok(())
}
fn move_to_workspace<C: Connection>(
    conn: &C,
    workspaces: &mut [Workspace],
    current: usize,
    target: usize,
    window: Window,
) -> Result<(), Box<dyn std::error::Error>> {
    workspaces[current].windows.retain(|&w| w != window);
    workspaces[target].windows.push(window);

    if current != target {
        conn.unmap_window(window)?;
    }

    conn.flush()?;
    Ok(())
}
fn focus_next<C: Connection>(
    conn: &C,
    workspace: &Workspace,
    focused: &mut Option<Window>,
) -> Result<(), Box<dyn std::error::Error>> {
    if workspace.windows.is_empty() {
        return Ok(());
    }

    let current = workspace
        .windows
        .iter()
        .position(|&w| Some(w) == *focused)
        .unwrap_or(0);

    let next = (current + 1) % workspace.windows.len();
    let window = workspace.windows[next];

    conn.set_input_focus(InputFocus::POINTER_ROOT, window, x11rb::CURRENT_TIME)?;

    conn.configure_window(
        window,
        &ConfigureWindowAux::new().stack_mode(StackMode::ABOVE),
    )?;

    *focused = Some(window);
    conn.flush()?;

    Ok(())
}
fn focus_prev<C: Connection>(
    conn: &C,
    workspace: &Workspace,
    focused: &mut Option<Window>,
) -> Result<(), Box<dyn std::error::Error>> {
    if workspace.windows.is_empty() {
        return Ok(());
    }

    let current = workspace
        .windows
        .iter()
        .position(|&w| Some(w) == *focused)
        .unwrap_or(0);

    let prev = if current == 0 {
        workspace.windows.len() - 1
    } else {
        current - 1
    };

    let window = workspace.windows[prev];

    conn.set_input_focus(InputFocus::POINTER_ROOT, window, x11rb::CURRENT_TIME)?;

    conn.configure_window(
        window,
        &ConfigureWindowAux::new().stack_mode(StackMode::ABOVE),
    )?;

    *focused = Some(window);
    conn.flush()?;

    Ok(())
}
