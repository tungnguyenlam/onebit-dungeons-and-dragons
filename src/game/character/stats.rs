/// Ability scores, derived stats, and the full `Character` struct.
///
/// See [docs/gameplay/character.md] for the game rules.
use crate::game::character::{
    conditions::Condition,
    progression::proficiency_bonus,
    skills::{Skill, SkillSet},
};
use crate::game::items::{equipment::EquipmentSlots, inventory::Inventory};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// ---------------------------------------------------------------------------
// AbilityScores
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AbilityScores {
    pub strength:     u8,
    pub dexterity:    u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom:       u8,
    pub charisma:     u8,
}

impl AbilityScores {
    /// Standard array: [15, 14, 13, 12, 10, 8].
    pub fn standard_array() -> Self {
        Self {
            strength:     15,
            dexterity:    14,
            constitution: 13,
            intelligence: 12,
            wisdom:       10,
            charisma:     8,
        }
    }

    /// Ability modifier for a raw score: `floor((score − 10) / 2)`.
    pub fn modifier(score: u8) -> i8 {
        (score as i8 - 10) / 2
    }

    pub fn str_mod(&self) -> i8 { Self::modifier(self.strength) }
    pub fn dex_mod(&self) -> i8 { Self::modifier(self.dexterity) }
    pub fn con_mod(&self) -> i8 { Self::modifier(self.constitution) }
    pub fn int_mod(&self) -> i8 { Self::modifier(self.intelligence) }
    pub fn wis_mod(&self) -> i8 { Self::modifier(self.wisdom) }
    pub fn cha_mod(&self) -> i8 { Self::modifier(self.charisma) }

    /// Get modifier by ability name (lowercase).
    pub fn modifier_by_name(&self, name: &str) -> i8 {
        match name {
            "strength"     => self.str_mod(),
            "dexterity"    => self.dex_mod(),
            "constitution" => self.con_mod(),
            "intelligence" => self.int_mod(),
            "wisdom"       => self.wis_mod(),
            "charisma"     => self.cha_mod(),
            _              => 0,
        }
    }
}

// ---------------------------------------------------------------------------
// SavingThrows
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingThrowProficiencies {
    pub strength:     bool,
    pub dexterity:    bool,
    pub constitution: bool,
    pub intelligence: bool,
    pub wisdom:       bool,
    pub charisma:     bool,
}

impl SavingThrowProficiencies {
    pub fn is_proficient(&self, ability: &str) -> bool {
        match ability {
            "strength"     => self.strength,
            "dexterity"    => self.dexterity,
            "constitution" => self.constitution,
            "intelligence" => self.intelligence,
            "wisdom"       => self.wisdom,
            "charisma"     => self.charisma,
            _              => false,
        }
    }
}

// ---------------------------------------------------------------------------
// Character
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Character {
    pub name:          String,
    pub class_id:      String,
    pub race_id:       String,
    pub level:         u8,
    pub xp:            u32,
    pub scores:        AbilityScores,
    pub max_hp:        i32,
    pub current_hp:    i32,
    pub temp_hp:       i32,
    pub speed:         u32,
    pub skills:        SkillSet,
    pub save_profs:    SavingThrowProficiencies,
    pub conditions:    HashSet<Condition>,
    pub inventory:     Inventory,
    pub equipment:     EquipmentSlots,
    /// Spell slots remaining per level (index 0 = 1st-level).
    pub spell_slots:   [u8; 9],
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
            scores,
            max_hp: max_hp.max(1),
            current_hp: max_hp.max(1),
            temp_hp: 0,
            speed: 30,
            skills: SkillSet::default(),
            save_profs: SavingThrowProficiencies::default(),
            conditions: HashSet::new(),
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
            + self.skills.bonus(Skill::Perception, self.proficiency_bonus())
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_char() -> Character {
        Character::new(
            "Theron".into(),
            "fighter".into(),
            "human".into(),
            AbilityScores {
                strength: 16, dexterity: 14, constitution: 14,
                intelligence: 10, wisdom: 12, charisma: 8,
            },
        )
    }

    #[test]
    fn modifier_formula() {
        assert_eq!(AbilityScores::modifier(10), 0);
        assert_eq!(AbilityScores::modifier(12), 1);
        assert_eq!(AbilityScores::modifier(8),  -1);
        assert_eq!(AbilityScores::modifier(20), 5);
    }

    #[test]
    fn take_damage_temp_hp() {
        let mut c = test_char();
        c.temp_hp = 5;
        c.take_damage(3);
        assert_eq!(c.temp_hp, 2);
        assert_eq!(c.current_hp, c.max_hp);
    }

    #[test]
    fn take_damage_to_zero_adds_unconscious() {
        let mut c = test_char();
        c.take_damage(c.current_hp as u32 + 10);
        assert_eq!(c.current_hp, 0);
        assert!(c.conditions.contains(&Condition::Unconscious));
    }

    #[test]
    fn heal_removes_unconscious() {
        let mut c = test_char();
        c.conditions.insert(Condition::Unconscious);
        c.heal(1);
        assert!(!c.conditions.contains(&Condition::Unconscious));
    }
}
