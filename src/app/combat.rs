use super::App;
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
    pub fn push_log(ctx: &mut CombatContext, line: impl Into<String>) {
        ctx.log.push(line.into());
        if ctx.log.len() > 100 {
            ctx.log.remove(0);
        }
    }

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

    #[allow(clippy::too_many_arguments)]
    pub fn resolve_attack_with_stats(
        ctx: &mut CombatContext,
        attacker_id: &str,
        target_id: &str,
        attack_bonus: i32,
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

    pub fn select_enemy_target(
        ctx: &CombatContext,
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

        Some(targets[0].id.clone())
    }

    pub fn advance_turn(ctx: &mut CombatContext) -> String {
        let (_, expired, next) = ctx.state.advance_turn_with_condition_tick();
        for cond in expired {
            if let Some(_c) = ctx.state.current_combatant() { // Not quite right but close
                 // Actually we want the combatant that just *left* or has the condition
            }
            Self::push_log(ctx, format!("Condition {:?} expired.", cond));
        }
        next.unwrap_or_else(|| "Unknown".into())
    }

    pub fn finish_combat_if_over(&mut self) {
        let Some((is_over, players_alive, player_hp, ws, gained_xp, loot_items)) =
            (match &self.state {
                AppState::Combat(ctx) => Some((
                    ctx.state.is_over(),
                    ctx.state
                        .combatants
                        .values()
                        .any(|c| c.is_player && c.is_alive()),
                    ctx.state.combatants.get("player").map(|c| c.current_hp),
                    ctx.world_state.clone(),
                    ctx.state
                        .combatants
                        .values()
                        .filter(|c| !c.is_player && c.current_hp <= 0)
                        .filter_map(|c| {
                            c.id.split('_')
                                .next()
                                .and_then(|mid| self.monster_defs.get(mid))
                                .map(|m| m.xp)
                        })
                        .sum::<u32>(),
                    ctx.state
                        .combatants
                        .values()
                        .filter(|c| !c.is_player && c.current_hp <= 0)
                        .filter_map(|c| {
                            c.id.split('_')
                                .next()
                                .and_then(|mid| self.monster_defs.get(mid))
                                .cloned()
                        })
                        .collect::<Vec<_>>(),
                )),
                _ => None,
            })
        else {
            return;
        };

        if !is_over {
            return;
        }
        if let Some(hp) = player_hp {
            self.player.current_hp = hp;
        }
        self.world_state = ws;

        if players_alive {
            self.grant_player_xp(gained_xp);

            let mut loot_granted = Vec::new();
            use rand::Rng;
            let mut rng = rand::rng();

            for monster in loot_items {
                if let Some(loot_table) = self.monster_defs.get(&monster.id).map(|m| &m.loot) {
                    for loot in loot_table {
                        if rng.random::<f32>() < loot.chance {
                            self.player.inventory.add(&loot.item_id, 1);
                            loot_granted.push(loot.item_id.clone());
                        }
                    }
                }
            }

            if !loot_granted.is_empty() {
                self.set_feedback(&format!("Found: {}", loot_granted.join(", ")));
            }

            self.world_state.set_flag("won_first_combat");
            self.modify_faction_rep("town_guard", 1);
            self.modify_faction_rep("goblin_tribe", -2);
            self.transition(AppState::WorldMap);
        } else {
            self.modify_faction_rep("town_guard", -1);
            self.transition(AppState::GameOver);
        }
    }

    pub fn use_potion_in_combat(ctx: &mut CombatContext, player: &mut Character) {
        if player.inventory.count("healing_potion") == 0 {
            Self::push_log(ctx, "You have no healing potions!");
            return;
        }

        let heal = DiceExpr::new(2, 4, 2).roll();
        ctx.seed = (ctx.seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;

        player.inventory.remove("healing_potion", 1);
        let actual_heal = heal.min(player.max_hp - player.current_hp);
        player.current_hp += actual_heal;

        if let Some(p) = ctx.state.combatants.get_mut("player") {
            p.current_hp = player.current_hp;
        }

        Self::push_log(
            ctx,
            format!("You drink a potion and heal for {} HP.", actual_heal),
        );
    }

    pub fn use_second_wind(ctx: &mut CombatContext) {
        let heal = DiceExpr::new(1, 10, 1).roll();
        ctx.seed = (ctx.seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;

        if let Some(p) = ctx.state.combatants.get_mut("player") {
            let actual = heal.min(p.max_hp - p.current_hp);
            p.current_hp += actual;
            Self::push_log(ctx, format!("Second Wind! Healed for {} HP.", actual));
        }
    }

    pub fn make_combat_context(&mut self) -> CombatContext {
        let (bonus, dmg_dice, _, _, _, _) = self.equipment_bonus_totals();
        let resistances = self.equipment_resistances();
        let immunities = self.equipment_immunities();
        let condition_immunities = self.equipment_condition_immunities();

        let armor_id = self.player.equipment.armor.as_deref();
        let armor_def = armor_id.and_then(|id| self.item_defs.get(id));
        let armor_tuple =
            armor_def.and_then(|d| d.armor.as_ref().map(|a| (a.base_ac, &a.armor_type)));

        let shield_equipped = self
            .player
            .equipment
            .off_hand
            .as_deref()
            .and_then(|id| self.item_defs.get(id))
            .map(|d| {
                d.armor
                    .as_ref()
                    .map(|a| a.armor_type == crate::data::types::ArmorType::Shield)
                    .unwrap_or(false)
            })
            .unwrap_or(false);

        let ac = crate::game::items::armor::armor_class(
            armor_tuple,
            shield_equipped,
            self.player.scores.dex_mod(),
        );

        let prof = self.player.proficiency_bonus();
        let str_mod = self.player.scores.str_mod() as i32;

        let mut p_dmg_dice = dmg_dice;
        p_dmg_dice.modifier += str_mod;

        let mut p_combatant = CombatantState::new(
            "player",
            self.player.name.clone(),
            true,
            self.player.max_hp,
            ac,
            30,
            self.player.scores.dex_mod() as i32,
            prof + str_mod + bonus,
            p_dmg_dice,
        );
        p_combatant.current_hp = self.player.current_hp;
        p_combatant.resistances = resistances;
        p_combatant.immunities = immunities;
        p_combatant.condition_immunities = condition_immunities;
        if crate::game::character::progression::has_extra_attack(
            &self.player.class_id,
            self.player.level,
        ) {
            p_combatant.action_slots.base_extra_attacks = 1;
        }

        // Find damage type of main hand weapon
        if let Some(weapon_id) = self.player.equipment.main_hand.as_deref() {
            if let Some(item) = self.item_defs.get(weapon_id) {
                if let Some(w) = &item.weapon {
                    p_combatant.damage_type = w.damage_type.clone();
                }
            }
        }

        let mut enemies = self.build_encounter_monsters(self.pending_encounter_monster.as_deref());
        self.pending_encounter_monster = None;

        let mut combatants = vec![p_combatant];

        // Add "guard_ally" if flag is set
        if self.world_state.flag("town_guard_trusted") {
            let mut guard = CombatantState::new(
                "guard_ally",
                "Town Guard",
                true,
                20,
                16,
                30,
                0,
                4,
                DiceExpr::new(1, 8, 2),
            );
            guard.damage_type = "piercing".into(); // assuming spear/crossbow
            combatants.push(guard);
        }

        combatants.append(&mut enemies);

        let seed = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            & 0x7fffffff) as u32;

        CombatContext {
            state: crate::game::combat::CombatState::new_with_seed(combatants, seed as u64),
            world_state: self.world_state.clone(),
            log: vec!["A new battle begins!".into()],
            seed,
            selected_enemy_id: None,
        }
    }

    pub fn build_encounter_monsters(&self, queued_monster: Option<&str>) -> Vec<CombatantState> {
        let mut out = Vec::new();
        let ids = if let Some(mid) = queued_monster {
            vec![mid]
        } else {
            vec!["goblin", "goblin"]
        };

        let mut seq: HashMap<&str, usize> = HashMap::new();
        for mid in ids {
            let Some(def) = self.monster_defs.get(mid) else {
                continue;
            };
            let n = seq.entry(mid).and_modify(|v| *v += 1).or_insert(1);
            let cid = format!("{}_{}", mid, *n);
            out.push(combatant_from_monster(
                &cid,
                def,
                self.settings.enemy_hp_multiplier,
            ));
        }
        out
    }
}
