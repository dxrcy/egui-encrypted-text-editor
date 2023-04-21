/// Private macros
#[macro_use]
mod macros;
/// Main app
mod app;
/// 'Attempt' something, such as close a file
mod attempt;
/// Handle file input/output and save state
mod file;
/// Wrapper for `Sender` and `Receiver` types in `std::sync::mpsc`
mod channel;

pub use crate::app::App;

use std::path::PathBuf;

use crate::{attempt::Attempt, file::File, channel::Channel};

/// Get default directory to open file open/save dialogs in
fn get_start_dir() -> Option<PathBuf> {
    if let Some(dir) = dirs_next::document_dir() {
        return Some(dir);
    }
    if let Some(dir) = dirs_next::desktop_dir() {
        return Some(dir);
    }
    if let Some(dir) = dirs_next::home_dir() {
        return Some(dir);
    }
    None
}

/// Create simple file open/save dialog with `rfd`
fn file_dialog() -> rfd::FileDialog {
    let dialog = rfd::FileDialog::new();

    if let Some(dir) = get_start_dir() {
        dialog.set_directory(dir)
    } else {
        dialog
    }
}
