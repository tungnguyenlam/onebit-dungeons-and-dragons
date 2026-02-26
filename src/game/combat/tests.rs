#[cfg(test)]
mod tests {
    use crate::game::character::conditions::Condition;
    use crate::game::combat::combatant::*;
    use crate::game::combat::state::*;
    use crate::game::dice::DiceExpr;

    fn actor(id: &str, is_player: bool, init_mod: i32) -> CombatantState {
        CombatantState::new(
            id,
            id,
            is_player,
            10,
            14,
            30,
            init_mod,
            4,
            DiceExpr::new(1, 6, 2),
        )
    }

    #[test]
    fn next_turn_advances_and_wraps_round() {
        let mut c = CombatState::new_with_seed(
            vec![
                actor("p1", true, 2),
                actor("m1", false, 1),
                actor("m2", false, 0),
            ],
            42,
        );

        let start = c.current_combatant_id().unwrap().to_string();
        c.next_turn();
        let second = c.current_combatant_id().unwrap().to_string();
        assert_ne!(start, second);

        let round_before = c.round;
        c.next_turn();
        c.next_turn(); // wraps
        assert!(c.round >= round_before + 1);
    }

    #[test]
    fn next_turn_skips_dead_combatants() {
        let mut c =
            CombatState::new_with_seed(vec![actor("p1", true, 0), actor("m1", false, 0)], 7);
        c.combatants.get_mut("m1").unwrap().current_hp = 0;
        for _ in 0..5 {
            let id = c.next_turn().unwrap();
            assert_ne!(id, "m1");
        }
    }

    #[test]
    fn can_take_actions_false_when_stunned() {
        let mut c = actor("p1", true, 1);
        c.conditions.insert(Condition::Stunned);
        assert!(!c.can_take_actions());
    }

    #[test]
    fn next_enemy_prefers_opposing_side() {
        let c = CombatState::new_with_seed(
            vec![
                actor("p1", true, 2),
                actor("m1", false, 1),
                actor("m2", false, 0),
            ],
            42,
        );
        assert!(matches!(c.next_enemy_id("p1"), Some("m1" | "m2")));
    }

    #[test]
    fn timed_condition_expires_after_tick() {
        let mut c = actor("p1", true, 1);
        c.apply_condition(Condition::Poisoned, Some(1));
        assert!(c.conditions.contains(&Condition::Poisoned));
        let expired = c.tick_condition_durations();
        assert_eq!(expired, vec![Condition::Poisoned]);
        assert!(!c.conditions.contains(&Condition::Poisoned));
    }
}
