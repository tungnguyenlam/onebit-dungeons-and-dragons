use super::types::*;
use crate::game::{
    character::conditions::Condition, combat::CombatantState, dice::DiceExpr,
    story::world_state::WorldState,
};
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
/// Roll a d20 attack: `d20 + attack_bonus >= AC`.
///
/// Natural 1 always misses. Natural 20 always crits (double damage dice).
pub fn roll_attack(
    attacker: &AttackProfile<'_>,
    target: &DefenseProfile<'_>,
    ws: &WorldState,
) -> AttackOutcome {
    let mut rng = rand::rng();
    roll_attack_with_rng(attacker, target, ws, &mut rng)
}

/// Deterministic helper for tests/replay.
pub fn roll_attack_with_seed(
    attacker: &AttackProfile<'_>,
    target: &DefenseProfile<'_>,
    ws: &WorldState,
    seed: u64,
) -> AttackOutcome {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    roll_attack_with_rng(attacker, target, ws, &mut rng)
}

/// Roll saving throw against DC.
pub fn roll_saving_throw(save_bonus: i32, dc: i32) -> SaveOutcome {
    let mut rng = rand::rng();
    roll_saving_throw_with_rng(save_bonus, dc, &mut rng)
}

fn roll_attack_with_rng<R: Rng + ?Sized>(
    attacker: &AttackProfile<'_>,
    target: &DefenseProfile<'_>,
    ws: &WorldState,
    rng: &mut R,
) -> AttackOutcome {
    let condition_disadv = attacker
        .conditions
        .iter()
        .any(Condition::imposes_attack_disadvantage);
    let flag_disadv = ws.flag("volcanic_curse_active");
    let fog_disadv = ws.flag("weather_fog") && attacker.is_ranged;
    let rain_fire_disadv = ws.flag("weather_rain") && attacker.damage_type == "fire";
    let attacker_disadv = condition_disadv || flag_disadv || fog_disadv || rain_fire_disadv;
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

    let rain_ranged_penalty = if ws.flag("weather_rain") && attacker.is_ranged {
        2
    } else {
        0
    };
    let total = d20 + attacker.attack_bonus - rain_ranged_penalty;

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
