use crate::app::samples::combatant_from_monster;
use crate::app::state::{AppState, CombatContext};
use crate::app::App;
use crate::game::{
    character::{conditions::Condition, Character},
    combat::{
        apply_damage, roll_attack, AttackProfile, CombatantState, DefenseProfile, HitType, RollMode,
    },
    dice::DiceExpr,
};
use std::collections::HashMap;

impl App {
    pub fn resolve_attack(
        ctx: &mut CombatContext,
        attacker_id: &str,
        target_id: &str,
        pdm: f32,
    ) -> bool {
        let (attacker_exists, target_exists) = (
            ctx.state.combatants.contains_key(attacker_id),
            ctx.state.combatants.contains_key(target_id),
        );
        if !attacker_exists || !target_exists {
            return false;
        }

        let (
            attacker_alive,
            attacker_action,
            attacker_name,
            attacker_attack_bonus,
            attacker_damage_dice,
            attacker_damage_type,
            attacker_conditions,
            attacker_on_hit_condition,
            attacker_is_player,
        ) = {
            let a = ctx.state.combatants.get(attacker_id).unwrap();
            (
                a.is_alive(),
                a.action_slots.action,
                a.name.clone(),
                a.attack_bonus,
                a.damage_dice.clone(),
                a.damage_type.clone(),
                a.conditions.clone(),
                a.on_hit_condition.clone(),
                a.is_player,
            )
        };

        if !attacker_alive {
            return false;
        }

        if !attacker_action {
            Self::push_log(ctx, format!("{} has no actions left.", attacker_name));
            return false;
        }

        let stop_cond = if attacker_conditions.contains(&Condition::Stunned) {
            Some("Stunned")
        } else if attacker_conditions.contains(&Condition::Paralyzed) {
            Some("Paralyzed")
        } else if attacker_conditions.contains(&Condition::Unconscious) {
            Some("Unconscious")
        } else if attacker_conditions.contains(&Condition::Incapacitated) {
            Some("Incapacitated")
        } else {
            None
        };

        if let Some(cond_name) = stop_cond {
            Self::push_log(
                ctx,
                format!("{} is {} and cannot act.", attacker_name, cond_name),
            );
            return false;
        }

        let (
            target_name,
            target_ac,
            target_conditions,
            target_resistances,
            target_vulnerabilities,
            target_immunities,
        ) = {
            let t = ctx.state.combatants.get(target_id).unwrap();
            (
                t.name.clone(),
                t.armor_class,
                t.conditions.clone(),
                t.resistances.clone(),
                t.vulnerabilities.clone(),
                t.immunities.clone(),
            )
        };

        let atk_profile = AttackProfile {
            id: attacker_id,
            attack_bonus: attacker_attack_bonus,
            is_ranged: false,
            damage_dice: &attacker_damage_dice,
            damage_type: &attacker_damage_type,
            conditions: &attacker_conditions,
            on_hit_condition: attacker_on_hit_condition,
        };

        let def_profile = DefenseProfile {
            id: target_id,
            armor_class: target_ac,
            conditions: &target_conditions,
            resistances: &target_resistances,
            vulnerabilities: &target_vulnerabilities,
            immunities: &target_immunities,
        };

        let result = roll_attack(&atk_profile, &def_profile, &ctx.world_state);
        ctx.seed = (ctx.seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;

        let final_damage = if attacker_is_player {
            (result.damage as f32 * pdm).max(0.0) as u32
        } else {
            result.damage
        };

        match result.hit_type {
            HitType::Critical => {
                Self::push_log(
                    ctx,
                    format!(
                        "CRITICAL HIT! {} deals {} {} damage to {}.",
                        attacker_name, final_damage, attacker_damage_type, target_name
                    ),
                );
            }
            HitType::Hit => {
                Self::push_log(
                    ctx,
                    format!(
                        "{} hits {} for {} {} damage.",
                        attacker_name, target_name, final_damage, attacker_damage_type
                    ),
                );
            }
            HitType::Miss => {
                Self::push_log(ctx, format!("{} misses {}.", attacker_name, target_name));
            }
        }

        if result.hit_type != HitType::Miss {
            if let Some(ref cond) = result.inflicted_condition {
                Self::push_log(ctx, format!("{} is now {:?}.", target_name, cond));
            }
            apply_damage(
                ctx.state.combatants.get_mut(target_id).unwrap(),
                final_damage,
            );
            if let Some(cond) = result.inflicted_condition {
                ctx.state
                    .combatants
                    .get_mut(target_id)
                    .unwrap()
                    .apply_condition(cond, Some(1));
            }
        }

        ctx.state
            .combatants
            .get_mut(attacker_id)
            .unwrap()
            .action_slots
            .use_attack_action();
        true
    }
    #[allow(clippy::too_many_arguments)]
    pub fn resolve_attack_with_stats(
        ctx: &mut CombatContext,
        attacker_id: &str,
        target_id: &str,
        attack_bonus: i32,
        is_ranged: bool,
        damage_dice: DiceExpr,
        damage_type: &str,
        on_hit_condition: Option<crate::game::character::conditions::Condition>,
        attack_label: &str,
    ) -> bool {
        let (attacker_exists, target_exists) = (
            ctx.state.combatants.contains_key(attacker_id),
            ctx.state.combatants.contains_key(target_id),
        );
        if !attacker_exists || !target_exists {
            return false;
        }

        let (attacker_name, attacker_conditions) = {
            let a = ctx.state.combatants.get(attacker_id).unwrap();
            (a.name.clone(), a.conditions.clone())
        };

        let (
            target_name,
            target_ac,
            target_conditions,
            target_resistances,
            target_vulnerabilities,
            target_immunities,
        ) = {
            let t = ctx.state.combatants.get(target_id).unwrap();
            (
                t.name.clone(),
                t.armor_class,
                t.conditions.clone(),
                t.resistances.clone(),
                t.vulnerabilities.clone(),
                t.immunities.clone(),
            )
        };

        let atk_profile = AttackProfile {
            id: attacker_id,
            attack_bonus,
            is_ranged,
            damage_dice: &damage_dice,
            damage_type,
            conditions: &attacker_conditions,
            on_hit_condition,
        };
        let def_profile = DefenseProfile {
            id: target_id,
            armor_class: target_ac,
            conditions: &target_conditions,
            resistances: &target_resistances,
            vulnerabilities: &target_vulnerabilities,
            immunities: &target_immunities,
        };

        let result = roll_attack(&atk_profile, &def_profile, &ctx.world_state);
        ctx.seed = (ctx.seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;

        match result.hit_type {
            HitType::Critical => {
                Self::push_log(
                    ctx,
                    format!(
                        "CRITICAL! {} {} {} for {} {} damage.",
                        attacker_name, attack_label, target_name, result.damage, damage_type
                    ),
                );
            }
            HitType::Hit => {
                Self::push_log(
                    ctx,
                    format!(
                        "{} {} {} for {} {} damage.",
                        attacker_name, attack_label, target_name, result.damage, damage_type
                    ),
                );
            }
            HitType::Miss => {
                Self::push_log(
                    ctx,
                    format!(
                        "{} {} {}, but misses.",
                        attacker_name, attack_label, target_name
                    ),
                );
            }
        }

        if result.hit_type != HitType::Miss {
            if let Some(ref cond) = result.inflicted_condition {
                Self::push_log(ctx, format!("{} is now {:?}.", target_name, cond));
            }
            apply_damage(
                ctx.state.combatants.get_mut(target_id).unwrap(),
                result.damage,
            );
            if let Some(cond) = result.inflicted_condition {
                ctx.state
                    .combatants
                    .get_mut(target_id)
                    .unwrap()
                    .apply_condition(cond, Some(1));
            }
        }
        true
    }
}
