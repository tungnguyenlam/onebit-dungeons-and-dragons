use crate::app::App;
use crate::game::character::progression::level_for_xp;

impl App {
    pub fn apply_character_creation(&mut self) {
        self.player.name = self.char_creation_ui.name.clone();
        let class_id =
            self.char_creation_ui.class_options[self.char_creation_ui.class_index].clone();
        self.player.classes = vec![crate::data::types::ClassLevel { class_id, level: 1 }];
        self.player.update_total_level();
        self.player.race_id =
            self.char_creation_ui.race_options[self.char_creation_ui.race_index].clone();
    }

    pub fn grant_player_xp(&mut self, gained_xp: u32) {
        self.player.xp += gained_xp;
        let old_level = self.player.total_level;
        let new_level = level_for_xp(self.player.xp);

        if new_level > old_level {
            let levels_gained = new_level - old_level;
            // For now, simplify and add levels to the main class
            if let Some(cl) = self.player.classes.first_mut() {
                cl.level += levels_gained;
            }
            self.player.update_total_level();
            
            self.player.max_hp += 8 * levels_gained as i32;
            self.player.current_hp = self.player.max_hp;
            self.player.skill_points += (levels_gained * 2) as u32;

            self.set_feedback(&format!(
                "Leveled up to {}! +{} HP, +{} skill points",
                new_level,
                8 * levels_gained as i32,
                levels_gained * 2
            ));
        }

        let gold_found = gained_xp / 10;
        if gold_found > 0 {
            self.player.gold += gold_found;
        }
    }
}
