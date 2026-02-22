use crate::app::{App, AppState};
use crate::data::types::{NpcDef, TriggerDef, TriggerKind};
use crate::game::{
    story::journal::Category as JournalCategory,
    world::region::Region,
};
use crate::renderer::SoundEffect;
use anyhow::Result;
impl App {
    pub fn is_near_door(&self) -> bool {
        let (cx, cy) = self.player_pos;
        let room = self.current_room();
        if let Some(room) = room {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = cx as i32 + dx;
                    let ny = cy as i32 + dy;
                    if nx >= 0 && ny >= 0 {
                        if let Some(tile) = room.grid.get(nx as u32, ny as u32) {
                            if matches!(
                                tile,
                                crate::game::world::map::Tile::DoorOpen
                                    | crate::game::world::map::Tile::DoorClosed
                            ) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    pub fn is_near_chest(&self) -> bool {
        let (cx, cy) = self.player_pos;
        let room = self.current_room();
        if let Some(room) = room {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = cx as i32 + dx;
                    let ny = cy as i32 + dy;
                    if nx >= 0 && ny >= 0 {
                        if let Some(tile) = room.grid.get(nx as u32, ny as u32) {
                            if matches!(tile, crate::game::world::map::Tile::Chest) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
}