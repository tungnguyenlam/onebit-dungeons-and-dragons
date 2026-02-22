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