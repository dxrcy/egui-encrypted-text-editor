use eframe::{egui, emath::Align2};

use super::{App, ConcurrentMessage};

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // println!("Saved? {} (main)", self.file.is_registered_and_saved());

        // * Handle concurrent messages

        if let Ok(msg) = self.channel.receiver.try_recv() {
            match msg {
                ConcurrentMessage::FinishSave => {
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
