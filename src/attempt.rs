/// 'Attempt' something, such as close a file
///
/// Takes enum of possible actions to allow
/// 
/// Useful for *"Exit without saving?"* dialogs
pub struct Attempt<T> {
    /// Action to run, if accepted
    ///
    /// `None` if attempt is not active (No attempt is being made)
    action: Option<T>,
    /// Whether attempt has been overridden
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
    /// If condition is met, or attempt is forced, return `true` and reset `self`
    ///
    /// Otherwise, return `false` and save action for next attempt
    pub fn allow_if(&mut self, condition: bool, action: T) -> bool {
        if self.force || condition {
            self.stop_attempt();
            true
        } else {
            self.action = Some(action);
            false
        }
    }

    /// Returns `true` if an attempt is being made
    pub fn is_attempting(&self) -> bool {
        self.action.is_some()
    }

    /// Reference to action to run (enum variant)
    pub fn action(&self) -> &Option<T> {
        &self.action
    }

    /// Override any condition, allow action to run on next attempt
    pub fn force(&mut self) {
        self.force = true;
    }

    /// Reset `self` and stop attempt
    pub fn stop_attempt(&mut self) {
        *self = Self::default()
    }
}
