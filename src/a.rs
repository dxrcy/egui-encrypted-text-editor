pub struct CurrentFile {
    path: String,
    contents: String,
    state: State,
}

enum State {
    Saved,
    Unsaved,
    Unregistered,
    // Writing,
}