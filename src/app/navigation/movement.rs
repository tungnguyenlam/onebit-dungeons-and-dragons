use crate::app::{App, AppState};
use crate::data::types::{NpcDef, TriggerDef, TriggerKind};
use crate::game::{story::journal::Category as JournalCategory, world::region::Region};
use crate::game::world::map::Tile;
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
            self.apply_hazard_tile_effects();
            self.pass_turn()?;
        }
        Ok(())
    }

    fn apply_hazard_tile_effects(&mut self) {
        let Some(room) = self.current_room() else {
            return;
        };
        let Some(tile) = room.grid.get(self.player_pos.0, self.player_pos.1) else {
            return;
        };
        match tile {
            Tile::DeepWater => {
                self.player.take_damage(1);
                self.set_feedback("Deep water drags you down. You take 1 damage.");
            }
            Tile::Pit => {
                self.player.take_damage(2);
                self.player
                    .conditions
                    .insert(crate::game::character::conditions::Condition::Prone);
                self.set_feedback("You stumble near the pit and take 2 damage.");
            }
            Tile::Rift => {
                if self.player.inventory.count("rope_of_climbing") == 0 {
                    self.player.take_damage(3);
                    self.player
                        .conditions
                        .insert(crate::game::character::conditions::Condition::Restrained);
                    self.set_feedback("The rift tears at you. Use a rope to cross safely.");
                } else {
                    self.set_feedback("You anchor your rope and cross the rift safely.");
                }
            }
            _ => {}
        }
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
