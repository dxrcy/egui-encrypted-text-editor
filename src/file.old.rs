use std::fs;

// use crate::dialog_save_file;

use crate::CurrentFile::*;
use crate::State::*;

pub enum CurrentFile {
    Existing {
        file: File,
        path: String,
        state: State,
    },

    Unregistered {
        file: File,
    },
}

pub enum State {
    Saved,
    Unsaved,
}

impl State {
    pub fn is_saved(&self) -> bool {
        matches!(self, Saved)
    }
}

type Contents = String;

#[allow(dead_code)]
#[derive(Clone)]
pub struct File {
    contents: Contents,
}

impl File {
    pub fn new() -> Self {
        Self {
            contents: String::new(),
        }
    }

    pub fn save(&self, path: &str) {
        fs::write(path, &self.contents).expect("Write file");
    }

    pub fn open(path: &str) -> Self {
        let contents = fs::read_to_string(path).expect("Read file");

        Self { contents }
    }

    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }
}

impl Default for CurrentFile {
    fn default() -> Self {
        Self::new_unregistered()
    }
}

impl CurrentFile {
    pub fn new_unregistered() -> Self {
        Unregistered { file: File::new() }
    }

    pub fn is_saved(&self) -> bool {
        match self {
            Existing { state, .. } => state.is_saved(),
            Unregistered { file } => file.is_empty(),
        }
    }

    pub fn path(&self) -> Option<&str> {
        if let Existing { path, .. } = self {
            Some(path)
        } else {
            None
        }
    }

    fn file(&self) -> &File {
        match self {
            Existing { file, .. } => file,
            Unregistered { file } => file,
        }
    }

    fn file_mut(&mut self) -> &mut File {
        match self {
            Existing { file, .. } => file,
            Unregistered { file } => file,
        }
    }

    pub fn contents(&self) -> &str {
        &self.file().contents
    }

    pub fn contents_mut(&mut self) -> &mut Contents {
        &mut self.file_mut().contents
    }

    pub fn set_unsaved(&mut self) {
        if let Existing { state, .. } = self {
            *state = Unsaved;
        }
    }

    pub fn set_saved(&mut self) {
        if let Existing { state, .. } = self {
            *state = Saved;
        }
    }

    // pub fn save(&mut self) {
    //     match self {
    //         Existing { file, path, state } => {
    //             match state {
    //                 Saved => {
    //                     // do nothing
    //                 }

    //                 Unsaved => {
    //                     file.save(path);
    //                 }
    //             }
    //         }

    //         Unregistered { .. } => self.save_as(),
    //     }
    // }

    // pub fn save_as(&mut self) {
    //     if let Some(new_path) = dialog_save_file() {
    //         match self {
    //             Existing { path, .. } => {
    //                 *path = new_path;
    //             }
                
    //             Unregistered { file } => {
    //                 *self = Existing {
    //                     path: new_path,
    //                     file: file.clone(),
    //                     state: Unsaved,
    //                 }
    //             }
    //         }
            
    //         self.save();
    //     }
    // }

    // pub fn close(&mut self) {
    //     *self = Self::new_unregistered();
    // }

    // pub fn open(&mut self, path: &str) {
    //     let file = File::open(path);

    //     *self = Existing {
    //         path: path.to_string(),
    //         file,
    //         state: Saved,
    //     };
    // }
}
