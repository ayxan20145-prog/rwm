use crate::{
    bar,
    config::{BAR, bindings},
    key_handler, layout,
    workspace::*,
};
use x11rb::{
    connect,
    connection::Connection,
    protocol::{Event, xproto::*},
};

/// Starts the window manager.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Load user-defined key bindings from config.rs.
    let bindings = bindings();

    // Controls whether the status bar is visible.
    let mut show_bar = BAR;

    // Connect to the X server and get the active screen.
    let (conn, screen_num) = connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    // Cursor
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

    // Event masks & key grabs
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

    // Bar
    let bar = bar::create_bar(&conn, screen)?;

    let mut focused: Option<Window> = None;

    // Workspace creation
    let mut workspaces = Vec::new();
    for _ in 0..9 {
        workspaces.push(Workspace {
            windows: Vec::new(),
        })
    }
    let mut current = 0;
    let mut fullscreen_states: Vec<FullscreenState> = Vec::new();

    // Main loop
    loop {
        let event = conn.wait_for_event()?;

        match event {
            Event::MapRequest(e) => {
                workspaces[current].windows.push(Client {
                    window: e.window,
                    floating: false,
                });
                focused = Some(e.window);

                conn.map_window(e.window)?;
                layout::tile(&conn, &workspaces[current], screen)?;
                conn.set_input_focus(InputFocus::POINTER_ROOT, e.window, x11rb::CURRENT_TIME)?;
                conn.flush()?;
            }

            Event::DestroyNotify(e) => {
                for workspace in &mut workspaces {
                    workspace.windows.retain(|c| c.window != e.window);
                }
                layout::tile(&conn, &workspaces[current], screen)?;
                if focused == Some(e.window) {
                    focused = workspaces[current].windows.last().map(|c| c.window);
                }
            }

            Event::FocusIn(e) => {
                if workspaces[current]
                    .windows
                    .iter()
                    .any(|c| c.window == e.event)
                {
                    focused = Some(e.event);
                }
            }

            Event::KeyPress(e) => {
                key_handler::handle_key_press(
                    &conn,
                    &e,
                    &bindings,
                    &mut workspaces,
                    &mut current,
                    &mut focused,
                    &mut fullscreen_states,
                    screen,
                    &bar,
                    &mut show_bar,
                )?;
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
