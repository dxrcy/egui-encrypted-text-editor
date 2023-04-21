use eframe::egui;
use egui::emath::Align2;

use crate::{file_dialog, Attempt, File};

#[derive(Default)]
pub struct App {
    file: File,
    close_attempt: Attempt<CloseAction>,
}

enum CloseAction {
    NewFile,
    OpenFile,
}

impl App {
    fn attempt_file_close(&mut self, action: CloseAction) -> bool {
        println!("? Close");

        self.close_attempt
            .allow_if(!self.file.is_changed(), action)
    }

    fn attempt_file_close_action(&mut self) {
        if let Some(close_attempt) = &mut self.close_attempt.action() {
            match close_attempt {
                CloseAction::NewFile => self.file_new(),
                CloseAction::OpenFile => self.file_open(),
            }
        }
    }

    fn file_save(&mut self) {
        println!("Save");

        // todo Remove clone ?
        if let Some(path) = self.file.clone().path() {
            self.file.save(path).expect("Save file");
        } else {
            self.file_save_as();
        }
    }

    fn file_save_as(&mut self) {
        println!("Save as");

        if let Some(path) = file_dialog()
            .save_file()
            .map(|path_buf| path_buf.display().to_string())
        {
            self.file.set_path(&path);
            self.file.save(&path).expect("Save file");
        };
    }

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
            // Hotkeys
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

            ui.horizontal(|ui| {
                ui.monospace(self.file.path().unwrap_or(&"Unknown file".to_string()));

                ui.label(if self.file.is_saved() {
                    "Saved"
                } else if self.file.is_changed() {
                    "UNSAVED"
                } else {
                    ""
                });
            });

            ui.horizontal(|ui| {
                if ui
                    .add_enabled(!self.file.is_saved(), egui::Button::new("Save"))
                    .clicked()
                {
                    self.file_save();
                }

                if ui.button("Save As").clicked() {
                    self.file_save_as();
                }

                if ui.button("Open").clicked() {
                    self.file_open();
                }

                if ui.button("New").clicked() {
                    self.file_new();
                }
            });

            let edit_contents = ui.text_edit_multiline(self.file.contents_mut());

            if edit_contents.changed() {
                self.file.mark_as_unsaved();
            }
        });

        if self.close_attempt.is_attempting() {
            egui::Window::new("Close file without saving?")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Don't save").clicked() {
                            self.close_attempt.force();
                            self.attempt_file_close_action();
                        }
                        if ui.button("Cancel").clicked() {
                            self.close_attempt.stop_attempt();
                        }
                        if ui.button("Save").clicked() {
                            self.file_save();
                            self.attempt_file_close_action();
                        }
                    });
                });
        }
    }
}
