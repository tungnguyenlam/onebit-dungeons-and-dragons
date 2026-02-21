/// `WorldState` — the central flag and counter store for the story system.
///
/// `WorldState` is the single shared source of truth for all story-relevant
/// information: which quests have been started, which NPCs have been met,
/// faction reputation counters, etc.
///
/// Story modules (quest machine, dialog evaluator, journal) **read** from
/// `WorldState` and **write** to it via the public helpers defined here.
/// They never mutate character stats directly; they fire events that the
/// game loop routes to the appropriate sub-system.
///
/// ## Condition string mini-language
///
/// Conditions appear as TOML strings in quest stages, dialog nodes, and
/// trigger definitions. `WorldState::evaluate` parses and evaluates them.
///
/// | Syntax                | Meaning                                  |
/// |-----------------------|------------------------------------------|
/// | `flag:key`            | `flags["key"] == true`                   |
/// | `not flag:key`        | `flags["key"] != true`                   |
/// | `counter:key >= N`    | `counters["key"] >= N`                   |
/// | `counter:key > N`     | `counters["key"] > N`                    |
/// | `counter:key <= N`    | `counters["key"] <= N`                   |
/// | `counter:key < N`     | `counters["key"] < N`                    |
/// | `counter:key == N`    | `counters["key"] == N`                   |
/// | `A && B`              | both sub-conditions must be true         |
/// | `A \|\| B`            | at least one sub-condition must be true  |
/// | `` (empty string)     | always true                              |
///
/// Sub-expressions are evaluated left-to-right; `&&` binds tighter than `||`.
///
/// See [docs/gameplay/story.md] for the full specification.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// WorldState
// ---------------------------------------------------------------------------

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

    // -----------------------------------------------------------------------
    // Condition evaluator
    // -----------------------------------------------------------------------

    /// Evaluate a condition string against the current state.
    ///
    /// Returns `true` for an empty condition (unconditional).
    ///
    /// `||` is split first (lowest precedence); each branch is then split on
    /// `&&`.  Individual predicates are parsed and evaluated.
    pub fn evaluate(&self, condition: &str) -> bool {
        let cond = condition.trim();
        if cond.is_empty() {
            return true;
        }
        // Split on `||` (lowest precedence).
        let or_branches: Vec<&str> = cond.split("||").collect();
        for branch in or_branches {
            // Each branch must satisfy ALL of its `&&` sub-expressions.
            if branch
                .split("&&")
                .map(str::trim)
                .all(|pred| self.eval_predicate(pred))
            {
                return true;
            }
        }
        false
    }

    /// Evaluate a single predicate atom (no `&&` or `||`).
    fn eval_predicate(&self, pred: &str) -> bool {
        let pred = pred.trim();

        // --- `not flag:key` ---
        if let Some(rest) = pred.strip_prefix("not flag:") {
            return !self.flag(rest.trim());
        }

        // --- `flag:key` ---
        if let Some(rest) = pred.strip_prefix("flag:") {
            return self.flag(rest.trim());
        }

        // --- `counter:key OP N` ---
        if let Some(rest) = pred.strip_prefix("counter:") {
            return self.eval_counter_pred(rest.trim());
        }

        // Unknown predicates are treated as false (fail-safe).
        false
    }

    /// Parse and evaluate `"key OP N"` where OP ∈ { >=, >, <=, <, == }.
    fn eval_counter_pred(&self, expr: &str) -> bool {
        // Try each operator longest first to avoid prefix ambiguity.
        for op in &[">=", "<=", "==", ">", "<"] {
            if let Some(pos) = expr.find(op) {
                let key = expr[..pos].trim();
                let n_str = expr[pos + op.len()..].trim();
                if let Ok(n) = n_str.parse::<i32>() {
                    let val = self.counter(key);
                    return match *op {
                        ">=" => val >= n,
                        "<=" => val <= n,
                        "==" => val == n,
                        ">" => val > n,
                        "<" => val < n,
                        _ => false,
                    };
                }
            }
        }
        false
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_condition_is_always_true() {
        let ws = WorldState::new();
        assert!(ws.evaluate(""));
        assert!(ws.evaluate("   "));
    }

    #[test]
    fn flag_set_and_check() {
        let mut ws = WorldState::new();
        assert!(!ws.flag("found_bandits"));
        ws.set_flag("found_bandits");
        assert!(ws.flag("found_bandits"));
    }

    #[test]
    fn flag_condition() {
        let mut ws = WorldState::new();
        ws.set_flag("met_kael");
        assert!(ws.evaluate("flag:met_kael"));
        assert!(!ws.evaluate("not flag:met_kael"));
        assert!(!ws.evaluate("flag:other"));
        assert!(ws.evaluate("not flag:other"));
    }

    #[test]
    fn counter_conditions() {
        let mut ws = WorldState::new();
        ws.set_counter("rep", 10);
        assert!(ws.evaluate("counter:rep >= 10"));
        assert!(ws.evaluate("counter:rep >  9"));
        assert!(!ws.evaluate("counter:rep >  10"));
        assert!(ws.evaluate("counter:rep <= 10"));
        assert!(!ws.evaluate("counter:rep <  10"));
        assert!(ws.evaluate("counter:rep == 10"));
        assert!(!ws.evaluate("counter:rep == 9"));
    }

    #[test]
    fn counter_default_zero() {
        let ws = WorldState::new();
        assert!(ws.evaluate("counter:nonexistent == 0"));
        assert!(!ws.evaluate("counter:nonexistent > 0"));
    }

    #[test]
    fn and_condition() {
        let mut ws = WorldState::new();
        ws.set_flag("met_kael");
        ws.set_counter("rep", 5);
        assert!(ws.evaluate("flag:met_kael && counter:rep >= 5"));
        assert!(!ws.evaluate("flag:met_kael && counter:rep >= 6"));
    }

    #[test]
    fn or_condition() {
        let mut ws = WorldState::new();
        ws.set_flag("a");
        assert!(ws.evaluate("flag:a || flag:b"));
        assert!(!ws.evaluate("flag:b || flag:c"));
    }

    #[test]
    fn delta_counter() {
        let mut ws = WorldState::new();
        ws.delta_counter("rep", 3);
        ws.delta_counter("rep", -1);
        assert_eq!(ws.counter("rep"), 2);
    }

    #[test]
    fn clear_flag() {
        let mut ws = WorldState::new();
        ws.set_flag("x");
        ws.clear_flag("x");
        assert!(!ws.flag("x"));
    }

    #[test]
    fn faction_rep_helpers() {
        let mut ws = WorldState::new();
        assert_eq!(ws.faction_rep("town_guard"), 0);
        ws.set_faction_rep("town_guard", 2);
        let val = ws.modify_faction_rep("town_guard", 3);
        assert_eq!(val, 5);
        assert_eq!(ws.faction_rep("town_guard"), 5);
    }
}
