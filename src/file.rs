use std::{fs, io};

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
    /// This should only be run after a concurrent `save_to_path`
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

    /// Save file to given path
    ///
    /// Sets save state to saved
    pub fn save_to_path(&mut self, path: &str) -> io::Result<()> {
        // @ simulate slow process time, until cryption is added
        std::thread::sleep(std::time::Duration::from_secs(1));

        fs::write(path, &self.contents)?;

        self.saved = true;

        Ok(())
    }

    /// Open file from given path
    ///
    /// Returns saved `File` with contents and associated path
    pub fn open_path(path: impl Into<String>) -> io::Result<Self> {
        let path = path.into();

        // @ simulate slow process time, until cryption is added
        std::thread::sleep(std::time::Duration::from_secs(1));

        let contents = fs::read_to_string(&path)?;

        Ok(Self {
            contents,
            path: Some(path),
            saved: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_save_state() {
        // * Unregistered
        // is_saved: Always FALSE
        // is_changed: TRUE if non-empty
        // actual save state should not affect outcome

        // Default (Unregistered, Empty)
        let file = File::default();
        assert_eq!(file.is_registered_and_saved(), false);
        assert_eq!(file.is_changed(), false);

        // Unregistered, Empty
        let file = File {
            path: None,
            contents: String::new(),
            saved: false,
        };
        assert_eq!(file.is_registered_and_saved(), false);
        assert_eq!(file.is_changed(), false);
        // Same, but saved (should not matter)
        let file = File {
            path: None,
            contents: String::new(),
            saved: true,
        };
        assert_eq!(file.is_registered_and_saved(), false);
        assert_eq!(file.is_changed(), false);

        // Unregistered, NON-Empty
        let file = File {
            path: None,
            contents: String::from("Some contents"),
            saved: false,
        };
        assert_eq!(file.is_registered_and_saved(), false);
        assert_eq!(file.is_changed(), true);
        // Same, but saved (should not matter)
        let file = File {
            path: None,
            contents: String::from("Some contents"),
            saved: true,
        };
        assert_eq!(file.is_registered_and_saved(), false);
        assert_eq!(file.is_changed(), true);

        // * Registered
        // is_saved: TRUE if saved is TRUE
        // is_changed: TRUE if saved is FALSE
        // contents should not affect outcome

        // Registered, unsaved
        let file = File {
            path: Some(String::from("some/path")),
            contents: String::new(),
            saved: false,
        };
        assert_eq!(file.is_registered_and_saved(), false);
        assert_eq!(file.is_changed(), true);
        // Same, but non-empty (should not matter)
        let file = File {
            path: Some(String::from("some/path")),
            contents: String::from("Some contents"),
            saved: false,
        };
        assert_eq!(file.is_registered_and_saved(), false);
        assert_eq!(file.is_changed(), true);

        // Registered, saved
        let file = File {
            path: Some(String::from("some/path")),
            contents: String::new(),
            saved: true,
        };
        assert_eq!(file.is_registered_and_saved(), true);
        assert_eq!(file.is_changed(), false);
        // Same, but non-empty (should not matter)
        let file = File {
            path: Some(String::from("some/path")),
            contents: String::from("Some contents"),
            saved: true,
        };
        assert_eq!(file.is_registered_and_saved(), true);
        assert_eq!(file.is_changed(), false);
    }
}
