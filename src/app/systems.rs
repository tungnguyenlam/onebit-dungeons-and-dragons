use crate::app::App;
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
        self.world_events
            .tick(&mut self.world_state, &mut self.journal, self.turn);
        self.quests
            .tick(&mut self.world_state, &mut self.journal, self.turn);
    }
}
