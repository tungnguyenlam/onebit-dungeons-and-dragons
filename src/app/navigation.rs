use crate::app::{App, AppState};
use crate::data::types::{NpcDef, TriggerDef, TriggerKind};
use crate::game::{
    story::journal::Category as JournalCategory,
    world::region::Region,
};
use crate::renderer::SoundEffect;

impl App {
    pub fn try_move_player(&mut self, dx: i32, dy: i32) {
        let Some(room) = self.current_room() else {
            return;
        };
        let next_col = self.player_pos.0 as i32 + dx;
        let next_row = self.player_pos.1 as i32 + dy;
        if room.grid.is_passable(next_col, next_row) {
            self.player_pos = (next_col as u32, next_row as u32);
        }
    }

    pub fn handle_travel(&mut self, target_id: &str) {
        let epic_progress = self.world_state.counter("epic_quest_progress");
        let macguffin_acquired = self.world_state.flag("macguffin_acquired");
        let threat_level = if self.world_state.flag("macguffin_acquired") {
            3
        } else if epic_progress >= 2 {
            2
        } else if epic_progress >= 1 {
            1
        } else {
            0
        };

        if threat_level > 0 {
            use rand::Rng;
            let mut rng = rand::rng();
            let ambush_chance = match threat_level {
                3 => 4,
                2 => 5,
                1 => 6,
                _ => 0,
            };

            if rng.random_range(1..=ambush_chance) == 1 {
                self.queue_sound(SoundEffect::Beep);
                let ambush_monster = match threat_level {
                    3 => "ghostly_knight",
                    2 => "orc_warchief",
                    _ => "forest_goblin",
                };
                self.pending_encounter_monster = Some(ambush_monster.into());
                let ctx = self.make_combat_context();
                self.transition(AppState::Combat(ctx));

                if threat_level >= 2 && !self.world_state.flag("antagonist_noticed") {
                    self.world_state.set_flag("antagonist_noticed");
                    self.journal.append(
                        format!("antagonist-notice-{}", self.turn),
                        self.turn,
                        JournalCategory::World,
                        None,
                        "The Antagonist Notices You",
                        "Dark scouts have reported your movements. Expect increased hostility.",
                    );
                }
                return;
            }
        }

        let from_room_id = self.current_room_id.clone();

        if self.region.room(target_id).is_some() {
            self.current_room_id = target_id.to_string();
            if let Some(new_room) = self.current_room() {
                self.player_pos = self.find_entry_pos(new_room, &from_room_id);
                self.check_room_hostilities();
            }
        } else if let Some(conn) = self
            .region
            .connections
            .iter()
            .find(|c| {
                c.from_room == self.current_room_id
                    && (c.to_region == target_id || c.to_room == target_id)
            })
            .cloned()
        {
            let mut target_region = conn.to_region.clone();
            let ruined_map = [("ironhold-mines", "ruined-ironhold-mines")];

            for (normal, ruined) in ruined_map {
                if macguffin_acquired && target_region == normal {
                    target_region = ruined.into();
                    break;
                }
            }

            if let Ok(loaded) = crate::data::loader::load_region("assets", &target_region) {
                self.region = Region::from_loaded(&loaded);
                self.region_npcs = loaded.npcs;
                self.region_dialogs = loaded.dialogs;
                self.current_room_id = conn.to_room.clone();
                if !self.region.rooms.contains_key(&self.current_room_id) {
                    self.current_room_id = loaded.manifest.entry_room;
                }
                if let Some(new_room) = self.current_room() {
                    self.player_pos = self.find_entry_pos(new_room, &from_room_id);
                    self.check_room_hostilities();
                }
            }
            self.queue_sound(SoundEffect::Beep);
        }
    }

    pub fn find_entry_pos(
        &self,
        room: &crate::game::world::room::Room,
        from_room_id: &str,
    ) -> (u32, u32) {
        // Try to find a travel trigger in the new room that leads back to the old room
        if let Some(back_trigger) = room.triggers.iter().find(|t| {
            matches!(t.kind, TriggerKind::Travel) && t.target_id == from_room_id
        }) {
            // Spawn next to the back trigger instead of on top of it, if possible
            let tx = back_trigger.position[0];
            let ty = back_trigger.position[1];

            // Just spawn on it for now to ensure it works, but usually you'd offset it
            return (tx, ty);
        }

        // Fallback to default spawn
        crate::app::samples::find_spawn_pos_for_room(room)
    }

