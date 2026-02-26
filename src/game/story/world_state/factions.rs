use super::types::WorldState;

impl WorldState {
    // -----------------------------------------------------------------------
    // Faction reputation helpers
    // -----------------------------------------------------------------------

    pub fn faction_key(faction: &str) -> String {
        format!("faction_{}_rep", faction)
    }

    pub fn faction_rep(&self, faction: &str) -> i32 {
        self.counter(&Self::faction_key(faction))
    }

    pub fn set_faction_rep(&mut self, faction: &str, value: i32) {
        self.set_counter(Self::faction_key(faction), value);
    }

    /// Modify faction reputation by `delta`. Returns the new value.
    pub fn modify_faction_rep(&mut self, faction: &str, delta: i32) -> i32 {
        self.delta_counter(Self::faction_key(faction), delta);
        self.faction_rep(faction)
    }
}
