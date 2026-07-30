use std::process::Command;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;

use crate::{
    bar,
    config::KeyBinding,
    layout,
    workspace::{
        FullscreenState, Workspace, focus_next, focus_prev, fullscreen, is_floating,
        move_to_workspace, move_window, resize_window, switch_workspace, toggle_floating,
    },
    actions::{Action, Direction},
};

/// Check if the event modifiers match the binding modifiers.
pub fn modifiers_match(event: KeyButMask, binding: ModMask) -> bool {
    event.contains(KeyButMask::MOD4) == binding.contains(ModMask::M4)
        && event.contains(KeyButMask::SHIFT) == binding.contains(ModMask::SHIFT)
        && event.contains(KeyButMask::CONTROL) == binding.contains(ModMask::CONTROL)
        && event.contains(KeyButMask::MOD1) == binding.contains(ModMask::M1)
}

/// Handle a KeyPress event by executing the matching action.
pub fn handle_key_press<C: Connection>(
    conn: &C,
    event: &KeyPressEvent,
    bindings: &[KeyBinding],
    workspaces: &mut [Workspace],
    current: &mut usize,
    focused: &mut Option<Window>,
    fullscreen_states: &mut Vec<FullscreenState>,
    screen: &Screen,
    bar: &bar::Bar,
    show_bar: &mut bool,
) -> Result<(), Box<dyn std::error::Error>> {
    for binding in bindings {
        if event.detail == binding.key && modifiers_match(event.state, binding.modifiers) {
            match binding.action {
                Action::Exit => std::process::exit(0),

                Action::Close => {
                    if let Some(window) = *focused {
                        conn.kill_client(window)?;
                        conn.flush()?;
                    }
                }

                Action::Move(dir) => {
                    if let Some(window) = *focused {
                        if is_floating(&workspaces[*current], window) {
                            let geom = conn.get_geometry(window)?.reply()?;
                            let (dx, dy) = match dir {
                                Direction::Left  => (-20, 0),
                                Direction::Right => (20, 0),
                                Direction::Up    => (0, -20),
                                Direction::Down  => (0, 20),
                            };
                            move_window(conn, window, geom.x as i32 + dx, geom.y as i32 + dy)?;
                        }
                    }
                }

                Action::IncreaseWidth => {
                    if let Some(window) = *focused {
                        let geom = conn.get_geometry(window)?.reply()?;
                        resize_window(conn, window, geom.width as u32 + 20, geom.height as u32)?;
                    }
                }
                Action::DecreaseWidth => {
                    if let Some(window) = *focused {
                        let geom = conn.get_geometry(window)?.reply()?;
                        resize_window(conn, window, geom.width as u32 - 20, geom.height as u32)?;
                    }
                }
                Action::IncreaseHeight => {
                    if let Some(window) = *focused {
                        let geom = conn.get_geometry(window)?.reply()?;
                        resize_window(conn, window, geom.width as u32, geom.height as u32 + 20)?;
                    }
                }
                Action::DecreaseHeight => {
                    if let Some(window) = *focused {
                        let geom = conn.get_geometry(window)?.reply()?;
                        resize_window(conn, window, geom.width as u32, geom.height as u32 - 20)?;
                    }
                }

                Action::Fullscreen => {
                    if let Some(window) = *focused {
                        fullscreen(conn, fullscreen_states, window, screen)?;
                    }
                }

                Action::Workspace(ws) => switch_workspace(conn, workspaces, current, (ws - 1) as usize, focused, screen, *show_bar)?,

                Action::MoveToWorkspace(ws) => {
                    if let Some(window) = *focused {
                        move_to_workspace(conn, workspaces, *current, (ws - 1) as usize, window, screen, *show_bar)?;
                    }
                }

                Action::Focus(dir) => {
                    match dir {
                        Direction::Left  => focus_prev(conn, &workspaces[*current], focused)?,
                        Direction::Right => focus_next(conn, &workspaces[*current], focused)?,
                        _ => {}
                    }
                }

                Action::ToggleBar => {
                    *show_bar = !*show_bar;
                    if *show_bar {
                        conn.map_window(bar.window)?;
                    } else {
                        conn.unmap_window(bar.window)?;
                    }
                    conn.flush()?;
                }

                Action::VolumeDown => {
                    Command::new("wpctl")
                        .args(["set-volume", "@DEFAULT_AUDIO_SINK@", "5%-"])
                        .spawn()?;
                }
                Action::VolumeUp => {
                    Command::new("wpctl")
                        .args(["set-volume", "@DEFAULT_AUDIO_SINK@", "5%+"])
                        .spawn()?;
                }
                Action::Mute => {
                    Command::new("wpctl")
                        .args(["set-mute", "@DEFAULT_AUDIO_SINK@", "toggle"])
                        .spawn()?;
                }

                Action::ToggleFloating => {
                    if let Some(window) = *focused {
                        toggle_floating(&mut workspaces[*current], window);
                        layout::tile(conn, &workspaces[*current], screen, *show_bar)?;
                    }
                }
                Action::Run(cmd) => {
                    Command::new("sh").arg("-c").arg(cmd).spawn()?;
                }
            }
            break;
        }
    }
    conn.flush()?;
    Ok(())
}
