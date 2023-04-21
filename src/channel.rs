use std::sync::mpsc::{channel, Receiver, Sender};

/// Wrapper for `Sender` and `Receiver` types in `std::sync::mpsc`
pub struct Channel<T> {
    pub sender: Sender<T>,
    pub receiver: Receiver<T>,
}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self { sender, receiver }
    }
}
