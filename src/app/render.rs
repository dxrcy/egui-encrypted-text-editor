use eframe::{egui, emath::Align2};

use super::{Action, App, ConcurrentMessage};

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Close window, if action was triggered in a previous frame
        //      (from a method that did not have `frame`)
        if self.close_window_on_next_frame {
            frame.close();
        }

        // println!("Saved? {} (main)", self.file.is_registered_and_saved());

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

        // Concurrent file write is active
        let concurrently_writing = *self.writing.lock().unwrap();

        // Disabled save action if:
        //  - Concurrently writing a file
        //  - Or file is registered and saved
        let action_disabled_save = concurrently_writing || self.file.is_registered_and_saved();

        egui::CentralPanel::default().show(ctx, |ui| {
            // Keybinds
            // These mirror the ui buttons, but should be written separate, as buttons can be disabled
            if !concurrently_writing {
                if keys!(ui: CTRL + S) {
                    // Save (concurrent)
                    if !action_disabled_save {
                        self.file_save(ctx);
                    }
                } else if keys!(ui: CTRL + SHIFT + S) {
                    // Save as (concurrent)
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
                    ""
                });
            });

            // File actions
            ui.horizontal(|ui| {
                // Save (concurrent)
                if ui
                    .add_enabled(!action_disabled_save, egui::Button::new("Save"))
                    .clicked()
                {
                    self.file_save(ctx);
                }

                // Save as (concurrent)
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

        // Attempting to close file
        // Create custom window dialog if necessary
        if self.attempting_file_close.is_attempting() {
            if concurrently_writing {
                // Wait for file to finish writing
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
                            self.file_save(ctx);

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
        self.attempting_file_close.set_action(Action::CloseWindow);
        // Returns true if file is allowed to close
        self.file_can_close()
    }
}

/// Create a simple reusable dialog window
fn dialog_window(title: &str) -> egui::Window {
    egui::Window::new(title)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
}
