// use std::thread;

use std::{
    sync::{Arc, Mutex},
    thread,
};

use eframe::egui;
use egui::emath::Align2;

use crate::{
    file_dialog,
    sync::{Channel, Message},
    Attempt, File,
};

/// Main app state
// #[derive(Default)]
pub struct App {
    channel: Channel,

    /// Current file opened
    file: File,

    /// Whether file is currently writing
    concurrent_write: Arc<Mutex<bool>>,

    /// Attempt to close file (See `Attempt`)
    file_close: Attempt<CloseAction>,
}

// @ debug
impl Default for App {
    fn default() -> Self {
        Self {
            channel: Default::default(),

            file: File::open_path("/home/darcy/Documents/hello.txt").expect("Open initial file"),

            concurrent_write: Default::default(),

            file_close: Default::default(),
        }
    }
}

/// Actions to allow after close attempt passes
enum CloseAction {
    NewFile,
    OpenFile,
}

impl App {
    /// Attempt to close file
    ///
    /// Returns `true` if file is not changed
    ///
    /// Otherwise, creates `Attempt` of `CloseAction`, which triggers dialog (and returns `false`)
    fn attempt_file_close(&mut self, action: CloseAction) -> bool {
        println!("? Close");

        self.file_close.allow_if(!self.file.is_changed(), action)
    }

    /// Run action from `attempt_file_close`
    fn attempt_file_close_action(&mut self) {
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
    fn file_save(&mut self, ctx: &egui::Context) {
        println!("Save");

        // todo Remove clone ?
        if let Some(path) = self.file.clone().path() {
            // self.file.save_to_path(path).expect("Save file");
            self.file_save_sync(path, ctx);
        } else {
            self.file_save_as(ctx);
        }
    }

    /// Save file as
    ///
    /// Shows *save file* dialog
    fn file_save_as(&mut self, ctx: &egui::Context) {
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
        let tx = self.channel.tx.clone();
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
            tx.send(Message::FinishSave).expect("Send message");
        });
    }

    /// Open file
    ///
    /// Attempts to close current file (See `self.attempt_file_close`)
    ///
    /// Shows *open file* dialog
    fn file_open(&mut self) {
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
    fn file_new(&mut self) {
        println!("? New file");

        if self.attempt_file_close(CloseAction::NewFile) {
            println!("New file");

            self.file = File::default();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // println!("Saved? {} (main)", self.file.is_registered_and_saved());

        // * Handle concurrent messages

        if let Ok(msg) = self.channel.rx.try_recv() {
            match msg {
                Message::FinishSave => {
                    println!("Save finished!");
                    self.file.force_set_saved();
                }
            }
        }

        // Concurrent file write is active
        let concurrent_write = *self.concurrent_write.lock().unwrap();

        // Disabled save action if:
        //  - Concurrently writing a file
        //  - Or file is registered and saved
        let action_disabled_save = concurrent_write || self.file.is_registered_and_saved();

        egui::CentralPanel::default().show(ctx, |ui| {
            // Keybinds
            if !concurrent_write {
                if keys!(ui: CTRL + S) {
                    // Save
                    if !action_disabled_save {
                        self.file_save(ctx);
                    }
                } else if keys!(ui: CTRL + SHIFT + S) {
                    // Save as
                    self.file_save_as(ctx);
                } else if keys!(ui: CTRL + O) {
                    // Open file
                    self.file_open();
                } else if keys!(ui: CTRL + N) {
                    // New blank file
                    self.file_new();
                }
            }

            ui.heading("Edit text files");

            ui.horizontal(|ui| {
                // Show filepath if file is registered
                if let Some(path) = self.file.path() {
                    ui.monospace(path);
                }

                // Save state
                ui.label(if concurrent_write {
                    // File is currently being written to
                    "Writing..."
                } else if self.file.is_registered_and_saved() {
                    // File is registered and saved
                    "Saved"
                } else if self.file.is_changed() {
                    // File has changed
                    "UNSAVED"
                } else {
                    // File is unregistered
                    ""
                });
            });

            // File actions
            ui.horizontal(|ui| {
                // Save
                if ui
                    .add_enabled(!action_disabled_save, egui::Button::new("Save"))
                    .clicked()
                {
                    self.file_save(ctx);
                }

                // Save as
                if ui
                    .add_enabled(!concurrent_write, egui::Button::new("Save As"))
                    .clicked()
                {
                    self.file_save_as(ctx);
                }

                // Open file
                if ui
                    .add_enabled(!concurrent_write, egui::Button::new("Open"))
                    .clicked()
                {
                    self.file_open();
                }

                // New blank file
                if ui
                    .add_enabled(!concurrent_write, egui::Button::new("New"))
                    .clicked()
                {
                    self.file_new();
                }
            });

            // Editable text of file contents
            let edit_contents = ui.text_edit_multiline(self.file.contents_mut());
            // Set save state to unsaved if text was changed
            if edit_contents.changed() {
                self.file.mark_as_unsaved();
            }
        });

        // Attempting to close file
        if self.file_close.is_attempting() {
            // Create custom window dialog
            egui::Window::new("Close file without saving?")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
                .show(ctx, |ui| {
                    ui.label("File has unsaved changes. Would you like to save it?");

                    // Actions
                    ui.horizontal(|ui| {
                        // Close file without saving
                        if ui.button("Don't save").clicked() {
                            self.file_close.force();
                            self.attempt_file_close_action();
                        }
                        // Cancel attempt, returning to current file
                        if ui.button("Cancel").clicked() {
                            self.file_close.stop_attempt();
                        }
                        // Save file and close
                        if ui.button("Save").clicked() {
                            self.file_save(ctx);
                            self.attempt_file_close_action();
                        }
                    });
                });
        }
    }
}
