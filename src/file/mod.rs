#[cfg(test)]
mod tests;

use std::{fs, io, string::FromUtf8Error};

use cocoon::Cocoon;

/// Simple file handler API
#[derive(Clone, Default)]
pub struct File {
    /// Path to file
    ///
    /// `None` if file is not registered on file system (was never saved)
    path: Option<String>,
    /// Contents of file
    contents: String,
    /// Whether file is saved
    saved: bool,
}

#[allow(dead_code)]
/// Error for `EncryptedFile` struct
#[derive(Debug)]
pub enum FileError {
    /// `io::Error`
    Io(io::Error),
    /// Cocoon error
    ///
    /// Encryption or decryption failed due to bad file
    ///    
    /// Should not occur
    Cryption(cocoon::Error),
    /// Failed to convert bytes to `String`
    ///
    /// Should not occur
    FromUtf8Error(FromUtf8Error),
}

impl File {
    /// Returns `true` if file does not have an associated filepath (was never saved)
    fn is_registered(&self) -> bool {
        self.path.is_some()
    }

    /// Returns `true` if file is registered and saved
    pub fn is_registered_and_saved(&self) -> bool {
        self.is_registered() && self.saved
    }

    /// Returns `true` if file is unregistered and unchanged (empty)
    pub fn is_unregistered_and_unchanged(&self) -> bool {
        !self.is_registered() && self.contents.is_empty()
    }

    /// Returns `true` if:
    ///  - File is registered, and NOT saved
    ///  - File is not registered, and NOT empty
    pub fn is_changed(&self) -> bool {
        if self.is_registered() {
            !self.saved
        } else {
            !self.contents().is_empty()
        }
    }

    /// Get file contents as reference
    pub fn contents(&self) -> &String {
        &self.contents
    }

    /// Get file contents as mutable reference
    pub fn contents_mut(&mut self) -> &mut String {
        &mut self.contents
    }

    /// Set save state to unsaved
    pub fn mark_as_unsaved(&mut self) {
        if self.is_registered_and_saved() {
            self.saved = false;
        }
    }

    /// Set save state to unsaved
    ///
    // /// This should only be run after a concurrent `save_to_path`
    pub fn force_set_saved(&mut self) {
        self.saved = true;
    }

    /// Get filepath as reference
    ///
    /// `None` if file is not registered on file system (was never saved)
    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Set filepath
    pub fn set_path(&mut self, path: impl Into<String>) {
        self.path = Some(path.into())
    }

    /// Save encrypted file to given path
    ///
    /// Sets save state to saved
    pub fn save_to_path_encrypted(&mut self, path: &str, key: &str) -> Result<(), FileError> {
        let cocoon = Cocoon::new(key.as_bytes());

        let bytes = self.contents.clone().into_bytes().to_vec();

        let mut file = match fs::File::create(path) {
            Ok(file) => file,
            Err(err) => return Err(FileError::Io(err)),
        };

        if let Err(err) = cocoon.dump(bytes, &mut file) {
            return Err(FileError::Cryption(err));
        };

        self.saved = true;

        Ok(())
    }

    /// Open encrypted file from given path
    ///
    /// Returns saved `File` with contents and associated path
    pub fn open_path_and_decrypt(path: impl Into<String>, key: &str) -> Result<Self, FileError> {
        let path = path.into();

        let cocoon = Cocoon::new(key.as_bytes());

        let mut file = match fs::File::open(&path) {
            Ok(file) => file,
            Err(err) => return Err(FileError::Io(err)),
        };

        let bytes = match cocoon.parse(&mut file) {
            Ok(bytes) => bytes,
            Err(err) => return Err(FileError::Cryption(err)),
        };

        let contents = match String::from_utf8(bytes) {
            Ok(string) => string,
            Err(err) => return Err(FileError::FromUtf8Error(err)),
        };

        Ok(Self {
            contents,
            path: Some(path),
            saved: true,
        })
    }
}
