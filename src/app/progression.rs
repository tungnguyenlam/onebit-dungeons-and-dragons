use crate::app::App;
use crate::game::character::feats::apply_feat_effect;
use crate::game::character::skills::Perk;
use crate::game::character::progression::{is_asi_level, level_for_xp};

impl App {
    pub fn apply_character_creation(&mut self) {
        self.player.name = self.char_creation_ui.name.clone();
        let class_id =
            self.char_creation_ui.class_options[self.char_creation_ui.class_index].clone();
        self.player.classes = vec![crate::data::types::ClassLevel { class_id, level: 1 }];
        self.player.update_total_level();
        self.player.race_id =
            self.char_creation_ui.race_options[self.char_creation_ui.race_index].clone();
        let con_before = self.player.scores.con_mod() as i32;
        self.apply_race_starting_bonuses();
        let con_after = self.player.scores.con_mod() as i32;
        if con_after > con_before {
            let delta = con_after - con_before;
            self.player.max_hp += delta * self.player.total_level as i32;
            self.player.current_hp += delta * self.player.total_level as i32;
        }
        self.refresh_derived_stats();
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
            self.apply_level_based_stat_growth(levels_gained);
            let hit_die = self
                .class_defs
                .get(self.player.main_class())
                .map(|c| c.hit_die as i32)
                .unwrap_or(8);
            let per_level_hp = ((hit_die / 2) + 1 + self.player.scores.con_mod() as i32).max(1);
            self.player.max_hp += per_level_hp * levels_gained as i32;
            self.player.current_hp = self.player.max_hp;
            self.player.skill_points += (levels_gained * 2) as u32;

            // Apply any level-based feat effects (e.g., Tough feat)
            let feats_to_apply: Vec<String> = self.player.feats.clone();
            for feat_id in feats_to_apply {
                if let Some(feat_def) = self.feat_defs.get(&feat_id) {
                    apply_feat_effect(&mut self.player, feat_def);
                }
            }

            self.set_feedback(&format!(
                "Leveled up to {}! +{} free stat points (1-6 to allocate)",
                new_level,
                levels_gained * 2
            ));
        }

        let gold_found = gained_xp / 10;
        if gold_found > 0 {
            self.player.gold += gold_found;
        }
    }

    pub fn grant_feat(&mut self, feat_id: &str) {
        if !self.player.feats.contains(&feat_id.to_string()) {
            self.player.feats.push(feat_id.to_string());
            if let Some(feat_def) = self.feat_defs.get(feat_id) {
                apply_feat_effect(&mut self.player, feat_def);
                self.set_feedback(&format!("Learned feat: {}", feat_def.name));
            }
        }
    }

    pub fn allocate_stat_point(&mut self, ability: &str) -> bool {
        if self.player.skill_points == 0 {
            return false;
        }
        let old_con_mod = self.player.scores.con_mod() as i32;
        if self.player.scores.increase_by_name(ability, 1, 20) {
            self.player.skill_points -= 1;
            let new_con_mod = self.player.scores.con_mod() as i32;
            if new_con_mod > old_con_mod {
                let delta = new_con_mod - old_con_mod;
                self.player.max_hp += delta * self.player.total_level as i32;
                self.player.current_hp += delta * self.player.total_level as i32;
            }
            self.refresh_derived_stats();
            return true;
        }
        false
    }

    fn apply_race_starting_bonuses(&mut self) {
        let Some(race) = self.race_defs.get(&self.player.race_id) else {
            return;
        };
        for (ability, bonus) in &race.ability_score_increases {
            if *bonus > 0 {
                let _ = self
                    .player
                    .scores
                    .increase_by_name(ability, *bonus as u8, 20);
            }
        }
    }

    fn apply_level_based_stat_growth(&mut self, levels_gained: u8) {
        let main_class = self.player.main_class().to_string();
        let class_def = self.class_defs.get(&main_class).cloned();
        for i in 0..levels_gained {
            let level_now = self.player.total_level.saturating_sub(levels_gained) + i + 1;
            if let Some(class_def) = &class_def {
                if class_def.stat_growth.is_empty() {
                    let _ = self
                        .player
                        .scores
                        .increase_by_name(&class_def.primary_ability, 1, 20);
                } else {
                    for (ability, amt) in &class_def.stat_growth {
                        let _ = self.player.scores.increase_by_name(ability, *amt, 20);
                    }
                }
                if class_def.special_ability_level > 0
                    && level_now >= class_def.special_ability_level
                    && !class_def.special_ability_perk.is_empty()
                {
                    if let Some(perk) = perk_from_id(&class_def.special_ability_perk) {
                        self.player.perks.insert(perk);
                    }
                }
            } else {
                let _ = self.player.scores.increase_by_name("strength", 1, 20);
            }

            if let Some(race) = self.race_defs.get(&self.player.race_id) {
                if race.level_growth_every > 0 && level_now % race.level_growth_every == 0 {
                    for (ability, amt) in &race.level_growth {
                        let _ = self.player.scores.increase_by_name(ability, *amt, 20);
                    }
                }
                if race.special_ability_level > 0
                    && level_now >= race.special_ability_level
                    && !race.special_ability_perk.is_empty()
                {
                    if let Some(perk) = perk_from_id(&race.special_ability_perk) {
                        self.player.perks.insert(perk);
                    }
                }
            }
        }
    }

    fn refresh_derived_stats(&mut self) {
        self.player.speed = self
            .race_defs
            .get(&self.player.race_id)
            .map(|r| r.speed)
            .unwrap_or(30);
    }
}

fn perk_from_id(id: &str) -> Option<Perk> {
    match id {
        "extra_attack" => Some(Perk::ExtraAttack),
        "toughness" => Some(Perk::Toughness),
        "lucky" => Some(Perk::Lucky),
        "mobile" => Some(Perk::Mobile),
        "alert" => Some(Perk::Alert),
        "heavy_armor_prof" => Some(Perk::HeavyArmorProf),
        "shield_prof" => Some(Perk::ShieldProf),
        "martial_weapon_prof" => Some(Perk::MartialWeaponProf),
        "dual_wielder" => Some(Perk::DualWielder),
        "grappler" => Some(Perk::Grappler),
        _ => None,
    }
}
