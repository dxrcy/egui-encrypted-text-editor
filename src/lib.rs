mod file;

use std::path::PathBuf;

pub use file::*;
use rfd::FileDialog;

#[derive(Default)]
pub struct Attempt {
    active: bool,
    forced: bool,
}

impl Attempt {
    pub fn allow_if(&mut self, condition: bool) -> bool {
        if self.forced || condition {
            *self = Self::default();
            true
        } else {
            self.active = true;
            false
        }
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn force(&mut self) {
        self.forced = true;
    }

    pub fn give_up(&mut self) {
        self.active = false;
    }
}

#[allow(unreachable_code)]
pub fn get_start_dir() -> Option<PathBuf> {
    // @ remove this
    return Some(PathBuf::from("/media/darcy/Windows-SSD/Users/darcy/Documents/code/egui-files"));

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
