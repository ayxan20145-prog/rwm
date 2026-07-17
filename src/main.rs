use x11rb::{connect, connection::Connection, protocol::xproto::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    conn.flush()?;

    Ok(())
}
