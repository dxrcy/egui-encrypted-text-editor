#[cfg(test)]
mod tests;

use std::{fs, io};

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
    /// This should only be run after saving with `save_to_path`,
    ///     which did not register as saved
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
    pub fn save_to_path_encrypted(&mut self, path: &str, key: &str) -> Result<(), cocoon::Error> {
        // Create encryptor
        let cocoon = Cocoon::new(key.as_bytes());

        // Get content as bytes
        let bytes = self.contents.clone().into_bytes().to_vec();

        // Open file (creates new if not already existing)
        let mut file = match fs::File::create(path) {
            Ok(file) => file,

            // Return an IO error if failed
            Err(error) => return Err(cocoon::Error::Io(error)),
        };

        // Write encrypted data to file
        cocoon.dump(bytes, &mut file)?;

        self.saved = true;
        Ok(())
    }

    /// Open encrypted file from given path
    ///
    /// Returns saved `File` with contents and associated path
    pub fn open_path_and_decrypt(
        path: impl Into<String>,
        key: &str,
    ) -> Result<Self, cocoon::Error> {
        let path = path.into();

        // Create decryptor
        let cocoon = Cocoon::new(key.as_bytes());

        // Open existing file
        let mut file = match fs::File::open(&path) {
            Ok(file) => file,

            // Return an IO error if failed
            Err(error) => return Err(cocoon::Error::Io(error)),
        };

        // Decrypt data (bytes) from file
        let bytes = cocoon.parse(&mut file)?;

        // Convert bytes to string
        // This may fail, if bytes do not form a valid utf8 string
        let contents = match String::from_utf8(bytes) {
            Ok(string) => string,

            // Bytes-to-string conversion failed
            // Return IO error of 'Invalid Data'
            Err(error) => {
                return Err(cocoon::Error::from(io::Error::new(
                    io::ErrorKind::InvalidData,
                    error,
                )))
            }
        };

        Ok(Self {
            contents,
            path: Some(path),
            saved: true,
        })
    }
}
