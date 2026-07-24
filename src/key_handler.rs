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
                "exit" => std::process::exit(0),

                "close" => {
                    if let Some(window) = *focused {
                        conn.kill_client(window)?;
                        conn.flush()?;
                    }
                }

                "move left" => {
                    if let Some(window) = *focused {
                        if is_floating(&workspaces[*current], window) {
                            let geom = conn.get_geometry(window)?.reply()?;
                            move_window(conn, window, geom.x as i32 - 20, geom.y as i32)?;
                        }
                    }
                }
                "move down" => {
                    if let Some(window) = *focused {
                        if is_floating(&workspaces[*current], window) {
                            let geom = conn.get_geometry(window)?.reply()?;
                            move_window(conn, window, geom.x as i32, geom.y as i32 + 20)?;
                        }
                    }
                }
                "move up" => {
                    if let Some(window) = *focused {
                        if is_floating(&workspaces[*current], window) {
                            let geom = conn.get_geometry(window)?.reply()?;
                            move_window(conn, window, geom.x as i32, geom.y as i32 - 20)?;
                        }
                    }
                }
                "move right" => {
                    if let Some(window) = *focused {
                        if is_floating(&workspaces[*current], window) {
                            let geom = conn.get_geometry(window)?.reply()?;
                            move_window(conn, window, geom.x as i32 + 20, geom.y as i32)?;
                        }
                    }
                }

                "increase width" => {
                    if let Some(window) = *focused {
                        let geom = conn.get_geometry(window)?.reply()?;
                        resize_window(conn, window, geom.width as u32 + 20, geom.height as u32)?;
                    }
                }
                "decrease width" => {
                    if let Some(window) = *focused {
                        let geom = conn.get_geometry(window)?.reply()?;
                        resize_window(conn, window, geom.width as u32 - 20, geom.height as u32)?;
                    }
                }
                "increase height" => {
                    if let Some(window) = *focused {
                        let geom = conn.get_geometry(window)?.reply()?;
                        resize_window(conn, window, geom.width as u32, geom.height as u32 + 20)?;
                    }
                }
                "decrease height" => {
                    if let Some(window) = *focused {
                        let geom = conn.get_geometry(window)?.reply()?;
                        resize_window(conn, window, geom.width as u32, geom.height as u32 - 20)?;
                    }
                }

                "fullscreen" => {
                    if let Some(window) = *focused {
                        fullscreen(conn, fullscreen_states, window, screen)?;
                    }
                }

                "workspace 1" => switch_workspace(conn, workspaces, current, 0, focused, screen)?,
                "workspace 2" => switch_workspace(conn, workspaces, current, 1, focused, screen)?,
                "workspace 3" => switch_workspace(conn, workspaces, current, 2, focused, screen)?,
                "workspace 4" => switch_workspace(conn, workspaces, current, 3, focused, screen)?,
                "workspace 5" => switch_workspace(conn, workspaces, current, 4, focused, screen)?,
                "workspace 6" => switch_workspace(conn, workspaces, current, 5, focused, screen)?,
                "workspace 7" => switch_workspace(conn, workspaces, current, 6, focused, screen)?,
                "workspace 8" => switch_workspace(conn, workspaces, current, 7, focused, screen)?,
                "workspace 9" => switch_workspace(conn, workspaces, current, 8, focused, screen)?,

                "move to workspace 1" => {
                    if let Some(window) = *focused {
                        move_to_workspace(conn, workspaces, *current, 0, window, screen)?;
                    }
                }
                "move to workspace 2" => {
                    if let Some(window) = *focused {
                        move_to_workspace(conn, workspaces, *current, 1, window, screen)?;
                    }
                }
                "move to workspace 3" => {
                    if let Some(window) = *focused {
                        move_to_workspace(conn, workspaces, *current, 2, window, screen)?;
                    }
                }
                "move to workspace 4" => {
                    if let Some(window) = *focused {
                        move_to_workspace(conn, workspaces, *current, 3, window, screen)?;
                    }
                }
                "move to workspace 5" => {
                    if let Some(window) = *focused {
                        move_to_workspace(conn, workspaces, *current, 4, window, screen)?;
                    }
                }
                "move to workspace 6" => {
                    if let Some(window) = *focused {
                        move_to_workspace(conn, workspaces, *current, 5, window, screen)?;
                    }
                }
                "move to workspace 7" => {
                    if let Some(window) = *focused {
                        move_to_workspace(conn, workspaces, *current, 6, window, screen)?;
                    }
                }
                "move to workspace 8" => {
                    if let Some(window) = *focused {
                        move_to_workspace(conn, workspaces, *current, 7, window, screen)?;
                    }
                }
                "move to workspace 9" => {
                    if let Some(window) = *focused {
                        move_to_workspace(conn, workspaces, *current, 8, window, screen)?;
                    }
                }

                "focus left" => focus_prev(conn, &workspaces[*current], focused)?,
                "focus right" => focus_next(conn, &workspaces[*current], focused)?,

                "toggle bar" => {
                    *show_bar = !*show_bar;
                    if *show_bar {
                        conn.map_window(bar.window)?;
                    } else {
                        conn.unmap_window(bar.window)?;
                    }
                    conn.flush()?;
                }

                "volume up" => {
                    Command::new("wpctl")
                        .args(["set-volume", "@DEFAULT_AUDIO_SINK@", "5%+"])
                        .spawn()?;
                }
                "volume down" => {
                    Command::new("wpctl")
                        .args(["set-volume", "@DEFAULT_AUDIO_SINK@", "5%-"])
                        .spawn()?;
                }
                "mute" => {
                    Command::new("wpctl")
                        .args(["set-mute", "@DEFAULT_AUDIO_SINK@", "toggle"])
                        .spawn()?;
                }

                "toggle floating" => {
                    if let Some(window) = *focused {
                        toggle_floating(&mut workspaces[*current], window);
                        layout::tile(conn, &workspaces[*current], screen)?;
                    }
                }

                cmd => {
                    Command::new("sh").arg("-c").arg(cmd).spawn()?;
                }
            }
            break;
        }
    }
    conn.flush()?;
    Ok(())
}
