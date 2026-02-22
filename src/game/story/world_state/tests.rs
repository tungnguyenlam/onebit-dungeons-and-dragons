#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::story::WorldState;

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

    #[test]
    fn complex_condition_evaluation() {
        let mut ws = WorldState::new();
        ws.set_flag("has_eye");
        ws.set_flag("has_heart");
        ws.set_counter("rep", 10);

        // A && B && C
        assert!(ws.evaluate("flag:has_eye && flag:has_heart && counter:rep >= 10"));
        // A && B || D (&& binds tighter)
        assert!(ws.evaluate("flag:has_eye && flag:has_heart || flag:never"));
        // A || B (where B is true)
        assert!(ws.evaluate("flag:never || counter:rep == 10"));
    }
}