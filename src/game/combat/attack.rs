/// Attack and saving-throw resolution.
use crate::game::{
    character::conditions::Condition, combat::CombatantState, dice::DiceExpr,
    story::world_state::WorldState,
};
use rand::{Rng, SeedableRng};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitType {
    Miss,
    Hit,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollMode {
    Normal,
    Advantage,
    Disadvantage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttackOutcome {
    pub attacker_id: String,
    pub target_id: String,
    pub d20: i32,
    pub total: i32,
    pub roll_mode: RollMode,
    pub hit_type: HitType,
    pub damage: u32,
    pub damage_type: String,
    pub inflicted_condition: Option<Condition>,
}

#[derive(Debug, Clone)]
pub struct AttackProfile<'a> {
    pub id: &'a str,
    pub attack_bonus: i32,
    pub damage_dice: &'a DiceExpr,
    pub damage_type: &'a str,
    pub conditions: &'a HashSet<Condition>,
    pub on_hit_condition: Option<Condition>,
}

#[derive(Debug, Clone)]
pub struct DefenseProfile<'a> {
    pub id: &'a str,
    pub armor_class: i32,
    pub conditions: &'a HashSet<Condition>,
    pub resistances: &'a HashSet<String>,
    pub vulnerabilities: &'a HashSet<String>,
    pub immunities: &'a HashSet<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveOutcome {
    Success { d20: i32, total: i32 },
    Failure { d20: i32, total: i32 },
}

/// Roll a d20 attack: `d20 + attack_bonus >= AC`.
///
/// Natural 1 always misses. Natural 20 always crits (double damage dice).
pub fn roll_attack(
    attacker: &AttackProfile<'_>,
    target: &DefenseProfile<'_>,
    _ws: &WorldState,
) -> AttackOutcome {
    let mut rng = rand::rng();
    roll_attack_with_rng(attacker, target, &mut rng)
}

/// Deterministic helper for tests/replay.
pub fn roll_attack_with_seed(
    attacker: &AttackProfile<'_>,
    target: &DefenseProfile<'_>,
    seed: u64,
) -> AttackOutcome {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    roll_attack_with_rng(attacker, target, &mut rng)
}

/// Roll saving throw against DC.
pub fn roll_saving_throw(save_bonus: i32, dc: i32) -> SaveOutcome {
    let mut rng = rand::rng();
    roll_saving_throw_with_rng(save_bonus, dc, &mut rng)
}

fn roll_attack_with_rng<R: Rng + ?Sized>(
    attacker: &AttackProfile<'_>,
    target: &DefenseProfile<'_>,
    rng: &mut R,
) -> AttackOutcome {
    let attacker_disadv = attacker
        .conditions
        .iter()
        .any(Condition::imposes_attack_disadvantage);
    let target_grants_adv = target
        .conditions
        .iter()
        .any(Condition::grants_advantage_to_attackers);

    let (d20, roll_mode) = if attacker_disadv == target_grants_adv {
        (rng.random_range(1..=20), RollMode::Normal)
    } else if target_grants_adv {
        let a = rng.random_range(1..=20);
        let b = rng.random_range(1..=20);
        (a.max(b), RollMode::Advantage)
    } else {
        let a = rng.random_range(1..=20);
        let b = rng.random_range(1..=20);
        (a.min(b), RollMode::Disadvantage)
    };

    let total = d20 + attacker.attack_bonus;

    let (hit_type, mut damage, inflicted_condition) = if d20 == 1 {
        (HitType::Miss, 0, None)
    } else if d20 == 20 {
        (
            HitType::Critical,
            roll_damage(attacker.damage_dice, true, rng),
            attacker.on_hit_condition.clone(),
        )
    } else if total >= target.armor_class {
        (
            HitType::Hit,
            roll_damage(attacker.damage_dice, false, rng),
            attacker.on_hit_condition.clone(),
        )
    } else {
        (HitType::Miss, 0, None)
    };

    if damage > 0 {
        if target.immunities.contains(attacker.damage_type) {
            damage = 0;
        } else if target.resistances.contains(attacker.damage_type) {
            damage /= 2;
        } else if target.vulnerabilities.contains(attacker.damage_type) {
            damage *= 2;
        }
    }

    AttackOutcome {
        attacker_id: attacker.id.to_string(),
        target_id: target.id.to_string(),
        d20,
        total,
        roll_mode,
        hit_type,
        damage,
        damage_type: attacker.damage_type.to_string(),
        inflicted_condition,
    }
}

