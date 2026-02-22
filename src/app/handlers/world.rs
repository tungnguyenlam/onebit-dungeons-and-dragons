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
            _ => {}
        }
        Ok(())
    }
}