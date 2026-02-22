use crate::app::App;
use crate::app::state::{AppState, FocusedPane, JournalUiState};
use crate::game::{
    items::equipment::EquipmentSlot,
    story::{
        dialog::{choose as dialog_choose, resolve as dialog_resolve},
        journal::Category as JournalCategory,
    },
};
use crate::renderer::{ControlFlow, GameEvent};
use anyhow::Result;


impl App {
    pub fn handle_combat(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::MoveUp | GameEvent::MoveDown => {
                if let AppState::Combat(ctx) = &mut self.state {
                    let targets = ctx
                        .state
                        .combatants
                        .values()
                        .filter(|c| !c.is_player && c.is_alive())
                        .map(|c| c.id.clone())
                        .collect::<Vec<_>>();
                    if !targets.is_empty() {
                        let current_idx = ctx
                            .selected_enemy_id
                            .as_ref()
                            .and_then(|id| targets.iter().position(|tid| tid == id))
                            .unwrap_or(0);

                        let next_idx = if event == GameEvent::MoveUp {
                            if current_idx == 0 {
                                targets.len() - 1
                            } else {
                                current_idx - 1
                            }
                        } else {
                            (current_idx + 1) % targets.len()
                        };
                        ctx.selected_enemy_id = Some(targets[next_idx].clone());
                    }
                }
            }
            GameEvent::Attack | GameEvent::Choice(1) => {
                if let AppState::Combat(ctx) = &mut self.state {
                    let Some(attacker_id) = ctx.state.current_combatant_id().map(str::to_string)
                    else {
                        Self::push_log(ctx, "No active combatant.");
                        return Ok(());
                    };

                    if !ctx
                        .state
                        .combatants
                        .get(&attacker_id)
                        .is_some_and(|c| c.is_player)
                    {
                        Self::push_log(ctx, "It's not the player's turn.");
                        return Ok(());
                    }

                    let target_id = ctx
                        .selected_enemy_id
                        .clone()
                        .or_else(|| ctx.state.next_enemy_id(&attacker_id).map(str::to_string));

                    let Some(target_id) = target_id else {
                        Self::push_log(ctx, "No valid target.");
                        return Ok(());
                    };

                    let _ = Self::resolve_attack(
                        ctx,
                        &attacker_id,
                        &target_id,
                        self.settings.player_damage_multiplier,
                    );
                }
                self.finish_combat_if_over();
            }
            GameEvent::Choice(4) => {
                let mut fled = false;
                if let AppState::Combat(ctx) = &mut self.state {
                    let is_player_turn = ctx
                        .state
                        .current_combatant_id()
                        .and_then(|id| ctx.state.combatants.get(id))
                        .map(|c| c.is_player)
                        .unwrap_or(false);

                    if !is_player_turn {
                        Self::push_log(ctx, "It's not your turn.");
                        return Ok(());
                    }

                    fled = Self::try_flee(ctx);
                    if !fled {
                        let before = ctx
                            .state
                            .current_combatant()
                            .map(|c| c.name.clone())
                            .unwrap_or_else(|| "Unknown".into());
                        let after = Self::advance_turn(ctx);
                        Self::push_log(ctx, format!("{before} failed to flee. {after} is up."));
                    }
                }
                if fled {
                    self.transition(AppState::WorldMap);
                } else {
                    self.pass_turn()?;
                }
            }
            GameEvent::Choice(2) | GameEvent::Choice(3) | GameEvent::Wait => {
                if let AppState::Combat(ctx) = &mut self.state {
                    let is_player_turn = ctx
                        .state
                        .current_combatant_id()
                        .and_then(|id| ctx.state.combatants.get(id))
                        .map(|c| c.is_player)
                        .unwrap_or(false);

                    if !is_player_turn {
                        Self::push_log(ctx, "It's not your turn.");
                        return Ok(());
                    }

                    let success = match event {
                        GameEvent::Choice(2) => Self::use_potion_in_combat(ctx, &mut self.player),
                        GameEvent::Choice(3) => Self::use_second_wind(ctx),
                        GameEvent::Wait => true,
                        _ => false,
                    };

                    if !success {
                        return Ok(());
                    }

                    let before = ctx
                        .state
                        .current_combatant()
                        .map(|c| c.name.clone())
                        .unwrap_or_else(|| "Unknown".into());
                    let after = Self::advance_turn(ctx);
                    if event == GameEvent::Wait {
                        Self::push_log(ctx, format!("{before} waits. {after} is up."));
                    } else {
                        Self::push_log(ctx, format!("{before} ends turn. {after} is up."));
                    }
                }
                self.pass_turn()?;
            }
            GameEvent::Cancel | GameEvent::Back => self.transition(AppState::WorldMap),
            _ => {}
        }
        Ok(())
    }
}