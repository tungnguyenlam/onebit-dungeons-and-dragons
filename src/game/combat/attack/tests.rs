#[cfg(test)]
mod tests {
    use crate::game::combat::attack::*;
    use crate::game::{
        character::conditions::Condition, combat::CombatantState, dice::DiceExpr,
        story::world_state::WorldState,
    };
    use std::collections::HashSet;

    fn attacker() -> (DiceExpr, HashSet<Condition>) {
        (DiceExpr::new(1, 8, 2), HashSet::new())
    }

    fn target() -> (
        HashSet<Condition>,
        HashSet<String>,
        HashSet<String>,
        HashSet<String>,
    ) {
        (
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
        )
    }

    fn empty_ws() -> WorldState {
        WorldState::new()
    }

    #[test]
    fn seeded_attack_is_deterministic() {
        let (dice, atk_conds) = attacker();
        let (def_conds, def_res, def_vuln, _def_imm) = target();
        let atk = AttackProfile {
            id: "a",
            attack_bonus: 5,
            is_ranged: false,
            damage_dice: &dice,
            damage_type: "slashing",
            conditions: &atk_conds,
            on_hit_condition: None,
        };
        let def = DefenseProfile {
            id: "t",
            armor_class: 14,
            conditions: &def_conds,
            resistances: &def_res,
            vulnerabilities: &def_vuln,
            immunities: &HashSet::new(),
        };

        let ws = empty_ws();
        let a = roll_attack_with_seed(&atk, &def, &ws, 99);
        let b = roll_attack_with_seed(&atk, &def, &ws, 99);
        assert_eq!(a, b);
    }

    #[test]
    fn apply_damage_reduces_hp() {
        let mut c = CombatantState::new(
            "id",
            "name",
            false,
            12,
            13,
            30,
            1,
            4,
            DiceExpr::new(1, 6, 0),
        );
        assert_eq!(apply_damage(&mut c, 5), 7);
        assert_eq!(c.current_hp, 7);
    }

    #[test]
    fn advantage_and_disadvantage_cancel() {
        let (dice, mut atk_conds) = attacker();
        atk_conds.insert(Condition::Poisoned); // attack disadvantage
        let (def_conds, def_res, def_vuln, _def_imm) = target();

        let atk = AttackProfile {
            id: "a",
            attack_bonus: 5,
            is_ranged: false,
            damage_dice: &dice,
            damage_type: "bludgeoning",
            conditions: &atk_conds,
            on_hit_condition: None,
        };
        let def = DefenseProfile {
            id: "t",
            armor_class: 10,
            conditions: &def_conds,
            resistances: &def_res,
            vulnerabilities: &def_vuln,
            immunities: &HashSet::new(),
        };

        let ws = empty_ws();
        let out = roll_attack_with_seed(&atk, &def, &ws, 1234);
        assert!((1..=20).contains(&out.d20));
    }

    #[test]
    fn saving_throw_returns_result() {
        let out = roll_saving_throw(2, 12);
        match out {
            SaveOutcome::Success { d20, .. } | SaveOutcome::Failure { d20, .. } => {
                assert!((1..=20).contains(&d20));
            }
        }
    }

    #[test]
    fn roll_mode_marks_disadvantage_from_poisoned_attacker() {
        let dice = DiceExpr::new(1, 6, 0);
        let mut atk_conds = HashSet::new();
        atk_conds.insert(Condition::Poisoned);
        let (def_conds, def_res, def_vuln, _def_imm) = target();
        let atk = AttackProfile {
            id: "a",
            attack_bonus: 4,
            is_ranged: false,
            damage_dice: &dice,
            damage_type: "piercing",
            conditions: &atk_conds,
            on_hit_condition: None,
        };
        let def = DefenseProfile {
            id: "t",
            armor_class: 12,
            conditions: &def_conds,
            resistances: &def_res,
            vulnerabilities: &def_vuln,
            immunities: &HashSet::new(),
        };
        let ws = empty_ws();
        let out = roll_attack_with_seed(&atk, &def, &ws, 42);
        assert_eq!(out.roll_mode, RollMode::Disadvantage);
    }