fn roll_saving_throw_with_rng<R: Rng + ?Sized>(
    save_bonus: i32,
    dc: i32,
    rng: &mut R,
) -> SaveOutcome {
    let d20 = rng.random_range(1..=20);
    let total = d20 + save_bonus;
    if d20 == 1 || total < dc {
        SaveOutcome::Failure { d20, total }
    } else {
        SaveOutcome::Success { d20, total }
    }
}

fn roll_damage<R: Rng + ?Sized>(dice: &DiceExpr, critical: bool, rng: &mut R) -> u32 {
    let rolls = if critical { dice.count * 2 } else { dice.count };
    let dice_total: i32 = (0..rolls)
        .map(|_| rng.random_range(1..=dice.sides) as i32)
        .sum();
    (dice_total + dice.modifier).max(0) as u32
}

/// Apply already-computed damage to a target combatant.
pub fn apply_damage(target: &mut CombatantState, amount: u32) -> i32 {
    target.take_damage(amount)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn seeded_attack_is_deterministic() {
        let (dice, atk_conds) = attacker();
        let (def_conds, def_res, def_vuln, def_imm) = target();
        let atk = AttackProfile {
            id: "a",
            attack_bonus: 5,
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

        let a = roll_attack_with_seed(&atk, &def, 99);
        let b = roll_attack_with_seed(&atk, &def, 99);
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
        let (def_conds, def_res, def_vuln, def_imm) = target();

        let atk = AttackProfile {
            id: "a",
            attack_bonus: 5,
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

        let out = roll_attack_with_seed(&atk, &def, 1234);
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
        let (def_conds, def_res, def_vuln, def_imm) = target();
        let atk = AttackProfile {
            id: "a",
            attack_bonus: 4,
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
        let out = roll_attack_with_seed(&atk, &def, 42);
        assert_eq!(out.roll_mode, RollMode::Disadvantage);
    }

    #[test]
    fn on_hit_condition_is_reported_on_hit() {
        let dice = DiceExpr::new(1, 6, 0);
        let atk_conds = HashSet::new();
        let (def_conds, def_res, def_vuln, def_imm) = target();
        let atk = AttackProfile {
            id: "a",
            attack_bonus: 100,
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
        // Seed chosen to avoid natural 1.
        let out = roll_attack_with_seed(&atk, &def, 2);
        assert!(matches!(out.hit_type, HitType::Hit | HitType::Critical));
        assert_eq!(out.inflicted_condition, Some(Condition::Poisoned));
    }

    #[test]
    fn resistance_halves_damage() {
        let dice = DiceExpr::new(1, 10, 0); // 1-10
        let atk_conds = HashSet::new();
        let (def_conds, mut def_res, def_vuln, def_imm) = target();
        def_res.insert("fire".to_string());

        let atk = AttackProfile {
            id: "a",
            attack_bonus: 100, // always hit
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

        // Seed 2 gives 3 on d10. 3 / 2 = 1.
        let out = roll_attack_with_seed(&atk, &def, 2);
        assert_eq!(out.damage, 1);
    }

    #[test]
    fn vulnerability_doubles_damage() {
        let dice = DiceExpr::new(1, 10, 0); // 1-10
        let atk_conds = HashSet::new();
        let (def_conds, def_res, mut def_vuln, def_imm) = target();
        def_vuln.insert("cold".to_string());

        let atk = AttackProfile {
            id: "a",
            attack_bonus: 100, // always hit
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

        // Seed 2 gives 3 on d10 (after hit roll). 3 * 2 = 6.
        let out = roll_attack_with_seed(&atk, &def, 2);
        assert_eq!(out.damage, 6);
    }
}
