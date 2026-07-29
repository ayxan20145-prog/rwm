//! EWMH (Extended Window Manager Hints) support.
//!
//! This module sets the required properties so that external tools
//! (like pagers, panels, `xprop`) can identify your window manager.

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::COPY_FROM_PARENT;

/// Holds all EWMH atom identifiers we use.
pub struct EwmhAtoms {
    pub wm_name: Atom,
    pub check: Atom,
    pub supported: Atom,
    pub active_window: Atom,
    pub utf8_string: Atom,
}

/// Initialise all needed atoms.
pub fn intern_atoms<C: Connection>(
    conn: &C,
) -> Result<EwmhAtoms, Box<dyn std::error::Error>> {
    let wm_name = conn.intern_atom(false, b"_NET_WM_NAME")?.reply()?.atom;
    let check = conn.intern_atom(false, b"_NET_SUPPORTING_WM_CHECK")?.reply()?.atom;
    let supported = conn.intern_atom(false, b"_NET_SUPPORTED")?.reply()?.atom;
    let active_window = conn.intern_atom(false, b"_NET_ACTIVE_WINDOW")?.reply()?.atom;
    let utf8_string = conn.intern_atom(false, b"UTF8_STRING")?.reply()?.atom;

    Ok(EwmhAtoms {
        wm_name,
        check,
        supported,
        active_window,
        utf8_string,
    })
}

/// Create the dedicated WM check window and set all properties
/// on both the root window and the check window.
///
/// Returns the window ID of the check window (unmapped, not needed further).
pub fn setup_ewmh<C: Connection>(
    conn: &C,
    screen: &Screen,
    atoms: &EwmhAtoms,
) -> Result<Window, Box<dyn std::error::Error>> {
    // 1. Create an input‑only window for the check.
    let check_window = conn.generate_id()?;
    conn.create_window(
        COPY_FROM_PARENT as u8,
        check_window,
        screen.root,
        0, 0, 1, 1, 0,
        WindowClass::INPUT_ONLY,
        0,
        &CreateWindowAux::new(),
    )?;

    // 2. Set properties on the check window.
    let self_bytes = check_window.to_ne_bytes();
    conn.change_property(
        PropMode::REPLACE,
        check_window,
        atoms.check,
        AtomEnum::WINDOW,
        32,
        1,
        &self_bytes,
    )?;

    let name_bytes = b"rwm";
    conn.change_property(
        PropMode::REPLACE,
        check_window,
        atoms.wm_name,
        atoms.utf8_string,
        8,
        name_bytes.len() as u32,
        name_bytes,
    )?;

    // 3. Set properties on the root window.
    conn.change_property(
        PropMode::REPLACE,
        screen.root,
        atoms.check,
        AtomEnum::WINDOW,
        32,
        1,
        &self_bytes,
    )?;

    conn.change_property(
        PropMode::REPLACE,
        screen.root,
        atoms.wm_name,
        atoms.utf8_string,
        8,
        name_bytes.len() as u32,
        name_bytes,
    )?;

    // 4. Advertise which EWMH features we support.
    let supported_atoms = [atoms.wm_name, atoms.check, atoms.active_window];
    let mut data = Vec::with_capacity(supported_atoms.len() * 4);
    for atom in supported_atoms {
        data.extend_from_slice(&atom.to_ne_bytes());
    }
    conn.change_property(
        PropMode::REPLACE,
        screen.root,
        atoms.supported,
        AtomEnum::ATOM,
        32,
        supported_atoms.len() as u32,
        &data,
    )?;

    conn.flush()?;

    Ok(check_window)
}

/// Update the `_NET_ACTIVE_WINDOW` property on the root window.
pub fn set_active_window<C: Connection>(
    conn: &C,
    root: Window,
    atoms: &EwmhAtoms,
    window: Window,
) -> Result<(), Box<dyn std::error::Error>> {
    conn.change_property(
        PropMode::REPLACE,
        root,
        atoms.active_window,
        AtomEnum::WINDOW,
        32,
        1,
        &window.to_ne_bytes(),
    )?;
    conn.flush()?;
    Ok(())
}
