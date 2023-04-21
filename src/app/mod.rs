/// Custom implementations for `App`
mod methods;
/// Render `App` with `eframe::App` implementation
mod render;

use std::sync::{Arc, Mutex};

use crate::{Attempt, Channel, File};

/// Possible messages between threads
enum ConcurrentMessage {
    /// Save has succeeded
    FinishSave,
}

/// Actions to allow after close attempt passes
enum CloseAction {
    NewFile,
    OpenFile,
}

/// Main app state
#[derive(Default)]
pub struct App {
    /// Current file opened
    file: File,

    /// Attempt to close file (See `Attempt`)
    file_close: Attempt<CloseAction>,

    /// Whether file is currently writing
    concurrent_write: Arc<Mutex<bool>>,

    /// Send messages between threads
    channel: Channel<ConcurrentMessage>,
}

// // @ debug
// impl Default for App {
//     fn default() -> Self {
//         Self {
//             channel: Default::default(),

//             file: File::open_path("/home/darcy/Documents/hello.txt").expect("Open initial file"),

//             concurrent_write: Default::default(),

//             file_close: Default::default(),
//         }
//     }
// }
