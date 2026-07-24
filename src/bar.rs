use x11rb::{COPY_FROM_PARENT, connection::Connection, protocol::xproto::*};

/// Represents the status bar window and its graphics context.
pub struct Bar {
    /// The X11 window ID used for the bar.
    pub window: Window,

    /// The graphics context used to draw text and other elements.
    pub gc: Gcontext,
}

/// Creates a new bar window at the top of the screen.
pub fn create_bar<C: Connection>(
    conn: &C,
    screen: &Screen,
) -> Result<Bar, Box<dyn std::error::Error>> {
    // Generate a unique ID for the bar window
    let window = conn.generate_id()?;

    // Create the actual X11 window.
    conn.create_window(
        COPY_FROM_PARENT as u8,
        window,
        screen.root,
        0,
        0,
        screen.width_in_pixels,
        25,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        // Set the background color of the bar.
        &CreateWindowAux::new().background_pixel(screen.black_pixel),
    )?;

    // Generate a unique ID for the graphics context.
    let gc = conn.generate_id()?;

    // Create a graphics context for drawing text.
    conn.create_gc(
        gc,
        window,
        // Use the screen's white pixel as the text color.
        &CreateGCAux::new().foreground(screen.white_pixel),
    )?;

    conn.map_window(window)?;

    conn.flush()?;

    Ok(Bar { window, gc })
}

/// Draws the workspace indicator on the bar.
pub fn draw<C: Connection>(
    conn: &C,
    bar: &Bar,
    current: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    // Clear the previous contents of the bar.
    // Width and height of 0 means clear the entire window.
    conn.clear_area(false, bar.window, 0, 0, 0, 0)?;

    let mut text = String::new();

    // Build the workspace string.
    for i in 0..9 {
        if i == current {
            text.push_str(&format!("[{}] ", i + 1));
        } else {
            text.push_str(&format!(" {}  ", i + 1));
        }
    }

    // Draw the workspace text at position (10, 16).
    conn.image_text8(bar.window, bar.gc, 10, 16, text.as_bytes())?;

    Ok(())
}
