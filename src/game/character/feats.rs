use crate::data::types::FeatDef;
use crate::game::character::stats::Character;
use std::collections::HashMap;

pub struct FeatRegistry {
    feats: HashMap<String, FeatDef>,
}

impl FeatRegistry {
    pub fn new(feats: HashMap<String, FeatDef>) -> Self {
        Self { feats }
    }

    pub fn get(&self, feat_id: &str) -> Option<&FeatDef> {
        self.feats.get(feat_id)
    }

    pub fn all(&self) -> impl Iterator<Item = &FeatDef> {
        self.feats.values()
    }
}

pub fn apply_feat_effect(character: &mut Character, feat_def: &FeatDef) {
    // Handle mechanical effects of feats
    if feat_def.id == "tough" {
        // Tough feat: +2 HP per level
        let hp_bonus = character.total_level as i32 * 2;
        character.max_hp += hp_bonus;
        character.current_hp += hp_bonus;
    }

    // Add other feat effects here as needed
    if feat_def.id == "great_weapon_master" {
        // Great Weapon Master: Add to perks or handle in combat
    }

    if feat_def.id == "magic_initiate" {
        // Magic Initiate: Add cantrips or spells
    }
}

pub fn meets_feat_prerequisites(_character: &Character, feat_def: &FeatDef) -> bool {
    // For now, just check if prerequisites list is empty (always true)
    // Future implementation: Parse and evaluate prerequisite conditions
    feat_def.prerequisites.is_empty()
}
