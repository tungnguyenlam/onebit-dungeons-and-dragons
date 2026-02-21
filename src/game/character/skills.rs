/// Skill definitions and skill-check helpers.
///
/// Each skill maps to one of the six ability scores.
/// `SkillSet` tracks proficiency and expertise per skill.
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Skill {
    // STR
    Athletics,
    // DEX
    Acrobatics,
    SleightOfHand,
    Stealth,
    // INT
    Arcana,
    History,
    Investigation,
    Nature,
    Religion,
    // WIS
    AnimalHandling,
    Insight,
    Medicine,
    Perception,
    Survival,
    // CHA
    Deception,
    Intimidation,
    Performance,
    Persuasion,
}

impl Skill {
    /// The ability score this skill is based on.
    pub fn ability(self) -> &'static str {
        match self {
            Skill::Athletics => "strength",
            Skill::Acrobatics | Skill::SleightOfHand | Skill::Stealth => "dexterity",
            Skill::Arcana
            | Skill::History
            | Skill::Investigation
            | Skill::Nature
            | Skill::Religion => "intelligence",
            Skill::AnimalHandling
            | Skill::Insight
            | Skill::Medicine
            | Skill::Perception
            | Skill::Survival => "wisdom",
            Skill::Deception | Skill::Intimidation | Skill::Performance | Skill::Persuasion => {
                "charisma"
            }
        }
    }

    /// Human-readable display name.
    pub fn display_name(self) -> &'static str {
        match self {
            Skill::Athletics => "Athletics",
            Skill::Acrobatics => "Acrobatics",
            Skill::SleightOfHand => "Sleight of Hand",
            Skill::Stealth => "Stealth",
            Skill::Arcana => "Arcana",
            Skill::History => "History",
            Skill::Investigation => "Investigation",
            Skill::Nature => "Nature",
            Skill::Religion => "Religion",
            Skill::AnimalHandling => "Animal Handling",
            Skill::Insight => "Insight",
            Skill::Medicine => "Medicine",
            Skill::Perception => "Perception",
            Skill::Survival => "Survival",
            Skill::Deception => "Deception",
            Skill::Intimidation => "Intimidation",
            Skill::Performance => "Performance",
            Skill::Persuasion => "Persuasion",
        }
    }

    /// All 18 skills, in order.
    pub fn all() -> &'static [Skill] {
        &[
            Skill::Athletics,
            Skill::Acrobatics,
            Skill::SleightOfHand,
            Skill::Stealth,
            Skill::Arcana,
            Skill::History,
            Skill::Investigation,
            Skill::Nature,
            Skill::Religion,
            Skill::AnimalHandling,
            Skill::Insight,
            Skill::Medicine,
            Skill::Perception,
            Skill::Survival,
            Skill::Deception,
            Skill::Intimidation,
            Skill::Performance,
            Skill::Persuasion,
        ]
    }
}

/// Tracks proficiency / expertise per skill.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SkillSet {
    pub proficient: HashSet<Skill>,
    pub expertise: HashSet<Skill>, // double proficiency
}

impl SkillSet {
    /// Bonus for a skill: 0, proficiency, or 2× proficiency (expertise).
    pub fn bonus(&self, skill: Skill, proficiency_bonus: i32) -> i32 {
        if self.expertise.contains(&skill) {
            proficiency_bonus * 2
        } else if self.proficient.contains(&skill) {
            proficiency_bonus
        } else {
            0
        }
    }

    pub fn is_proficient(&self, skill: Skill) -> bool {
        self.proficient.contains(&skill)
    }

    pub fn is_expert(&self, skill: Skill) -> bool {
        self.expertise.contains(&skill)
    }
}
