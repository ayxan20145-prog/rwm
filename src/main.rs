//! # rwm
//!
//! A minimal tiling window manager written in Rust using X11 and `x11rb`.
//!
//! rwm manages application windows, handles keyboard shortcuts,
//! provides workspaces, and controls window layouts.
//!
//! ## Features
//!
//! - X11 window management
//! - Keyboard shortcut handling
//! - Workspace support
//! - Automatic tiling layouts
//! - Floating window support
//! - Custom status bar
//!
//! ## Modules
//!
//! - [`bar`] - Creates and draws the status bar.
//! - [`config`] - Stores user configuration and key bindings.
//! - [`key_handler`] - Handles keyboard shortcuts.
//! - [`layout`] - Calculates and applies window layouts.
//! - [`rwm`] - Main window manager event loop.
//! - [`workspace`] - Workspace and client window management.

mod bar;
mod config;
mod key_handler;
mod layout;
mod rwm;
mod workspace;

/// Program entry point.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start rwm.
    rwm::run()
}
