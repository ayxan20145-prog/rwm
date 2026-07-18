use x11rb::{COPY_FROM_PARENT, connection::Connection, protocol::xproto::*};

pub struct Bar {
    pub window: Window,
    pub gc: Gcontext,
}

pub fn create_bar<C: Connection>(
    conn: &C,
    screen: &Screen,
) -> Result<Bar, Box<dyn std::error::Error>> {
    let window = conn.generate_id()?;

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
        &CreateWindowAux::new().background_pixel(screen.black_pixel),
    )?;

    let gc = conn.generate_id()?;

    conn.create_gc(
        gc,
        window,
        &CreateGCAux::new().foreground(screen.white_pixel),
    )?;

    conn.map_window(window)?;

    conn.flush()?;

    Ok(Bar { window, gc })
}

pub fn draw<C: Connection>(
    conn: &C,
    bar: &Bar,
    current: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    conn.clear_area(false, bar.window, 0, 0, 0, 0)?;

    let mut text = String::new();

    for i in 0..9 {
        if i == current {
            text.push_str(&format!("[{}] ", i + 1));
        } else {
            text.push_str(&format!(" {}  ", i + 1));
        }
    }

    conn.image_text8(bar.window, bar.gc, 10, 16, text.as_bytes())?;

    Ok(())
}
