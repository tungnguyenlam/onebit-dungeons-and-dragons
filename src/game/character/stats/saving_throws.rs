use crate::game::character::{
    conditions::Condition, progression::proficiency_bonus, skills::Skill,
};
use crate::game::items::equipment::EquipmentSlot;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingThrowProficiencies {
    pub strength: bool,
    pub dexterity: bool,
    pub constitution: bool,
    pub intelligence: bool,
    pub wisdom: bool,
    pub charisma: bool,
}

impl SavingThrowProficiencies {
    pub fn is_proficient(&self, ability: &str) -> bool {
        match ability {
            "strength" => self.strength,
            "dexterity" => self.dexterity,
            "constitution" => self.constitution,
            "intelligence" => self.intelligence,
            "wisdom" => self.wisdom,
            "charisma" => self.charisma,
            _ => false,
        }
    }
}
