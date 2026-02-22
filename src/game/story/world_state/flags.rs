use super::types::WorldState;

impl WorldState {

    // -----------------------------------------------------------------------
    // Flag helpers
    // -----------------------------------------------------------------------

    /// Set a flag to `true`.
    pub fn set_flag(&mut self, key: impl Into<String>) {
        self.flags.insert(key.into(), true);
    }

    /// Clear a flag (set to `false`).
    pub fn clear_flag(&mut self, key: impl Into<String>) {
        self.flags.insert(key.into(), false);
    }

    /// Return `true` if the flag is `true`; `false` if absent or `false`.
    pub fn flag(&self, key: &str) -> bool {
        *self.flags.get(key).unwrap_or(&false)
    }
}