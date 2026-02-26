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

        let out_of_bounds = next_col < 0
            || next_row < 0
            || next_col >= room.width() as i32
            || next_row >= room.height() as i32;
        if out_of_bounds {
            if self.try_edge_room_transition(dx, dy)? {
                self.set_feedback("You move into the next area.");
            }
            return Ok(());
        }

        if room.grid.is_passable(next_col, next_row) {
            self.player_pos = (next_col as u32, next_row as u32);
            if self.try_step_travel_trigger()? {
                return Ok(());
            }
            self.apply_hazard_tile_effects();
            self.pass_turn()?;
        } else {
            let is_boundary_wall = next_col == 0
                || next_row == 0
                || next_col == room.width() as i32 - 1
                || next_row == room.height() as i32 - 1;
            if is_boundary_wall && self.try_edge_room_transition(dx, dy)? {
                self.set_feedback("You move into the next area.");
            }
        }
        Ok(())
    }

    fn try_step_travel_trigger(&mut self) -> Result<bool> {
        let room_id = self.current_room_id.clone();
        let (col, row) = self.player_pos;
        if let Some(trigger) = self
            .region
            .room(&room_id)
            .and_then(|r| r.trigger_at(col, row).cloned())
        {
            if matches!(trigger.kind, TriggerKind::Travel) {
                // Intra-region room links are now edge-driven via explicit exits.
                if self.region.room(&trigger.target_id).is_some() {
                    return Ok(false);
                }
                self.execute_trigger(&trigger);
                // Travel should always spend a world turn.
                if matches!(self.state, AppState::WorldMap) {
                    self.pass_turn()?;
                }
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn try_edge_room_transition(&mut self, dx: i32, dy: i32) -> Result<bool> {
        let room_id = self.current_room_id.clone();
        let Some(room) = self.region.room(&room_id) else {
            return Ok(false);
        };

        let target = match edge_direction(dx, dy) {
            (-1, 0) => room.exits.west.clone(),
            (1, 0) => room.exits.east.clone(),
            (0, -1) => room.exits.north.clone(),
            (0, 1) => room.exits.south.clone(),
            _ => String::new(),
        };

        if !target.is_empty() {
            self.handle_travel(&target);
            if matches!(self.state, AppState::WorldMap) {
                self.pass_turn()?;
            }
            return Ok(true);
        }
        Ok(false)
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

fn edge_direction(dx: i32, dy: i32) -> (i32, i32) {
    match (dx.signum(), dy.signum()) {
        (-1, 0) => (-1, 0),
        (1, 0) => (1, 0),
        (0, -1) => (0, -1),
        (0, 1) => (0, 1),
        _ => (0, 0),
    }
}
