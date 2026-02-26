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
    pub fn push_log(ctx: &mut CombatContext, line: impl Into<String>) {
        ctx.log.push(line.into());
        if ctx.log.len() > 100 {
            ctx.log.remove(0);
        }
    }
    pub fn advance_turn(ctx: &mut CombatContext) -> String {
        let (leaving_id, expired, next_id_opt) = ctx.state.advance_turn_with_condition_tick();
        for cond in expired {
            if let Some(id) = &leaving_id {
                if let Some(c) = ctx.state.combatants.get(id) {
                    Self::push_log(ctx, format!("{} is no longer {:?}.", c.name, cond));
                }
            }
        }
        next_id_opt
            .and_then(|id| ctx.state.combatants.get(&id))
            .map(|c| c.name.clone())
            .unwrap_or_else(|| "Unknown".into())
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
            let mut harvest_granted = Vec::new();
            use rand::Rng;
            let mut rng = rand::rng();

            for monster in loot_items {
                self.world_state.discover_monster(&monster.id);
                self.world_state.register_monster_kill(&monster.id);
                if monster.id == "void_architect" {
                    if self.world_state.flag("drow_allied") {
                        self.world_state.set_flag("silence_heard");
                    } else {
                        self.world_state.set_flag("silence_silenced");
                    }
                    self.world_state.set_flag("game_completed");
                    self.world_state.set_flag("ng_plus_unlocked");
                    self.ng_plus_unlocked = true;
                }

                if let Some(loot_table) = self.monster_defs.get(&monster.id).map(|m| &m.loot) {
                    for loot in loot_table {
                        if rng.random::<f32>() < loot.chance {
                            self.player.inventory.add(&loot.item_id, 1);
                            loot_granted.push(loot.item_id.clone());
                        }
                    }
                }

                if let Some(msg) = self.harvest_from_monster(&monster.id) {
                    harvest_granted.push(msg);
                }
            }

            if !loot_granted.is_empty() {
                self.set_feedback(&format!("Found: {}", loot_granted.join(", ")));
            } else if !harvest_granted.is_empty() {
                self.set_feedback(&harvest_granted.join(" "));
            }

            self.world_state.set_flag("won_first_combat");
            self.modify_faction_rep("town_guard", 1);
            self.modify_faction_rep("goblin_tribe", -2);
            if self.world_state.flag("game_completed") {
                self.ending_scroll = 0;
                self.transition(AppState::Ending);
            } else {
                self.transition(AppState::WorldMap);
            }
        } else {
            self.modify_faction_rep("town_guard", -1);
            self.transition(AppState::GameOver);
        }
    }
    pub fn use_potion_in_combat(ctx: &mut CombatContext, player: &mut Character) -> bool {
        if player.inventory.count("healing_potion") == 0 {
            Self::push_log(ctx, "You have no healing potions!");
            return false;
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
        true
    }
    pub fn try_flee(ctx: &mut CombatContext) -> bool {
        let max_enemy_dex = ctx
            .state
            .combatants
            .values()
            .filter(|c| !c.is_player && c.is_alive())
            .map(|c| c.initiative_mod)
            .max()
            .unwrap_or(0);

        let player_dex = ctx
            .state
            .combatants
            .get("player")
            .map(|p| p.initiative_mod)
            .unwrap_or(0);

        let p_roll = DiceExpr::new(1, 20, player_dex).roll();
        ctx.seed = (ctx.seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;

        let e_roll = DiceExpr::new(1, 20, max_enemy_dex).roll();
        ctx.seed = (ctx.seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;

        if p_roll > e_roll {
            Self::push_log(ctx, "You successfully fled from combat!");
            true
        } else {
            Self::push_log(
                ctx,
                format!("You failed to flee! ({} vs {})", p_roll, e_roll),
            );
            false
        }
    }
    pub fn use_second_wind(ctx: &mut CombatContext) -> bool {
        let heal = DiceExpr::new(1, 10, 1).roll();
        ctx.seed = (ctx.seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;

        if let Some(p) = ctx.state.combatants.get_mut("player") {
            let actual = heal.min(p.max_hp - p.current_hp);
            p.current_hp += actual;
            Self::push_log(ctx, format!("Second Wind! Healed for {} HP.", actual));
        }
        true
    }
}
