use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;

use crate::workspace::Workspace;

/// Tile windows in the given workspace using a master‑stack layout.
pub fn tile<C: Connection>(
    conn: &C,
    workspace: &Workspace,
    screen: &Screen,
) -> Result<(), Box<dyn std::error::Error>> {
    let tiled: Vec<Window> = workspace
        .windows
        .iter()
        .filter(|c| !c.floating)
        .map(|c| c.window)
        .collect();

    if tiled.is_empty() {
        return Ok(());
    }

    let width = screen.width_in_pixels as u32;
    let height = screen.height_in_pixels as u32;
    let master_width = width * 60 / 100;

    if tiled.len() == 1 {
        conn.configure_window(
            tiled[0],
            &ConfigureWindowAux::new()
                .x(0)
                .y(0)
                .width(width)
                .height(height),
        )?;
        return Ok(());
    }

    // Master window
    conn.configure_window(
        tiled[0],
        &ConfigureWindowAux::new()
            .x(0)
            .y(0)
            .width(master_width)
            .height(height),
    )?;

    // Stack windows
    let stack_count = tiled.len() - 1;
    let stack_height = height / stack_count as u32;

    for (i, window) in tiled[1..].iter().enumerate() {
        conn.configure_window(
            *window,
            &ConfigureWindowAux::new()
                .x(master_width as i32)
                .y((i as u32 * stack_height) as i32)
                .width(width - master_width)
                .height(stack_height),
        )?;
    }

    conn.flush()?;
    Ok(())
}
