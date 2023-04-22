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
enum CloseFileAction {
    NewFile,
    OpenFile,
    CloseWindow,
}

/// Main app state
// #[derive(Default)]
pub struct App {
    /// Current file opened
    file: File,

    /// Key for cryption
    key: String,

    /// Whether file is currently writing
    writing: Arc<Mutex<bool>>,

    /// Attempt to close file (See `Attempt`)
    attempting_file_close: Attempt<CloseFileAction>,

    /// Whether program window should be closed on next frame render
    close_window_on_next_frame: bool,

    /// Send messages between threads
    channel: Channel<ConcurrentMessage>,

    /// Display any error message
    error: Arc<Mutex<Option<&'static str>>>,
}

// @ debug
impl Default for App {
    fn default() -> Self {
        Self {
            // file: File::open_path("/home/darcy/Documents/hello.txt").expect("Open initial file"),
            file:Default::default(),
            
            key: "foo".to_string(),

            writing: Default::default(),

            attempting_file_close: Default::default(),

            close_window_on_next_frame: Default::default(),

            channel: Default::default(),

            error: Default::default(),
        }
    }
}
