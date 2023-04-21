use std::thread;

use eframe::egui;

use super::{App, CloseAction, ConcurrentMessage};
use crate::{file_dialog, File};

impl App {
    /// Attempt to close file
    ///
    /// Returns `true` if file is not changed
    ///
    /// Otherwise, creates `Attempt` of `CloseAction`, which triggers dialog (and returns `false`)
    pub(super) fn attempt_file_close(&mut self, action: CloseAction) -> bool {
        println!("? Close");

        self.file_close.allow_if(!self.file.is_changed(), action)
    }

    /// Run action from `attempt_file_close`
    pub(super) fn attempt_file_close_action(&mut self) {
        if let Some(close_attempt) = &mut self.file_close.action() {
            match close_attempt {
                CloseAction::NewFile => self.file_new(),
                CloseAction::OpenFile => self.file_open(),
            }
        }
    }

    /// Save file
    ///
    /// If file is unregistered, runs `self.save_as()`
    pub(super) fn file_save(&mut self, ctx: &egui::Context) {
        println!("Save");

        // todo Remove clone ?
        if let Some(path) = self.file.clone().path() {
            self.file_save_sync(path, ctx);
        } else {
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
            self.file_save_sync(&path, ctx);
            // self.file_save_sync(&path);
            // self.file.save_to_path(&path).expect("Save file");
        };
    }

    /// Save file in new thread
    fn file_save_sync(&mut self, path: &str, ctx: &egui::Context) {
        println!("      thread: Save");

        // Set as writing
        *self.concurrent_write.lock().unwrap() = true;
        // Request to draw a new frame to update writing status
        //      (otherwise it would not update until user interaction)
        ctx.request_repaint();

        // Clone values to move to thread
        // This must be done, as closure lives longer than this method call
        //      (as it is a new thread), so values must be moved

        // Type Sender<_> can be cloned while preserving state
        let tx = self.channel.sender.clone();
        // path (type &str), and ctx (type &Context) can be cloned with no troubles, as they are references
        let path = path.to_owned();
        let ctx = ctx.clone();
        // Note that this file is no longer the same object
        // This is why a message needs to be sent to the main thread to update save status
        let mut file = self.file.clone();
        // Type Arc<Mutex<_>> can be cloned while preserving state
        let concurrent_write = self.concurrent_write.clone();

        // Create a new thread, moving values into closure
        thread::spawn(move || {
            // Save file
            // This is a slow process, hence the concurrent thread
            file.save_to_path(&path).expect("File save (sync)");

            println!("      thread: Saved? {}", file.is_registered_and_saved());
            println!("      thread: Finish save");

            // Set as not writing
            *concurrent_write.lock().unwrap() = false;

            // Request to draw a new frame to update display of writing and save statuses
            //      (otherwise it would not update until user interaction)
            ctx.request_repaint();
            // Send a message to main thread, to update value of save status
            // This will be recieved on the next frame (requested above)
            tx.send(ConcurrentMessage::FinishSave)
                .expect("Send message");
        });
    }

    /// Open file
    ///
    /// Attempts to close current file (See `self.attempt_file_close`)
    ///
    /// Shows *open file* dialog
    pub(super) fn file_open(&mut self) {
        println!("Open");

        if self.attempt_file_close(CloseAction::OpenFile) {
            if let Some(path) = file_dialog()
                .pick_file()
                .map(|path_buf| path_buf.display().to_string())
            {
                // This is a slow process, but should not use concurrent thread,
                //      as no user actions can be performed until file loads anyway
                self.file = File::open_path(path).expect("Open file");
            };
        }
    }

    /// Create new file
    ///
    /// Attempts to close current file (See `self.attempt_file_close`)
    ///
    /// Sets current file to empty and unregistered (default)
    pub(super) fn file_new(&mut self) {
        println!("? New file");

        if self.attempt_file_close(CloseAction::NewFile) {
            println!("New file");

            self.file = File::default();
        }
    }
}
