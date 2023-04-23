use std::thread;

use eframe::egui;

use super::{App, CloseFileAction, ConcurrentMessage};
use crate::{file::FileError, file_dialog, File};

impl App {
    /// Set error message
    ///
    /// This function can only be used in main thread
    fn set_error(&mut self, error: &'static str) {
        *self.error.lock().unwrap() = Some(error);
    }

    /// Remove error message
    ///
    /// This function can only be used in main thread
    pub fn clear_error(&mut self) {
        *self.error.lock().unwrap() = None;
    }

    // * Save file (save, save as)

    /// Save existing file, or save as if not registered
    pub(super) fn file_save_or_save_as(&mut self, ctx: &egui::Context) {
        println!("Save or save as");

        // Clone path, not whole file object
        let path_option = self.file.path().map(|path| path.clone());

        if let Some(path) = path_option {
            // File exists (registered)
            self.file_save_existing(&path, ctx);
        } else {
            // File is unregistered on system
            // Save as
            self.file_save_as(ctx);
        }
    }

    /// Save file as
    ///
    /// Shows *save file* dialog
    pub(super) fn file_save_as(&mut self, ctx: &egui::Context) {
        println!("Save as");

        if let Some(path) = file_dialog()
            .save_file()
            .map(|path_buf| path_buf.display().to_string())
        {
            self.file.set_path(&path);

            self.file_save_existing(&path, ctx);
        };
    }

    /// Save existing file in new thread (concurrent / synchronous)
    ///
    /// Should not be ran, unless file is already registered
    fn file_save_existing(&mut self, path: &str, ctx: &egui::Context) {
        println!("Save existing");

        // Get cryption key or return
        let Some(key) = &self.key else {
            self.set_error("Please enter a password");
            return;
        };

        // Set as writing
        *self.writing.lock().unwrap() = true;
        // Request to draw a new frame to update writing status
        //      (otherwise it would not update until user interaction)
        ctx.request_repaint();

        // Clone values to move to new thread

        // Note that this file is no longer the same object
        // This is why a message needs to be sent to the main thread to update save status
        let mut file = self.file.clone();

        // path and key (both type String), and ctx (type Context) can be cloned with no troubles
        let path = path.to_owned();
        let key = key.clone();
        let ctx = ctx.clone();

        // These variables (types Sender<_> and Arc<Mutex<_>>) can be
        //      cloned and moved into threads, while preserving state
        let sender = self.channel.sender.clone();
        let concurrent_write = self.writing.clone();
        let error = self.error.clone();

        // Create a new thread, moving values into closure
        thread::spawn(move || {
            // Save file and Handle errors
            // This is a slow process, hence the concurrent thread
            if let Err(err) = file.save_to_path(&path, &key) {
                match err {
                    //todo move to function
                    FileError::Cryption(err) => {
                        println!("cryption error: {:?}", err);

                        let error_msg = Some(match err {
                            cocoon::Error::Cryptography => "Invalid password",
                            cocoon::Error::UnrecognizedFormat => {
                                "Invalid file: Unrecognized format"
                            }
                            cocoon::Error::TooLarge => "Invalid file: Too large",
                            cocoon::Error::TooShort => "Invalid file: Too short",

                            cocoon::Error::Io(err) => panic!("[io] {:?}", err),
                        });

                        *error.lock().unwrap() = error_msg;
                    }

                    FileError::FromUtf8Error(_) => {
                        *error.lock().unwrap() = Some("Invalid file: Not formatted as string")
                    }

                    FileError::Io(err) => panic!("[io] {:?}", err),
                }
            }

            // Set as not writing
            *concurrent_write.lock().unwrap() = false;
            // Request to draw a new frame to update display of writing and save statuses
            //      (otherwise it would not update until user interaction)
            ctx.request_repaint();
            // Send a message to main thread, to update value of save status
            // This will be recieved on the next frame (requested above)
            sender
                .send(ConcurrentMessage::FinishConcurrentSave)
                .expect("Send message");
        });
    }

    // * Open existing file

    /// Open file
    ///
    /// Attempts to close current file (See `self.attempt_file_close`)
    ///
    /// Shows *open file* dialog
    pub(super) fn file_open(&mut self) {
        println!("Open");

        if !self.file_can_close() {
            self.attempting_file_close
                .set_action(CloseFileAction::OpenFile);
            // self.attempting_file_close = Some(Action::OpenFile);
            return;
        }

        // Get cryption key or return
        let Some(key) = &self.key else {
            self.set_error("Please enter the password for the file you wish to open");
            return;
        };

        if let Some(path) = file_dialog()
            .pick_file()
            .map(|path_buf| path_buf.display().to_string())
        {
            // Same file is already open
            // Don't open again
            if Some(&path) == self.file.path() {
                println!("  Save file");
                return;
            }

            // This is a slow process, but should not use concurrent thread,
            //      as no user actions can be performed until file loads anyway
            match File::open_path(path, key) {
                // Successful read
                Ok(file) => {
                    self.file = file;
                }

                // Handle errors
                Err(err) => match err {
                    //todo move to function
                    FileError::Cryption(err) => {
                        println!("cryption error: {:?}", err);

                        self.set_error(match err {
                            cocoon::Error::Cryptography => "Invalid password",
                            cocoon::Error::UnrecognizedFormat => {
                                "Invalid file: Unrecognized format"
                            }
                            cocoon::Error::TooLarge => "Invalid file: Too large",
                            cocoon::Error::TooShort => "Invalid file: Too short",

                            cocoon::Error::Io(err) => panic!("[io] {:?}", err),
                        });
                    }

                    FileError::FromUtf8Error(_) => {
                        self.set_error("Invalid file: Not formatted as string")
                    }

                    FileError::Io(err) => panic!("[io] {:?}", err),
                },
            }
        };
    }

    // * New file

    /// Create new file
    ///
    /// Attempts to close current file (See `self.attempt_file_close`)
    ///
    /// Sets current file to empty and unregistered (default)
    pub(super) fn file_new(&mut self) {
        println!("? New file");

        if !self.file_can_close() {
            self.attempting_file_close
                .set_action(CloseFileAction::NewFile);
            // self.attempting_file_close = Some(Action::NewFile);
            return;
        }

        println!("New file");

        self.file = File::default();
    }

    // * Handle file close

    /// Returns `true` if file is not changed, or condition is overridden
    pub(super) fn file_can_close(&self) -> bool {
        self.attempting_file_close
            .check_condition(!self.file.is_changed())
    }

    /// Run close action if allowed
    pub(super) fn call_close_action(&mut self) {
        // Cancel if file is not allowed to close (unsaved file)
        if !self.file_can_close() {
            return;
        }

        // If action was registered
        if let Some(action) = &self.attempting_file_close.action() {
            match action {
                // These 2 actions were registered by methods in this file,
                //      and call themselves again
                // They must reset the close action, or it will loop
                CloseFileAction::OpenFile => {
                    self.file_open();
                    self.reset_close_action();
                }
                CloseFileAction::NewFile => {
                    self.file_new();
                    self.reset_close_action();
                }

                // This action was registered by the `on_close_event` method
                // This cannot call `reset_close_action,
                //      as window will not close on next frame
                // This will not loop, as program will close before that
                CloseFileAction::CloseWindow => self.close_window_on_next_frame = true,
            }
        }
    }

    /// Reset close action
    pub(super) fn reset_close_action(&mut self) {
        self.attempting_file_close.reset_attempt();
    }
}
