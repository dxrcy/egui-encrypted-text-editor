#[macro_use]
mod macros;

mod app;
mod attempt;
mod file;

use rfd::FileDialog;
use std::path::PathBuf;

pub use crate::app::App;
use crate::{attempt::Attempt, file::File};

pub fn get_start_dir() -> Option<PathBuf> {
    if let Some(dir) = dirs_next::document_dir() {
        return Some(dir);
    }
    if let Some(dir) = dirs_next::desktop_dir() {
        return Some(dir);
    }
    None
}

pub fn file_dialog() -> FileDialog {
    let dialog = FileDialog::new();

    if let Some(dir) = get_start_dir() {
        dialog.set_directory(dir)
    } else {
        dialog
    }
}
