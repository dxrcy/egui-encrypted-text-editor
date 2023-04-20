use std::{fs, io};

#[allow(dead_code)]
#[derive(Clone, Copy, Default)]
enum State {
    Saved,
    #[default]
    Unsaved,
}

#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct File {
    path: Option<String>,
    contents: String,
    //todo convert to boolean
    state: State,
}

impl File {
    pub fn is_saved_or_not_changed(&self) -> bool {
        if self.path.is_some() {
            matches!(self.state, State::Saved)
        } else {
            self.contents.is_empty()
        }
    }

    pub fn is_saved(&self) -> bool {
        matches!(self.state, State::Saved)
    }

    pub fn is_changed(&self) -> bool {
        if self.is_saved() {
            return false;
        }
        self.path().is_some() || !self.contents().is_empty()
    }

    pub fn contents(&self) -> &String {
        &self.contents
    }

    pub fn contents_mut(&mut self) -> &mut String {
        &mut self.contents
    }

    pub fn mark_as_unsaved(&mut self) {
        if self.is_saved() {
            self.state = State::Unsaved;
        }
    }

    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    pub fn set_path(&mut self, path: impl Into<String>) {
        self.path = Some(path.into())
    }

    pub fn save(&mut self, path: &str) -> io::Result<()> {
        println!("{:?}", self.contents);
        fs::write(path, &self.contents)?;
        self.state = State::Saved;
        Ok(())
    }

    pub fn open_path(path: impl Into<String>) -> io::Result<Self> {
        let path = path.into();

        let contents = fs::read_to_string(&path)?;

        Ok(Self {
            contents,
            path: Some(path),
            state: State::Saved,
        })
    }
}
