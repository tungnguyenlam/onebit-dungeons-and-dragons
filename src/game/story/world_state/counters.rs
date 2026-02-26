use super::types::WorldState;

impl WorldState {
    // -----------------------------------------------------------------------
    // Counter helpers
    // -----------------------------------------------------------------------

    /// Get the current value of a counter (0 if absent).
    pub fn counter(&self, key: &str) -> i32 {
        *self.counters.get(key).unwrap_or(&0)
    }

    /// Set a counter to an exact value.
    pub fn set_counter(&mut self, key: impl Into<String>, value: i32) {
        self.counters.insert(key.into(), value);
    }

    /// Add `delta` to a counter (creates it with value 0 first if absent).
    pub fn delta_counter(&mut self, key: impl Into<String>, delta: i32) {
        let key = key.into();
        let v = self.counters.entry(key).or_insert(0);
        *v += delta;
    }
}
