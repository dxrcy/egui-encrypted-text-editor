use eframe::{egui, emath::Align2, epaint::Color32};

use super::{App, CloseFileAction, ConcurrentMessage};

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Close window, if action was triggered in a previous frame
        //      (from a method that did not have `frame`)
        if self.close_window_on_next_frame {
            frame.close();
        }

        // * Handle concurrent messages

        if let Ok(msg) = self.channel.receiver.try_recv() {
            match msg {
                ConcurrentMessage::FinishConcurrentSave => {
                    println!("Save finished!");
                    self.file.force_set_saved();

                    if self.attempting_file_close.is_attempting() {
                        self.call_close_action();
                    }
                }
            }
        }

        // * Render main window

        // Whether the file is currently writing on a different thread
        let concurrently_writing = *self.writing.lock().unwrap();

        // Whether the save action should be disabled
        let disable_save_action = concurrently_writing || self.file.is_registered_and_saved();

        egui::CentralPanel::default().show(ctx, |ui| {
            // Keybinds
            // These mirror the ui buttons, but should be kept separate in code, as buttons can be disabled
            if !concurrently_writing {
                if keys!(ui: CTRL + S) {
                    // Save
                    if !disable_save_action {
                        self.file_save_or_save_as(ctx);
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
                ui.label(if concurrently_writing {
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
                    "No file open"
                });
            });

            // Cryption key (password)
            ui.horizontal(|ui| {
                let label = ui.label("Password: ");
                ui.monospace(&self.key).labelled_by(label.id);
            });

            // Error message
            if let Some(error) = *self.error.lock().unwrap() {
                ui.colored_label(Color32::RED, error);
            }

            // File actions
            ui.horizontal(|ui| {
                // Save
                if ui
                    .add_enabled(!disable_save_action, egui::Button::new("Save"))
                    .clicked()
                {
                    self.file_save_or_save_as(ctx);
                }
                // Save as
                if ui
                    .add_enabled(!concurrently_writing, egui::Button::new("Save As"))
                    .clicked()
                {
                    self.file_save_as(ctx);
                }
                // Open file
                if ui
                    .add_enabled(!concurrently_writing, egui::Button::new("Open"))
                    .clicked()
                {
                    self.file_open();
                }
                // New blank file
                if ui
                    .add_enabled(!concurrently_writing, egui::Button::new("New"))
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

        // * Render popup windows

        // Attempting to close file
        // Create custom window dialog if necessary
        if self.attempting_file_close.is_attempting() {
            if concurrently_writing {
                // Wait for file to finish writing
                // This cannot be overridden with a button,
                //      because it would only ever need to be closed while the file is writing
                //      if the program has frozen, and in that case it can be closed with task manager
                dialog_window("Waiting for file to save...").show(ctx, |ui| {
                    ui.label("File may corrupt if not saved properly.")
                });
            } else if !self.file.is_registered_and_saved() {
                // Closing unsaved file
                dialog_window("Do you want to save your changes?").show(ctx, |ui| {
                    ui.label("Your changes will be lost if you don't save them.");

                    // Actions
                    ui.horizontal(|ui| {
                        // Close file without saving
                        if ui.button("Don't save").clicked() {
                            // Override close condition
                            self.attempting_file_close.override_condition();

                            // Try action again
                            self.call_close_action();
                        }

                        // Cancel attempt, returning to current file
                        // Button and keybind
                        if ui.button("Cancel").clicked() || keys!(ui: Escape) {
                            // Stop attempting close file
                            self.reset_close_action();
                        }

                        // Save file and close
                        if ui.button("Save").clicked() {
                            // Save (concurrently)
                            // This will show 'wait for file to save' until save completes
                            self.file_save_or_save_as(ctx);

                            // Try action again
                            self.call_close_action();
                        }
                    });
                });
            }
        }
    }

    // Program was closed
    // ALT+F4, Close button, ect.
    fn on_close_event(&mut self) -> bool {
        // Set file close action to quit app
        self.attempting_file_close
            .set_action(CloseFileAction::CloseWindow);
        // Returns true if file is allowed to close
        self.file_can_close()
    }
}

/// Create a simple reusable popup dialog window
fn dialog_window(title: &str) -> egui::Window {
    egui::Window::new(title)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
}
