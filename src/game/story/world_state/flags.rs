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

    // -----------------------------------------------------------------------
    // Discovery helpers (Bestiary & Lore Library)
    // -----------------------------------------------------------------------

    /// Discover a monster (marks it as discovered in bestiary).
    pub fn discover_monster(&mut self, monster_id: &str) {
        if self.discovered_monsters.insert(monster_id.to_string()) {
            // Keep a legacy flag for backward-compatible condition checks.
            self.set_flag(format!("monster_discovered:{}", monster_id));
            let count_key = "bestiary_progress";
            self.delta_counter(count_key, 1);
        }
    }

    /// Check if a monster has been discovered.
    pub fn is_monster_discovered(&self, monster_id: &str) -> bool {
        self.discovered_monsters.contains(monster_id)
            || self.flag(&format!("monster_discovered:{}", monster_id))
    }

    /// Get all discovered monster IDs.
    pub fn discovered_monsters(&self) -> Vec<String> {
        let mut out: Vec<String> = self.discovered_monsters.iter().cloned().collect();
        for key in self.flags.keys() {
            if key.starts_with("monster_discovered:") && self.flag(key) {
                out.push(key.replace("monster_discovered:", ""));
            }
        }
        out.sort();
        out.dedup();
        out
    }

    /// Discover a lore entry.
    pub fn discover_lore(&mut self, lore_id: &str) {
        if self.discovered_lore.insert(lore_id.to_string()) {
            // Keep a legacy flag for backward-compatible condition checks.
            self.set_flag(format!("lore_discovered:{}", lore_id));
        }
    }

    /// Check if a lore entry has been discovered.
    pub fn is_lore_discovered(&self, lore_id: &str) -> bool {
        self.discovered_lore.contains(lore_id) || self.flag(&format!("lore_discovered:{}", lore_id))
    }

    /// Get all discovered lore IDs.
    pub fn discovered_lore(&self) -> Vec<String> {
        let mut out: Vec<String> = self.discovered_lore.iter().cloned().collect();
        for key in self.flags.keys() {
            if key.starts_with("lore_discovered:") && self.flag(key) {
                out.push(key.replace("lore_discovered:", ""));
            }
        }
        out.sort();
        out.dedup();
        out
    }

    /// Increment kill counter for a monster id.
    pub fn register_monster_kill(&mut self, monster_id: &str) {
        self.delta_counter(format!("monster_kills:{}", monster_id), 1);
    }

    /// Return kills for a specific monster id.
    pub fn monster_kill_count(&self, monster_id: &str) -> i32 {
        self.counter(&format!("monster_kills:{}", monster_id))
    }
}
