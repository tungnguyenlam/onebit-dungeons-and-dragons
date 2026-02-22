use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldState {
    /// Boolean flags, e.g. `"killed_bandit_lord"` → `true`.
    pub flags: HashMap<String, bool>,
    /// Integer counters, e.g. `"faction_guild_rep"` → `12`.
    pub counters: HashMap<String, i32>,
}

impl WorldState {
    /// Create an empty `WorldState`.
    pub fn new() -> Self {
        Self::default()
    }
}