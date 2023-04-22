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
    pub overridden: bool,
}

impl<T> Default for Attempt<T> {
    fn default() -> Self {
        Self {
            action: None,
            overridden: false,
        }
    }
}

impl<T> Attempt<T> {
    /// Returns `true` if condition is met or overridden
    pub fn check_condition(&self, condition: bool) -> bool {
        println!(
            "{} || {} == {}",
            condition,
            self.overridden,
            condition || self.overridden
        );
        condition || self.overridden
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
    pub fn override_condition(&mut self) {
        self.overridden = true;
    }

    /// Reset `self` and stop attempt
    pub fn reset_attempt(&mut self) {
        *self = Self::default()
    }

    /// Sets action
    pub fn set_action(&mut self, action: T) {
        self.action = Some(action)
    }
}