    pub fn get_npc_at_player_position(&self) -> Option<&NpcDef> {
        let (col, row) = self.player_pos;
        if let Some(room) = self.current_room() {
            if let Some(room_npc) = room
                .npcs
                .iter()
                .find(|n| n.position[0] == col && n.position[1] == row)
            {
                return self.region_npcs.get(&room_npc.id);
            }
        }
        None
    }

    pub fn interact_current_tile(&mut self) {
        let room_id = self.current_room_id.clone();
        let (col, row) = self.player_pos;

        // Check triggers at current position - get fresh borrow
        if let Some(trigger) = self
            .region
            .room(&room_id)
            .and_then(|r| r.trigger_at(col, row).cloned())
        {
            self.execute_trigger(&trigger);
            return;
        }

        // Check for NPCs at current position
        if let Some(room) = self.region.room(&room_id) {
            if let Some(npc_id) = room
                .npcs
                .iter()
                .find(|n| n.position[0] == col && n.position[1] == row)
                .map(|n| n.id.clone())
            {
                self.start_dialog_with_npc(&npc_id);
                return;
            }
        }

        // Check adjacent for doors/chests/travel
        let mut interactable_found = false;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = col as i32 + dx;
                let ny = row as i32 + dy;
                if nx >= 0 && ny >= 0 {
                    let nx = nx as u32;
                    let ny = ny as u32;

                    if let Some(trigger) = self
                        .region
                        .room(&room_id)
                        .and_then(|r| r.trigger_at(nx, ny).cloned())
                    {
                        if matches!(trigger.kind, TriggerKind::Travel) {
                            self.execute_trigger(&trigger);
                            interactable_found = true;
                            break;
                        }
                    }
                }
            }
            if interactable_found {
                break;
            }
        }

        if !interactable_found {
            if self.is_near_door() {
                self.set_feedback("You are near a door. Step on it or face it to interact.");
            } else if self.is_near_chest() {
                self.set_feedback("You are near a chest. Step on it to open.");
            } else {
                self.set_feedback("Nothing here to interact with.");
            }
        }
    }

    pub fn execute_trigger(&mut self, trigger: &TriggerDef) {
        match trigger.kind {
            TriggerKind::Dialog => {
                self.start_dialog_with_npc(&trigger.target_id);
            }
            TriggerKind::Encounter => {
                self.pending_encounter_monster = Some(trigger.target_id.clone());
                let ctx = self.make_combat_context();
                self.transition(AppState::Combat(ctx));
            }
            TriggerKind::Lore => {
                if let Some(entry) = self.lore_defs.get(&trigger.target_id) {
                    crate::game::story::events::inspect_lore(
                        entry,
                        &mut self.world_state,
                        &mut self.journal,
                        self.turn,
                    );
                }
            }
            TriggerKind::QuestStage => {
                self.world_state.set_flag(trigger.target_id.clone());

                let macguffins = [
                    "has_obsidian_eye",
                    "has_obsidian_heart",
                    "has_sylvan_glitch_key",
                    "has_null_scepter",
                ];

                if macguffins.iter().any(|m| *m == trigger.target_id) {
                    if !self.world_state.flag("macguffin_acquired") {
                        self.world_state.set_flag("macguffin_acquired");
                        let macguffin_count = macguffins
                            .iter()
                            .filter(|m| self.world_state.flag(m))
                            .count();
                        self.world_state
                            .delta_counter("epic_quest_progress", macguffin_count as i32);

                        self.journal.append(
                            format!("macguffin-acquired-{}", self.turn),
                            self.turn,
                            JournalCategory::World,
                            None,
                            "The Antagonist Stirs",
                            "Dark forces have detected your acquisition. The enemy grows stronger against you.",
                        );
                    }
                }
            }
            TriggerKind::Travel => {
                self.handle_travel(&trigger.target_id);
            }
        }
    }

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

    pub fn is_blocked(&self) -> bool {
        let room = self.current_room();
        if let Some(room) = room {
            let (col, row) = self.player_pos;
            return !room.grid.is_passable(col as i32, row as i32);
        }
        false
    }
}
