use super::App;
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

// Helper for rotating category
fn next_category(c: JournalCategory) -> JournalCategory {
    match c {
        JournalCategory::Quest => JournalCategory::Lore,
        JournalCategory::Lore => JournalCategory::World,
        JournalCategory::World => JournalCategory::Combat,
        JournalCategory::Combat => JournalCategory::Dialog,
        JournalCategory::Dialog => JournalCategory::System,
        JournalCategory::System => JournalCategory::Quest,
    }
}

fn prev_category(c: JournalCategory) -> JournalCategory {
    match c {
        JournalCategory::Quest => JournalCategory::System,
        JournalCategory::Lore => JournalCategory::Quest,
        JournalCategory::World => JournalCategory::Lore,
        JournalCategory::Combat => JournalCategory::World,
        JournalCategory::Dialog => JournalCategory::Combat,
        JournalCategory::System => JournalCategory::Dialog,
    }
}

impl App {
    pub fn handle_main_menu(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::MoveUp => {
                self.menu_ui.selected = self.menu_ui.selected.saturating_sub(1);
            }
            GameEvent::MoveDown => {
                self.menu_ui.selected = (self.menu_ui.selected + 1).min(3);
            }
            GameEvent::Confirm => match self.menu_ui.selected {
                0 => self.transition(AppState::CharacterCreation),
                1 => self.transition(AppState::WorldMap),
                2 => {
                    if let Err(_e) = self.load_from_default_path() {
                        // In a real app we'd log this or show a message
                    }
                    self.transition(AppState::WorldMap);
                }
                3 => {}
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    pub fn handle_settings(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Back | GameEvent::Cancel | GameEvent::OpenSettings => {
                if self.player.current_hp > 0 {
                    self.transition(AppState::WorldMap);
                } else {
                    self.transition(AppState::MainMenu);
                }
            }
            GameEvent::MoveUp => {
                self.settings_ui.selected = self.settings_ui.selected.saturating_sub(1);
            }
            GameEvent::MoveDown => {
                let max_opts = 2; // 0, 1, 2
                if self.settings_ui.selected < max_opts {
                    self.settings_ui.selected += 1;
                }
            }
            GameEvent::MoveLeft => match self.settings_ui.selected {
                0 => {
                    self.settings.enemy_hp_multiplier =
                        (self.settings.enemy_hp_multiplier - 0.1).max(0.5)
                }
                1 => {
                    self.settings.player_damage_multiplier =
                        (self.settings.player_damage_multiplier - 0.1).max(0.5)
                }
                2 => self.settings.reduced_motion = !self.settings.reduced_motion,
                _ => {}
            },
            GameEvent::MoveRight | GameEvent::Confirm => match self.settings_ui.selected {
                0 => {
                    self.settings.enemy_hp_multiplier =
                        (self.settings.enemy_hp_multiplier + 0.1).min(2.0)
                }
                1 => {
                    self.settings.player_damage_multiplier =
                        (self.settings.player_damage_multiplier + 0.1).min(2.0)
                }
                2 => self.settings.reduced_motion = !self.settings.reduced_motion,
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    pub fn handle_world_map(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Attack => {
                let ctx = self.make_combat_context();
                self.transition(AppState::Combat(ctx));
            }
            GameEvent::OpenInventory => self.transition(AppState::Inventory),
            GameEvent::OpenSpellbook => self.transition(AppState::Spellbook),
            GameEvent::OpenJournal => {
                self.journal.mark_read();
                self.journal_ui.selected = 0;
                self.journal_ui.detail_scroll = 0;
                self.focused_pane = FocusedPane::Main;
                self.transition(AppState::Journal);
            }
            GameEvent::MoveUp => self.try_move_player(0, -1),
            GameEvent::MoveDown => self.try_move_player(0, 1),
            GameEvent::MoveLeft => self.try_move_player(-1, 0),
            GameEvent::MoveRight => self.try_move_player(1, 0),
            GameEvent::Confirm | GameEvent::OpenMap => self.interact_current_tile(),
            _ => {}
        }
        Ok(())
    }

    pub fn handle_combat(&mut self, event: GameEvent) -> Result<()> {
        match event {
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

                    let Some(target_id) = ctx.state.next_enemy_id(&attacker_id).map(str::to_string)
                    else {
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
            GameEvent::Choice(2) => {
                if let AppState::Combat(ctx) = &mut self.state {
                    Self::use_potion_in_combat(ctx, &mut self.player);
                }
            }
            GameEvent::Choice(3) => {
                if let AppState::Combat(ctx) = &mut self.state {
                    Self::use_second_wind(ctx);
                }
            }
            GameEvent::Wait => {
                if let AppState::Combat(ctx) = &mut self.state {
                    let before = ctx
                        .state
                        .current_combatant()
                        .map(|c| c.name.clone())
                        .unwrap_or_else(|| "Unknown".into());
                    let after = Self::advance_turn(ctx);
                    Self::push_log(ctx, format!("{before} ends turn. {after} is up."));
                }
                self.run_enemy_turns();
                self.finish_combat_if_over();
            }
            GameEvent::Cancel | GameEvent::Back => self.transition(AppState::WorldMap),
            _ => {}
        }
        Ok(())
    }

    pub fn handle_dialog(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Cancel | GameEvent::Back => self.transition(AppState::WorldMap),
            GameEvent::Choice(n) => {
                let idx = n.saturating_sub(1) as usize;
                if let AppState::Dialog(ctx) = &mut self.state {
                    let Some(next) =
                        dialog_choose(&ctx.tree, &ctx.current_node, idx, &mut self.world_state)
                    else {
                        self.journal.append(
                            format!("dialog-blocked-{}-{}", ctx.tree.npc_id, self.turn),
                            self.turn,
                            JournalCategory::Dialog,
                            None,
                            format!("Talked with {}", ctx.npc_name),
                            "That option is unavailable right now.",
                        );
                        return Ok(());
                    };
                    if next == "END" {
                        self.transition(AppState::WorldMap);
                        return Ok(());
                    }
                    if let Some(resolved) = dialog_resolve(&ctx.tree, &next, &mut self.world_state)
                    {
                        ctx.current_node = next;
                        ctx.resolved = resolved;
                        self.journal.append(
                            format!("dialog-{}-{}", ctx.tree.npc_id, ctx.current_node),
                            self.turn,
                            JournalCategory::Dialog,
                            None,
                            format!("Talked with {}", ctx.npc_name),
                            ctx.resolved.text.clone(),
                        );
                    } else {
                        self.journal.append(
                            format!("dialog-broken-{}-{}", ctx.tree.npc_id, self.turn),
                            self.turn,
                            JournalCategory::Dialog,
                            None,
                            format!("Talked with {}", ctx.npc_name),
                            "Conversation path is blocked. Try another response or return later.",
                        );
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn handle_inventory(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Back | GameEvent::Cancel => self.transition(AppState::WorldMap),
            GameEvent::Choice(1) => self.toggle_equip(EquipmentSlot::MainHand, "longsword"),
            GameEvent::Choice(2) => self.toggle_equip(EquipmentSlot::Armor, "leather_armor"),
            GameEvent::Choice(3) => self.toggle_equip(EquipmentSlot::OffHand, "shield"),
            GameEvent::Choice(4) => self.use_healing_potion(),
            _ => {}
        }
        Ok(())
    }

    pub fn handle_journal(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Back | GameEvent::Cancel => {
                if self.focused_pane == FocusedPane::Side {
                    self.focused_pane = FocusedPane::Main;
                } else {
                    self.transition(AppState::WorldMap);
                }
            }
            GameEvent::Confirm => {
                if self.focused_pane == FocusedPane::Main {
                    self.focused_pane = FocusedPane::Side;
                }
            }
            GameEvent::MoveUp => {
                if self.focused_pane == FocusedPane::Main {
                    self.journal_ui.selected = self.journal_ui.selected.saturating_sub(1);
                    self.journal_ui.detail_scroll = 0;
                } else {
                    self.journal_ui.detail_scroll = self.journal_ui.detail_scroll.saturating_sub(1);
                }
            }
            GameEvent::MoveDown => {
                if self.focused_pane == FocusedPane::Main {
                    self.journal_ui.selected = self.journal_ui.selected.saturating_add(1);
                    self.journal_ui.detail_scroll = 0;
                } else {
                    self.journal_ui.detail_scroll = self.journal_ui.detail_scroll.saturating_add(1);
                }
            }
            GameEvent::MoveLeft => {
                if self.focused_pane == FocusedPane::Side {
                    self.focused_pane = FocusedPane::Main;
                } else {
                    self.journal_ui.category = prev_category(self.journal_ui.category);
                    self.journal_ui.selected = 0;
                    self.journal_ui.detail_scroll = 0;
                }
            }
            GameEvent::MoveRight => {
                if self.focused_pane == FocusedPane::Main {
                    self.journal_ui.category = next_category(self.journal_ui.category);
                    self.journal_ui.selected = 0;
                    self.journal_ui.detail_scroll = 0;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn handle_spellbook(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Back | GameEvent::Cancel => self.transition(AppState::WorldMap),
            GameEvent::Choice(n @ 1..=9) => self.cast_known_spell((n - 1) as usize),
            _ => {}
        }
        Ok(())
    }

    pub fn handle_char_creation(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::MoveUp => {
                self.char_creation_ui.selected = self.char_creation_ui.selected.saturating_sub(1);
            }
            GameEvent::MoveDown => {
                self.char_creation_ui.selected = (self.char_creation_ui.selected + 1).min(3);
            }
            GameEvent::MoveLeft => {
                if self.char_creation_ui.selected == 1 {
                    self.char_creation_ui.class_index =
                        self.char_creation_ui.class_index.saturating_sub(1);
                } else if self.char_creation_ui.selected == 2 {
                    self.char_creation_ui.race_index =
                        self.char_creation_ui.race_index.saturating_sub(1);
                }
            }
            GameEvent::MoveRight => {
                if self.char_creation_ui.selected == 1 {
                    self.char_creation_ui.class_index = (self.char_creation_ui.class_index + 1)
                        .min(self.char_creation_ui.class_options.len().saturating_sub(1));
                } else if self.char_creation_ui.selected == 2 {
                    self.char_creation_ui.race_index = (self.char_creation_ui.race_index + 1)
                        .min(self.char_creation_ui.race_options.len().saturating_sub(1));
                }
            }
            GameEvent::Choice(n @ 1..=9) => {
                if self.char_creation_ui.selected == 0 {
                    self.char_creation_ui.name.push(char::from(b'0' + n));
                }
            }
            GameEvent::Back | GameEvent::Cancel => self.transition(AppState::MainMenu),
            GameEvent::Confirm => {
                if self.char_creation_ui.selected == 3 {
                    self.apply_character_creation();
                    self.transition(AppState::WorldMap);
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn handle_game_over(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Confirm => self.transition(AppState::MainMenu),
            GameEvent::LoadGame => self.load_from_default_path()?,
            _ => {}
        }
        Ok(())
    }
}
