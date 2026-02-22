use crate::game::character::{
    conditions::Condition,
    progression::proficiency_bonus,
    skills::{Skill, SkillSet, Perk},
};
use crate::game::items::inventory::Inventory;
use crate::game::items::equipment::{EquipmentSlot, EquipmentSlots};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use super::ability_scores::AbilityScores;
use super::saving_throws::SavingThrowProficiencies;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Character {
    pub name: String,
    pub class_id: String,
    pub race_id: String,
    pub level: u8,
    pub xp: u32,
    pub gold: u32,
    pub skill_points: u32,
    pub scores: AbilityScores,
    pub max_hp: i32,
    pub current_hp: i32,
    pub temp_hp: i32,
    pub speed: u32,
    pub skills: SkillSet,
    pub save_profs: SavingThrowProficiencies,
    pub conditions: HashSet<Condition>,
    pub perks: HashSet<Perk>,
    pub inventory: Inventory,
    pub equipment: EquipmentSlots,
    /// Spell slots remaining per level (index 0 = 1st-level).
    pub spell_slots: [u8; 9],
    /// Spell slots max per level.
    pub spell_slots_max: [u8; 9],
}

impl Character {
    /// Create a new Level 1 character with default stats.
    pub fn new(name: String, class_id: String, race_id: String, scores: AbilityScores) -> Self {
        let con_mod = AbilityScores::modifier(scores.constitution) as i32;
        let max_hp = 8 + con_mod; // default d8 hit die; caller should adjust by class
        Self {
            name,
            class_id,
            race_id,
            level: 1,
            xp: 0,
            gold: 10,
            skill_points: 0,
            scores,
            max_hp: max_hp.max(1),
            current_hp: max_hp.max(1),
            temp_hp: 0,
            speed: 30,
            skills: SkillSet::default(),
            save_profs: SavingThrowProficiencies::default(),
            conditions: HashSet::new(),
            perks: HashSet::new(),
            inventory: Inventory::default(),
            equipment: EquipmentSlots::default(),
            spell_slots: [0; 9],
            spell_slots_max: [0; 9],
        }
    }

    /// Proficiency bonus based on current level.
    pub fn proficiency_bonus(&self) -> i32 {
        proficiency_bonus(self.level)
    }

    /// Passive Perception = 10 + WIS modifier + proficiency (if proficient).
    pub fn passive_perception(&self) -> i32 {
        10 + self.scores.wis_mod() as i32
            + self
                .skills
                .bonus(Skill::Perception, self.proficiency_bonus())
    }

    /// Total skill check bonus for a given skill.
    pub fn skill_bonus(&self, skill: Skill) -> i32 {
        let ability_mod = self.scores.modifier_by_name(skill.ability()) as i32;
        ability_mod + self.skills.bonus(skill, self.proficiency_bonus())
    }

    /// Saving throw bonus for an ability (by name).
    pub fn save_bonus(&self, ability: &str) -> i32 {
        let base = self.scores.modifier_by_name(ability) as i32;
        if self.save_profs.is_proficient(ability) {
            base + self.proficiency_bonus()
        } else {
            base
        }
    }

    /// Apply damage, reducing temp HP first.
    /// Returns the actual HP remaining.
    pub fn take_damage(&mut self, amount: u32) -> i32 {
        let mut dmg = amount as i32;
        if self.temp_hp > 0 {
            let absorbed = dmg.min(self.temp_hp);
            self.temp_hp -= absorbed;
            dmg -= absorbed;
        }
        self.current_hp = (self.current_hp - dmg).max(0);
        if self.current_hp == 0 {
            self.conditions.insert(Condition::Unconscious);
        }
        self.current_hp
    }

    /// Heal by `amount`. Cannot exceed `max_hp`.
    pub fn heal(&mut self, amount: u32) {
        self.conditions.remove(&Condition::Unconscious);
        self.current_hp = (self.current_hp + amount as i32).min(self.max_hp);
    }

    /// Whether the character is alive and conscious.
    pub fn is_conscious(&self) -> bool {
        self.current_hp > 0 && !self.conditions.contains(&Condition::Unconscious)
    }

    /// Whether any actions are possible (not incapacitated).
    pub fn can_act(&self) -> bool {
        !self.conditions.iter().any(|c| c.is_incapacitating())
    }
}