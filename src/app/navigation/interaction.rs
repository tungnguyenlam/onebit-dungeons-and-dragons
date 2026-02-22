use crate::app::{App, AppState};
use crate::data::types::{NpcDef, TriggerDef, TriggerKind};
use crate::game::{
    story::journal::Category as JournalCategory,
    world::region::Region,
};
use crate::renderer::SoundEffect;
use anyhow::Result;
impl App {
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
                self.run_enemy_turns();
                self.finish_combat_if_over();
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
}