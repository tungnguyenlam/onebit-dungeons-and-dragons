use crate::app::state::{AppState, FocusedPane, JournalUiState};
use crate::app::App;
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
    pub fn handle_world_map(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Attack => {
                let ctx = self.make_combat_context();
                self.transition(AppState::Combat(ctx));
                self.run_enemy_turns();
                self.finish_combat_if_over();
            }
            GameEvent::OpenInventory => self.transition(AppState::Inventory),
            GameEvent::OpenSpellbook => self.transition(AppState::Spellbook),
            GameEvent::OpenCrafting => self.transition(AppState::Crafting),
            GameEvent::OpenBestiary => self.transition(AppState::Bestiary),
            GameEvent::OpenLoreLibrary => self.transition(AppState::LoreLibrary),
            GameEvent::OpenJournal => {
                self.journal.mark_read();
                self.journal_ui.selected = 0;
                self.journal_ui.detail_scroll = 0;
                self.focused_pane = FocusedPane::Main;
                self.transition(AppState::Journal);
            }
            GameEvent::OpenHelp => {
                self.show_help = !self.show_help;
            }
            GameEvent::Wait => self.pass_turn()?,
            GameEvent::MoveUp => self.try_move_player(0, -1)?,
            GameEvent::MoveDown => self.try_move_player(0, 1)?,
            GameEvent::MoveLeft => self.try_move_player(-1, 0)?,
            GameEvent::MoveRight => self.try_move_player(1, 0)?,
            GameEvent::Confirm | GameEvent::OpenMap => self.interact_current_tile(),
            GameEvent::Choice(n @ 1..=6) => {
                let ability = match n {
                    1 => "strength",
                    2 => "dexterity",
                    3 => "constitution",
                    4 => "intelligence",
                    5 => "wisdom",
                    6 => "charisma",
                    _ => "",
                };
                if !ability.is_empty() {
                    if self.allocate_stat_point(ability) {
                        self.set_feedback(&format!(
                            "Increased {}. Free points left: {}",
                            ability, self.player.skill_points
                        ));
                    } else if self.player.skill_points == 0 {
                        self.set_feedback("No free stat points to allocate.");
                    } else {
                        self.set_feedback("That stat is already at cap.");
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
