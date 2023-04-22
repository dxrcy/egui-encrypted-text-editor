use std::thread;

use eframe::egui;

use super::{Action, App, ConcurrentMessage};
use crate::{file_dialog, File};

impl App {
    // /// Attempt to close file
    // ///
    // /// Returns `true` if file is not changed
    // ///
    // /// Otherwise, creates `Attempt` of `CloseAction`, which triggers dialog (and returns `false`)
    // pub(super) fn attempt_file_close(&mut self, action: CloseAction) -> bool {
    //     println!("? Close");

    //     self.attempting_file_close.allow_if(!self.file.is_changed(), action)
    // }

    // /// Run action from `attempt_file_close`
    // pub(super) fn attempt_file_close_action(&mut self, frame: &mut eframe::Frame) {
    //     if let Some(close_attempt) = &mut self.attempting_file_close.action() {
    //         match close_attempt {
    //             CloseAction::NewFile => self.file_new(),
    //             CloseAction::OpenFile => self.file_open(),

    //             CloseAction::CloseWindow => {
    //                 println!("Really closing now!");
    //                 frame.close()
    //             }
    //         }
    //     }
    // }

    /// Save file
    ///
    /// If file is unregistered, runs `self.save_as()`
    pub(super) fn file_save(&mut self, ctx: &egui::Context) {
        println!("Save");

        // todo Remove clone ?
        if let Some(path) = self.file.clone().path() {
            self.real_file_save(path, ctx);
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

            self.real_file_save(&path, ctx);

            // self.file_save_sync(&path);
            // self.file.save_to_path(&path).expect("Save file");
        };
    }

    // /// Save file in new thread (concurrent / synchronous)
    fn real_file_save(&mut self, path: &str, ctx: &egui::Context) {
        // println!("      Save file: concurrent? {}", concurrent);

        // Set as writing
        *self.writing.lock().unwrap() = true;
        // Request to draw a new frame to update writing status
        //      (otherwise it would not update until user interaction)
        ctx.request_repaint();

        //todo remove blocking option ?

        // if concurrent {
        // * Concurrent file save

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
        let concurrent_write = self.writing.clone();

        // Create a new thread, moving values into closure
        thread::spawn(move || {
            // Save file
            // This is a slow process, hence the concurrent thread
            file.save_to_path(&path).expect("File save (concurrent)");

            // println!(
            //     "      concurrent: Saved? {}",
            //     file.is_registered_and_saved()
            // );
            // println!("      concurrent: Finish save");

            // Set as not writing
            *concurrent_write.lock().unwrap() = false;
            // Request to draw a new frame to update display of writing and save statuses
            //      (otherwise it would not update until user interaction)
            ctx.request_repaint();
            // Send a message to main thread, to update value of save status
            // This will be recieved on the next frame (requested above)
            tx.send(ConcurrentMessage::FinishConcurrentSave)
                .expect("Send message");
        });
        // } else {
        //     // * Thread-blocking file save

        //     // Save file
        //     // This is a slow process, so blocks thread for a bit
        //     self.file.save_to_path(&path).expect("File save (blocking)");

        //     // println!(
        //     //     "      blocking: Saved? {}",
        //     //     self.file.is_registered_and_saved()
        //     // );
        //     // println!("      blocking: Finish save");

        //     // Set as not writing
        //     *self.writing.lock().unwrap() = false;
        //     // ? is this required ?
        //     ctx.request_repaint();
        // }
    }

    // /// Save file in same thread (blocking)
    // fn real_file_save_blocking(&mut self, path: &str, ctx: &egui::Context) {
    //     println!("      blocking: Save");

    //     // Set as writing
    //     *self.writing.lock().unwrap() = false;
    //     // ? is this required ?
    //     // ? this does not do anything ?
    //     ctx.request_repaint();

    //     // Save file
    //     // This is a slow process, so blocks thread for a bit
    //     self.file.save_to_path(&path).expect("File save (sync)");

    //     println!(
    //         "      blocking: Saved? {}",
    //         self.file.is_registered_and_saved()
    //     );
    //     println!("      blocking: Finish save");

