#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use text_editor::App;


fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(1000.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Text file editor",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}
