use eframe::egui;
use egui::emath::Align2;

use crate::{file_dialog, Attempt, File};

/// Main app state
#[derive(Default)]
pub struct App {
    /// Current file opened
    file: File,
    /// Attempt to close file (See `Attempt`)
    file_close: Attempt<CloseAction>,
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
    fn file_save(&mut self) {
        println!("Save");

        // todo Remove clone ?
        if let Some(path) = self.file.clone().path() {
            self.file.save_to_path(path).expect("Save file");
        } else {
            self.file_save_as();
        }
    }

    /// Save file as
    ///
    /// Shows *save file* dialog
    fn file_save_as(&mut self) {
        println!("Save as");

        if let Some(path) = file_dialog()
            .save_file()
            .map(|path_buf| path_buf.display().to_string())
        {
            self.file.set_path(&path);
            self.file.save_to_path(&path).expect("Save file");
        };
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
        egui::CentralPanel::default().show(ctx, |ui| {
            // Keybinds
            if keys!(ui: CTRL + S) {
                self.file_save();
            } else if keys!(ui: CTRL + SHIFT + S) {
                self.file_save_as();
            } else if keys!(ui: CTRL + O) {
                self.file_open();
            } else if keys!(ui: CTRL + N) {
                self.file_new();
            }

            ui.heading("Edit text files");

            // File path and save state
            ui.horizontal(|ui| {
                // Show filepath if file is registered
                if let Some(path) = self.file.path() {
                    ui.monospace(path);
                }

                ui.label(if self.file.is_registered_and_saved() {
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
                // Disabled if file is registered and saved
                if ui
                    .add_enabled(
                        !self.file.is_registered_and_saved(),
                        egui::Button::new("Save"),
                    )
                    .clicked()
                {
                    self.file_save();
                }

                // Save as
                if ui.button("Save As").clicked() {
                    self.file_save_as();
                }

                // Open file
                if ui.button("Open").clicked() {
                    self.file_open();
                }

                // New blank file
                if ui.button("New").clicked() {
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
                            self.file_save();
                            self.attempt_file_close_action();
                        }
                    });
                });
        }
    }
}
