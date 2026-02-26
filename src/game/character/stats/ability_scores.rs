use crate::game::character::{
    conditions::Condition, progression::proficiency_bonus, skills::Skill,
};
use crate::game::items::equipment::EquipmentSlot;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AbilityScores {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

impl AbilityScores {
    /// Standard array: [15, 14, 13, 12, 10, 8].
    pub fn standard_array() -> Self {
        Self {
            strength: 15,
            dexterity: 14,
            constitution: 13,
            intelligence: 12,
            wisdom: 10,
            charisma: 8,
        }
    }

    /// Ability modifier for a raw score: `floor((score − 10) / 2)`.
    pub fn modifier(score: u8) -> i8 {
        (score as i8 - 10) / 2
    }

    pub fn str_mod(&self) -> i8 {
        Self::modifier(self.strength)
    }
    pub fn dex_mod(&self) -> i8 {
        Self::modifier(self.dexterity)
    }
    pub fn con_mod(&self) -> i8 {
        Self::modifier(self.constitution)
    }
    pub fn int_mod(&self) -> i8 {
        Self::modifier(self.intelligence)
    }
    pub fn wis_mod(&self) -> i8 {
        Self::modifier(self.wisdom)
    }
    pub fn cha_mod(&self) -> i8 {
        Self::modifier(self.charisma)
    }

    /// Get modifier by ability name (lowercase).
    pub fn modifier_by_name(&self, name: &str) -> i8 {
        match name {
            "strength" => self.str_mod(),
            "dexterity" => self.dex_mod(),
            "constitution" => self.con_mod(),
            "intelligence" => self.int_mod(),
            "wisdom" => self.wis_mod(),
            "charisma" => self.cha_mod(),
            _ => 0,
        }
    }
}
