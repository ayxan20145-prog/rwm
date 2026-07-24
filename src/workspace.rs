use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;

pub struct Client {
    pub window: Window,
    pub floating: bool,
}

pub struct Workspace {
    pub windows: Vec<Client>,
}

pub struct FullscreenState {
    pub window: Window,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
}

/// Switch to another workspace.
pub fn switch_workspace<C: Connection>(
    conn: &C,
    workspaces: &mut [Workspace],
    current: &mut usize,
    new: usize,
    focused: &mut Option<Window>,
    screen: &Screen,
) -> Result<(), Box<dyn std::error::Error>> {
    if *current == new {
        return Ok(());
    }

    for client in &workspaces[*current].windows {
        conn.unmap_window(client.window)?;
    }

    *current = new;

    for client in &workspaces[*current].windows {
        conn.map_window(client.window)?;
    }

    *focused = workspaces[*current].windows.last().map(|c| c.window);

    crate::layout::tile(conn, &workspaces[*current], screen)?;

    conn.flush()?;
    Ok(())
}

/// Move a window to a different workspace.
pub fn move_to_workspace<C: Connection>(
    conn: &C,
    workspaces: &mut [Workspace],
    current: usize,
    target: usize,
    window: Window,
    screen: &Screen,
) -> Result<(), Box<dyn std::error::Error>> {
    workspaces[current].windows.retain(|c| c.window != window);
    workspaces[target].windows.push(Client {
        window,
        floating: false,
    });

    if current != target {
        conn.unmap_window(window)?;
    }

    crate::layout::tile(conn, &workspaces[current], screen)?;

    conn.flush()?;
    Ok(())
}

/// Focus the next window in the workspace.
pub fn focus_next<C: Connection>(
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
        .position(|c| Some(c.window) == *focused)
        .unwrap_or(0);

    let next = (current + 1) % workspace.windows.len();
    let window = workspace.windows[next].window;

    conn.set_input_focus(InputFocus::POINTER_ROOT, window, x11rb::CURRENT_TIME)?;
    conn.configure_window(
        window,
        &ConfigureWindowAux::new().stack_mode(StackMode::ABOVE),
    )?;

    *focused = Some(window);
    conn.flush()?;
    Ok(())
}

/// Focus the previous window in the workspace.
pub fn focus_prev<C: Connection>(
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
        .position(|c| Some(c.window) == *focused)
        .unwrap_or(0);

    let prev = if current == 0 {
        workspace.windows.len() - 1
    } else {
        current - 1
    };

    let window = workspace.windows[prev].window;

    conn.set_input_focus(InputFocus::POINTER_ROOT, window, x11rb::CURRENT_TIME)?;
    conn.configure_window(
        window,
        &ConfigureWindowAux::new().stack_mode(StackMode::ABOVE),
    )?;

    *focused = Some(window);
    conn.flush()?;
    Ok(())
}

/// Toggle the floating state of a window.
pub fn toggle_floating(workspace: &mut Workspace, window: Window) {
    if let Some(client) = workspace.windows.iter_mut().find(|c| c.window == window) {
        client.floating = !client.floating;
    }
}

/// Check if a window is floating.
pub fn is_floating(workspace: &Workspace, window: Window) -> bool {
    workspace
        .windows
        .iter()
        .find(|c| c.window == window)
        .map(|c| c.floating)
        .unwrap_or(false)
}

/// Toggle fullscreen state for a window.
pub fn fullscreen<C: Connection>(
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

/// Move a window by a given offset.
pub fn move_window<C: Connection>(
    conn: &C,
    window: Window,
    x: i32,
    y: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    conn.configure_window(window, &ConfigureWindowAux::new().x(x).y(y))?;
    conn.flush()?;
    Ok(())
}

/// Resize a window.
pub fn resize_window<C: Connection>(
    conn: &C,
    window: Window,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    conn.configure_window(
        window,
        &ConfigureWindowAux::new().width(width).height(height),
    )?;
    conn.flush()?;
    Ok(())
}
