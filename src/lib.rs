mod file;

use std::path::PathBuf;

pub use file::*;
use rfd::FileDialog;

pub struct Attempt<T> {
    action: Option<T>,
    force: bool,
}

impl<T> Default for Attempt<T> {
    fn default() -> Self {
        Self {
            action: None,
            force: false,
        }
    }
}

impl<T> Attempt<T> {
    pub fn allow_if(&mut self, condition: bool, action: T) -> bool {
        if self.force || condition {
            *self = Self::default();
            true
        } else {
            self.action = Some(action);
            false
        }
    }

    pub fn active(&self) -> bool {
        self.action.is_some()
    }

    pub fn action(&self) -> &Option<T> {
        &self.action
    }

    pub fn force(&mut self) {
        self.force = true;
    }

    pub fn give_up(&mut self) {
        *self = Self::default()
    }
}

#[allow(unreachable_code)]
pub fn get_start_dir() -> Option<PathBuf> {
    // @ remove this
    return Some(PathBuf::from(
        "/media/darcy/Windows-SSD/Users/darcy/Documents/code/egui-files",
    ));

    if let Some(dir) = dirs_next::document_dir() {
        return Some(dir);
    }
    if let Some(dir) = dirs_next::desktop_dir() {
        return Some(dir);
    }
    None
}

pub fn file_dialog() -> FileDialog {
    let dialog = rfd::FileDialog::new();

    if let Some(dir) = get_start_dir() {
        dialog.set_directory(dir)
    } else {
        dialog
    }
}

// pub fn dialog_open_file() -> Option<String> {
//     file_dialog()
//         .pick_file()
//         .map(|path_buf| path_buf.display().to_string())
// }

// pub fn dialog_save_file() -> Option<String> {
//     file_dialog()
//         .pick_file()
//         .map(|path_buf| path_buf.display().to_string())
// }
