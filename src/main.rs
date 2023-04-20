#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::emath::Align2;
use egui_files::{file_dialog, Attempt, File};

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

#[allow(dead_code)]
#[derive(Default)]
struct App {
    error: Option<&'static str>,

    file: File,

    close_attempt: Attempt,
}

impl App {
    pub fn attempt_file_close(&mut self) -> bool {
        println!("Attempt close");

        let is_able = self.close_attempt.allow_if(self.file.is_saved());
        if is_able {
            println!("Close");

            self.file = File::default();
        }
        is_able
    }

    fn file_save(&mut self) {
        println!("Save");

        // ? Remove clone
        if let Some(path) = self.file.clone().path() {
            self.file.save(&path).expect("Save file");
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

        if self.attempt_file_close() {
            if let Some(path) = file_dialog()
                .pick_file()
                .map(|path_buf| path_buf.display().to_string())
            {
                self.file = File::open_path(path).expect("Open file");
            };
        }
    }

    fn file_new(&mut self) {
        println!("New file");

        if self.attempt_file_close() {
            self.file = File::default();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Text file editor");

            ui.horizontal(|ui| {
                ui.monospace(self.file.path().unwrap_or(&"Unknown file".to_string()));

                ui.label(if self.file.is_saved() {
                    "Saved"
                } else {
                    "UNSAVED"
                });
            });

            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
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

        // preview_files_being_dropped(ctx);

        // // Collect dropped files:
        // ctx.input(|input_state| {
        //     let files = &input_state.raw.dropped_files;

        //     if files.len() == 1 {
        //         if let Some(file) = files.first() {
        //             if let Some(path) = &file.path {
        //                 self.file_open(&path.display().to_string())
        //             } else if !file.name.is_empty() {
        //                 self.file_open(&file.name)
        //             } else {
        //                 self.error = Some("Unable to open file");
        //             };
        //         }
        //     }
        // });

        if self.close_attempt.active() {
            dialog_window("Close file without saving?").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Don't save").clicked() {
                        self.close_attempt.force();
                        self.attempt_file_close();
                    }
                    if ui.button("Cancel").clicked() {
                        self.close_attempt.give_up();
                    }
                    if ui.button("Save").clicked() {
                        self.file_save();
                        self.attempt_file_close();
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

#[allow(dead_code)]
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