    //     // Set as not writing
    //     *self.writing.lock().unwrap() = false;
    //     // ? is this required ?
    //     ctx.request_repaint();
    // }

    // /// Save file in new thread (concurrent / synchronous)
    // fn real_file_save_concurrent(&mut self, path: &str, ctx: &egui::Context) {
    //     println!("      concurrent: Save");

    //     // Set as writing
    //     *self.writing.lock().unwrap() = true;
    //     // Request to draw a new frame to update writing status
    //     //      (otherwise it would not update until user interaction)
    //     ctx.request_repaint();

    //     // Clone values to move to thread
    //     // This must be done, as closure lives longer than this method call
    //     //      (as it is a new thread), so values must be moved

    //     // Type Sender<_> can be cloned while preserving state
    //     let tx = self.channel.sender.clone();
    //     // path (type &str), and ctx (type &Context) can be cloned with no troubles, as they are references
    //     let path = path.to_owned();
    //     let ctx = ctx.clone();
    //     // Note that this file is no longer the same object
    //     // This is why a message needs to be sent to the main thread to update save status
    //     let mut file = self.file.clone();
    //     // Type Arc<Mutex<_>> can be cloned while preserving state
    //     let concurrent_write = self.writing.clone();

    //     // Create a new thread, moving values into closure
    //     thread::spawn(move || {
    //         // Save file
    //         // This is a slow process, hence the concurrent thread
    //         file.save_to_path(&path).expect("File save (sync)");

    //         println!(
    //             "      concurrent: Saved? {}",
    //             file.is_registered_and_saved()
    //         );
    //         println!("      concurrent: Finish save");

    //         // Set as not writing
    //         *concurrent_write.lock().unwrap() = false;
    //         // Request to draw a new frame to update display of writing and save statuses
    //         //      (otherwise it would not update until user interaction)
    //         ctx.request_repaint();
    //         // Send a message to main thread, to update value of save status
    //         // This will be recieved on the next frame (requested above)
    //         tx.send(ConcurrentMessage::FinishConcurrentSave)
    //             .expect("Send message");
    //     });
    // }

    // pub(super) fn attempt_file_close(&mut self) -> bool {}

    /// Open file
    ///
    /// Attempts to close current file (See `self.attempt_file_close`)
    ///
    /// Shows *open file* dialog
    pub(super) fn file_open(&mut self) {
        println!("Open");

        if !self.file_can_close() {
            self.attempting_file_close.set_action(Action::OpenFile);
            // self.attempting_file_close = Some(Action::OpenFile);
            return;
        }

        if let Some(path) = file_dialog()
            .pick_file()
            .map(|path_buf| path_buf.display().to_string())
        {
            // This is a slow process, but should not use concurrent thread,
            //      as no user actions can be performed until file loads anyway
            self.file = File::open_path(path).expect("Open file");
        };
    }

    /// Create new file
    ///
    /// Attempts to close current file (See `self.attempt_file_close`)
    ///
    /// Sets current file to empty and unregistered (default)
    pub(super) fn file_new(&mut self) {
        println!("? New file");

        if !self.file_can_close() {
            self.attempting_file_close.set_action(Action::NewFile);
            // self.attempting_file_close = Some(Action::NewFile);
            return;
        }

        println!("New file");

        self.file = File::default();
    }

    /// Returns `true` if condition is met for file to close, or is overridden
    pub(super) fn file_can_close(&self) -> bool {
        self.attempting_file_close
            .check_condition(self.file.is_registered_and_saved())
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
                Action::OpenFile => {
                    self.file_open();
                    self.reset_close_action();
                }
                Action::NewFile => {
                    self.file_new();
                    self.reset_close_action();
                }

                // This action was registered by the `on_close_event` method
                // This cannot call `reset_close_action,
                //      as window will not close on next frame
                // This will not loop, as program will close before that
                Action::CloseWindow => self.close_window_on_next_frame = true,
            }
        }
    }

    /// Reset close action
    pub(super) fn reset_close_action(&mut self) {
        self.attempting_file_close.reset_attempt();
    }
}