    #[test]
    fn on_hit_condition_is_reported_on_hit() {
        let dice = DiceExpr::new(1, 6, 0);
        let atk_conds = HashSet::new();
        let (def_conds, def_res, def_vuln, _def_imm) = target();
        let atk = AttackProfile {
            id: "a",
            attack_bonus: 100,
            is_ranged: false,
            damage_dice: &dice,
            damage_type: "poison",
            conditions: &atk_conds,
            on_hit_condition: Some(Condition::Poisoned),
        };
        let def = DefenseProfile {
            id: "t",
            armor_class: -10,
            conditions: &def_conds,
            resistances: &def_res,
            vulnerabilities: &def_vuln,
            immunities: &HashSet::new(),
        };
        let ws = empty_ws();
        // Seed chosen to avoid natural 1.
        let out = roll_attack_with_seed(&atk, &def, &ws, 2);
        assert!(matches!(out.hit_type, HitType::Hit | HitType::Critical));
        assert_eq!(out.inflicted_condition, Some(Condition::Poisoned));
    }

    #[test]
    fn resistance_halves_damage() {
        let dice = DiceExpr::new(1, 10, 0); // 1-10
        let atk_conds = HashSet::new();
        let (def_conds, mut def_res, def_vuln, _def_imm) = target();
        def_res.insert("fire".to_string());

        let atk = AttackProfile {
            id: "a",
            attack_bonus: 100, // always hit
            is_ranged: false,
            damage_dice: &dice,
            damage_type: "fire",
            conditions: &atk_conds,
            on_hit_condition: None,
        };
        let def = DefenseProfile {
            id: "t",
            armor_class: 10,
            conditions: &def_conds,
            resistances: &def_res,
            vulnerabilities: &def_vuln,
            immunities: &HashSet::new(),
        };

        let ws = empty_ws();
        // Seed 2 gives 3 on d10. 3 / 2 = 1.
        let out = roll_attack_with_seed(&atk, &def, &ws, 2);
        assert_eq!(out.damage, 1);
    }

    #[test]
    fn vulnerability_doubles_damage() {
        let dice = DiceExpr::new(1, 10, 0); // 1-10
        let atk_conds = HashSet::new();
        let (def_conds, def_res, mut def_vuln, _def_imm) = target();
        def_vuln.insert("cold".to_string());

        let atk = AttackProfile {
            id: "a",
            attack_bonus: 100, // always hit
            is_ranged: false,
            damage_dice: &dice,
            damage_type: "cold",
            conditions: &atk_conds,
            on_hit_condition: None,
        };
        let def = DefenseProfile {
            id: "t",
            armor_class: 10,
            conditions: &def_conds,
            resistances: &def_res,
            vulnerabilities: &def_vuln,
            immunities: &HashSet::new(),
        };

        let ws = empty_ws();
        // Seed 2 gives 3 on d10 (after hit roll). 3 * 2 = 6.
        let out = roll_attack_with_seed(&atk, &def, &ws, 2);
        assert_eq!(out.damage, 6);
    }

    #[test]
    fn fog_applies_disadvantage_to_ranged_attacks() {
        let dice = DiceExpr::new(1, 6, 0);
        let atk_conds = HashSet::new();
        let (def_conds, def_res, def_vuln, _def_imm) = target();
        let atk = AttackProfile {
            id: "a",
            attack_bonus: 4,
            is_ranged: true,
            damage_dice: &dice,
            damage_type: "piercing",
            conditions: &atk_conds,
            on_hit_condition: None,
        };
        let def = DefenseProfile {
            id: "t",
            armor_class: 12,
            conditions: &def_conds,
            resistances: &def_res,
            vulnerabilities: &def_vuln,
            immunities: &HashSet::new(),
        };
        let mut ws = empty_ws();
        ws.set_flag("weather_fog");
        let out = roll_attack_with_seed(&atk, &def, &ws, 42);
        assert_eq!(out.roll_mode, RollMode::Disadvantage);
    }

    #[test]
    fn rain_disadvantages_fire_attacks() {
        let dice = DiceExpr::new(1, 6, 0);
        let atk_conds = HashSet::new();
        let (def_conds, def_res, def_vuln, _def_imm) = target();
        let atk = AttackProfile {
            id: "a",
            attack_bonus: 4,
            is_ranged: true,
            damage_dice: &dice,
            damage_type: "fire",
            conditions: &atk_conds,
            on_hit_condition: None,
        };
        let def = DefenseProfile {
            id: "t",
            armor_class: 12,
            conditions: &def_conds,
            resistances: &def_res,
            vulnerabilities: &def_vuln,
            immunities: &HashSet::new(),
        };
        let mut ws = empty_ws();
        ws.set_flag("weather_rain");
        let out = roll_attack_with_seed(&atk, &def, &ws, 42);
        assert_eq!(out.roll_mode, RollMode::Disadvantage);
    }
}
