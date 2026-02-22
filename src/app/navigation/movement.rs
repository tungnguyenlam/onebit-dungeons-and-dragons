use crate::app::{App, AppState};
use crate::data::types::{NpcDef, TriggerDef, TriggerKind};
use crate::game::{
    story::journal::Category as JournalCategory,
    world::region::Region,
};
use crate::renderer::SoundEffect;
use anyhow::Result;
impl App {
    pub fn try_move_player(&mut self, dx: i32, dy: i32) -> Result<()> {
        let Some(room) = self.current_room() else {
            return Ok(());
        };
        let next_col = self.player_pos.0 as i32 + dx;
        let next_row = self.player_pos.1 as i32 + dy;
        if room.grid.is_passable(next_col, next_row) {
            self.player_pos = (next_col as u32, next_row as u32);
            self.pass_turn()?;
        }
        Ok(())
    }
    pub fn is_blocked(&self) -> bool {
        let room = self.current_room();
        if let Some(room) = room {
            let (col, row) = self.player_pos;
            return !room.grid.is_passable(col as i32, row as i32);
        }
        false
    }
}