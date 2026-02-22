use crate::app::App;
use crate::app::samples::combatant_from_monster;
use crate::app::state::{AppState, CombatContext};
use crate::game::{
    character::{conditions::Condition, Character},
    combat::{
        apply_damage, roll_attack, AttackProfile, CombatantState, DefenseProfile, HitType, RollMode,
    },
    dice::DiceExpr,
};
use std::collections::HashMap;

impl App {
    pub fn run_enemy_turns(&mut self) {
        let AppState::Combat(ctx) = &mut self.state else {
            return;
        };

        while !ctx.state.is_over() {
            let Some(current_id) = ctx.state.current_combatant_id().map(str::to_string) else {
                break;
            };
            let is_player = ctx
                .state
                .combatants
                .get(&current_id)
                .map(|c| c.is_player)
                .unwrap_or(false);
            if is_player {
                break;
            }

            let (is_alive, is_incapacitated, name, role) = {
                let combatant = ctx.state.combatants.get(&current_id).unwrap();
                let incapacitated = combatant.conditions.contains(&Condition::Stunned)
                    || combatant.conditions.contains(&Condition::Paralyzed)
                    || combatant.conditions.contains(&Condition::Incapacitated)
                    || combatant.conditions.contains(&Condition::Unconscious);
                (
                    combatant.is_alive(),
                    incapacitated,
                    combatant.name.clone(),
                    combatant.enemy_role,
                )
            };

            if !is_alive {
                Self::advance_turn(ctx);
                continue;
            }

            if is_incapacitated {
                Self::push_log(
                    ctx,
                    format!("{} is incapacitated and skips their turn.", name),
                );
                Self::advance_turn(ctx);
                continue;
            }

            let acted = match role {
                crate::game::combat::EnemyAiRole::Spellcaster => {
                    Self::try_spellcaster_support_action(ctx, &current_id)
                }
                _ => false,
            };

            if !acted {
                let Some(target_id) = Self::select_enemy_target(ctx, &current_id, true) else {
                    Self::advance_turn(ctx);
                    continue;
                };

                match role {
                    crate::game::combat::EnemyAiRole::Melee => {
                        Self::resolve_attack(ctx, &current_id, &target_id, 1.0);
                    }
                    crate::game::combat::EnemyAiRole::Ranged => {
                        let (bonus, dice, damage_type, cond) = {
                            let c = ctx.state.combatants.get(&current_id).unwrap();
                            (
                                c.ranged_attack_bonus.unwrap_or(2),
                                c.ranged_damage_dice
                                    .clone()
                                    .unwrap_or_else(|| DiceExpr::new(1, 6, 0)),
                                c.ranged_damage_type
                                    .clone()
                                    .unwrap_or_else(|| "piercing".to_string()),
                                c.ranged_on_hit_condition.clone(),
                            )
                        };
                        Self::resolve_attack_with_stats(
                            ctx,
                            &current_id,
                            &target_id,
                            bonus,
                            dice,
                            &damage_type,
                            cond,
                            "shoots",
                        );
                    }
                    crate::game::combat::EnemyAiRole::Spellcaster => {
                        let (bonus, dice, damage_type, cond) = {
                            let c = ctx.state.combatants.get(&current_id).unwrap();
                            (
                                c.spell_attack_bonus.unwrap_or(4),
                                c.spell_damage_dice
                                    .clone()
                                    .unwrap_or_else(|| DiceExpr::new(1, 8, 0)),
                                c.spell_damage_type
                                    .clone()
                                    .unwrap_or_else(|| "fire".to_string()),
                                c.spell_on_hit_condition.clone(),
                            )
                        };
                        Self::resolve_attack_with_stats(
                            ctx,
                            &current_id,
                            &target_id,
                            bonus,
                            dice,
                            &damage_type,
                            cond,
                            "casts a spell at",
                        );
                    }
                }
            }

            Self::advance_turn(ctx);
        }
    }
    pub fn try_spellcaster_support_action(ctx: &mut CombatContext, attacker_id: &str) -> bool {
        let (mut needing_heal, attacker_name) = {
            let n = ctx
                .state
                .combatants
                .values()
                .filter(|c| !c.is_player && c.is_alive() && c.current_hp < c.max_hp / 2)
                .map(|c| (c.id.clone(), c.current_hp, c.max_hp, c.name.clone()))
                .collect::<Vec<_>>();
            let a_name = ctx
                .state
                .combatants
                .get(attacker_id)
                .map(|c| c.name.clone())
                .unwrap_or_default();
            (n, a_name)
        };
        needing_heal.sort_by_key(|(_, hp, _, _)| *hp);

        if let Some((tid, cur_hp, max_hp, target_name)) = needing_heal.first() {
            let _seed = ctx.seed;
            let heal_orig = DiceExpr::new(1, 8, 2).roll(); // non-seeded for now to avoid complexity
            ctx.seed = (ctx.seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;

            let actual_heal = heal_orig.min(max_hp - cur_hp);
            if let Some(target_mut) = ctx.state.combatants.get_mut(tid) {
                target_mut.current_hp += actual_heal;
            }

            Self::push_log(
                ctx,
                format!(
                    "{} heals {} for {} HP.",
                    attacker_name, target_name, actual_heal
                ),
            );
            return true;
        }
        false
    }
    pub fn select_enemy_target(
        ctx: &mut CombatContext,
        attacker_id: &str,
        prefer_low_hp: bool,
    ) -> Option<String> {
        let targets = ctx
            .state
            .combatants
            .values()
            .filter(|c| c.id != attacker_id && c.is_player && c.is_alive())
            .collect::<Vec<_>>();

        if targets.is_empty() {
            return None;
        }

        if prefer_low_hp {
            let mut sorted = targets;
            sorted.sort_by_key(|c| c.current_hp);
            return Some(sorted[0].id.clone());
        }

        let len = targets.len() as u32;
        let idx = ctx.seed % len;
        ctx.seed = (ctx.seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;
        Some(targets[idx as usize].id.clone())
    }
}