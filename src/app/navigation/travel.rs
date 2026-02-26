use crate::app::{App, AppState};
use crate::data::types::{NpcDef, TriggerDef, TriggerKind};
use crate::game::{story::journal::Category as JournalCategory, world::region::Region};
use crate::renderer::SoundEffect;
use anyhow::Result;
impl App {
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
                self.run_enemy_turns();
                self.finish_combat_if_over();

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
        if let Some(back_trigger) = room
            .triggers
            .iter()
            .find(|t| matches!(t.kind, TriggerKind::Travel) && t.target_id == from_room_id)
        {
            // Spawn next to the back trigger instead of on top of it, if possible
            let tx = back_trigger.position[0];
            let ty = back_trigger.position[1];

            // Just spawn on it for now to ensure it works, but usually you'd offset it
            return (tx, ty);
        }

        // Fallback to default spawn
        crate::app::samples::find_spawn_pos_for_room(room)
    }
}
