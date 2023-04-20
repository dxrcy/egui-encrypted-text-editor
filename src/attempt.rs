pub struct Attempt<T> {
    action: Option<T>,
    force: bool,
}

impl<T> Default for Attempt<T> {
    fn default() -> Self {
        Self {
            action: None,
            force: false,
        }
    }
}

impl<T> Attempt<T> {
    pub fn allow_if(&mut self, condition: bool, action: T) -> bool {
        if self.force || condition {
            *self = Self::default();
            true
        } else {
            self.action = Some(action);
            false
        }
    }

    pub fn active(&self) -> bool {
        self.action.is_some()
    }

    pub fn action(&self) -> &Option<T> {
        &self.action
    }

    pub fn force(&mut self) {
        self.force = true;
    }

    pub fn give_up(&mut self) {
        *self = Self::default()
    }
}
