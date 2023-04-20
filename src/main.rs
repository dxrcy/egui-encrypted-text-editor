#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, emath::Align2, epaint::Color32};
use egui_files::{dialog_open_file, dialog_save_file, Attempt, CurrentFile, File, State};

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(1000.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Native file dialogs and drag-and-drop files",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}

#[derive(Default)]
struct App {
    // path: Option<String>,
    error: Option<&'static str>,

    file: CurrentFile,

    // saved: bool,
    close_file_attempt: Attempt,
}

impl App {
    pub fn attempt_close_file(&mut self) -> bool {
        let is_able = self.close_file_attempt.allow_if(self.file.is_saved());
        if is_able {
            println!("Close file");

            self.file = CurrentFile::default();
        }
        is_able
    }

    pub fn file_open(&mut self, path: &str) {
        if self.attempt_close_file() {
            let file = File::open(path);

            *self = CurrentFile::Existing {
                path: path.to_string(),
                file,
                state: State::Saved,
            };
        }
    }

    pub fn file_close(&mut self) {
        if self.attempt_close_file() {
            self.file = CurrentFile::new_unregistered();
        }
    }

    pub fn file_save(&mut self) {
        println!("SAVE FILE");

        match &self.file {
            CurrentFile::Existing { file, path, state } => {
                match state {
                    State::Saved => {
                        // do nothing
                    }

                    State::Unsaved => {
                        file.save(path);
                    }
                }
            }

            CurrentFile::Unregistered { .. } => self.file_save_as(),
        }
    }

    pub fn file_save_as(&mut self) {
        println!("SAVE AS file");

        if let Some(new_path) = dialog_save_file() {
            match &mut self.file {
                CurrentFile::Existing { path, .. } => {
                    *path = new_path;
                }

                CurrentFile::Unregistered { file } => {
                    self.file = CurrentFile::Existing {
                        path: new_path,
                        file: file.clone(),
                        state: State::Unsaved,
                    }
                }
            }

            self.file_save();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Drag-and-drop files onto the window!");

            ui.label(if self.file.is_saved() {
                "Saved"
            } else {
                "UNSAVED"
            });

            // if ui.button("saved?").clicked() {
            //     self.saved ^= true;
            // }

            ui.horizontal(|ui| {
                if ui.button("Open").clicked() {
                    if let Some(path) = dialog_open_file() {
                        self.file_open(&path);
                    }
                }

                if ui.button("New").clicked() {
                    self.file_close();
                }

                if ui.button("Save").clicked() {
                    self.file_save();
                }

                if ui.button("Save As").clicked() {
                    self.file_save_as();
                }
            });

            if let Some(path) = self.file.path() {
                ui.horizontal(|ui| {
                    ui.label("Current Filepath:");
                    ui.monospace(path);
                });
            }

            if let Some(error) = self.error {
                ui.colored_label(Color32::RED, error);
            }

            ui.separator();

            let edit_contents = ui.text_edit_multiline(self.file.contents_mut());

            if edit_contents.changed() {
                self.file.set_unsaved();
            }
        });

        preview_files_being_dropped(ctx);

        // Collect dropped files:
        ctx.input(|input_state| {
            let files = &input_state.raw.dropped_files;

            if files.len() == 1 {
                if let Some(file) = files.first() {
                    if let Some(path) = &file.path {
                        self.file_open(&path.display().to_string())
                    } else if !file.name.is_empty() {
                        self.file_open(&file.name)
                    } else {
                        self.error = Some("Unable to open file");
                    };
                }
            }
        });

        if self.close_file_attempt.is_active() {
            dialog_window("Close file without saving?").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Don't save").clicked() {
                        self.close_file_attempt.force();
                        self.attempt_close_file();
                    }
                    if ui.button("Cancel").clicked() {
                        self.close_file_attempt.give_up();
                    }
                    if ui.button("Save").clicked() {
                        self.file_save();
                        self.attempt_close_file();
                    }
                });
            });
        }
    }
}

fn dialog_window(title: &str) -> egui::Window {
    egui::Window::new(title)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
}

/// Preview hovering files:
fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|input_state| {
            let files = &input_state.raw.hovered_files;

            if files.len() > 1 {
                "Cannot open more than 1 file"
            } else {
                "Drag and drop file to open"
            }
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}
