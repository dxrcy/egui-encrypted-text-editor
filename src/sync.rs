use std::sync::mpsc;

/// Possible messages between threads
pub enum Message {
    /// Save has succeeded
    FinishSave,
}


pub type Sender = mpsc::Sender<Message>;
pub type Receiver = mpsc::Receiver<Message>;

pub struct Channel {
    pub tx: Sender,
    pub rx: Receiver,
}

impl Default for Channel {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { tx, rx }
    }
}
