use crate::app::App;
use crate::game::world::weather::WeatherType;
use crate::game::save::{load_from_path, save_to_path, SaveGame, SAVE_FORMAT_VERSION};
use crate::renderer::SoundEffect;
use anyhow::Result;

impl App {
    pub fn save_to_default_path(&mut self) -> Result<()> {
        let save = SaveGame {
            format_version: SAVE_FORMAT_VERSION,
            player: self.player.clone(),
            world_state: self.world_state.clone(),
            journal: self.journal.clone(),
            turn: self.turn,
            region_slug: self.region.slug.clone(),
            room_id: self.current_room_id.clone(),
            player_pos: self.player_pos,
            state: self.state.clone(),
            menu_ui: self.menu_ui.clone(),
            char_creation_ui: self.char_creation_ui.clone(),
            journal_ui: self.journal_ui.clone(),
            settings_ui: self.settings_ui.clone(),
        };
        save_to_path("save.toml", &save)
    }

    pub fn load_from_default_path(&mut self) -> Result<()> {
        let save = load_from_path("save.toml")?;
        self.player = save.player;
        self.world_state = save.world_state;
        self.journal = save.journal;
        self.turn = save.turn;
        self.current_room_id = save.room_id;
        self.player_pos = save.player_pos;
        self.state = save.state;
        self.menu_ui = save.menu_ui;
        self.char_creation_ui = save.char_creation_ui;
        self.journal_ui = save.journal_ui;
        self.settings_ui = save.settings_ui;
        Ok(())
    }

    pub fn queue_sound(&self, effect: SoundEffect) {
        if self.sound_enabled {
            self.sound_queue.borrow_mut().push(effect);
        }
    }

    pub fn modify_faction_rep(&mut self, faction: &str, delta: i32) {
        let key = format!("faction_{}_rep", faction);
        let cur = self.world_state.counter(&key);
        self.world_state.set_counter(&key, cur + delta);
    }

    pub fn check_room_hostilities(&mut self) {
        // Logic to check if room is hostile
    }

    pub fn tick_story_systems(&mut self) {
        let weather = WeatherType::from_region_tag(&self.region.weather);
        weather.apply_world_flags(&mut self.world_state);
        if weather == WeatherType::Ash && self.turn % 5 == 0 {
            self.player.conditions.insert(crate::game::character::conditions::Condition::Poisoned);
            self.set_feedback("Ash-choked air stings your lungs. You cough and lose focus.");
        } else if weather != WeatherType::Ash {
            self.player
                .conditions
                .remove(&crate::game::character::conditions::Condition::Poisoned);
        }

        self.world_events
            .tick(&mut self.world_state, &mut self.journal, self.turn);

        // Auto-accept quests whose first-stage transition condition is satisfied
        let unaccepted: Vec<String> = self
            .quests
            .defs
            .keys()
            .filter(|id| !self.quests.states.contains_key(id.as_str()))
            .cloned()
            .collect();
        for qid in unaccepted {
            if let Some(def) = self.quests.defs.get(&qid) {
                if let Some(first) = def.stages.first() {
                    let should_accept = first
                        .next
                        .iter()
                        .any(|t| self.world_state.evaluate(&t.condition));
                    if should_accept {
                        self.quests.accept_quest(
                            &qid,
                            &mut self.world_state,
                            &mut self.journal,
                            self.turn,
                        );
                    }
                }
            }
        }

        self.quests
            .tick(&mut self.world_state, &mut self.journal, self.turn);
    }
}
