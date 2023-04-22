/// Custom implementations for `App`
mod methods;
/// Render `App` with `eframe::App` implementation
mod render;

use std::sync::{Arc, Mutex};

use crate::{Channel, File, Attempt};

/// Possible messages between threads
enum ConcurrentMessage {
    /// Save has succeeded
    FinishConcurrentSave,
}

/// Actions to allow after close attempt passes
enum Action {
    NewFile,
    OpenFile,
    CloseWindow,
}

/// Main app state
#[derive(Default)]
pub struct App {
    /// Current file opened
    file: File,

    /// Whether file is currently writing
    writing: Arc<Mutex<bool>>,

    /// Attempt to close file (See `Attempt`)
    attempting_file_close: Attempt<Action>,

    close_window_on_next_frame: bool,

    /// Send messages between threads
    channel: Channel<ConcurrentMessage>,
}

// // @ debug
// impl Default for App {
//     fn default() -> Self {
//         Self {
//             file: File::open_path("/home/darcy/Documents/hello.txt").expect("Open initial file"),

//             writing: Default::default(),

//             attempting_file_close: Default::default(),

//             close_window_on_next_frame: Default::default(),

//             channel: Default::default(),
//         }
//     }
// }
