use crate::data::types::{ClassDef, RaceDef};
use std::collections::HashMap;

pub fn sample_class_defs() -> HashMap<String, ClassDef> {
    let mut out = HashMap::new();
    out.insert(
        "fighter".into(),
        ClassDef {
            id: "fighter".into(),
            name: "Fighter".into(),
            hit_die: 10,
            primary_ability: "strength".into(),
            stat_growth: HashMap::from([("strength".into(), 1)]),
            special_ability_perk: "extra_attack".into(),
            special_ability_level: 5,
            saving_throw_proficiencies: vec!["strength".into(), "constitution".into()],
            armor_proficiencies: vec![],
            weapon_proficiencies: vec![],
            features: HashMap::new(),
            spell_slots: HashMap::new(),
        },
    );
    out.insert(
        "wizard".into(),
        ClassDef {
            id: "wizard".into(),
            name: "Wizard".into(),
            hit_die: 6,
            primary_ability: "intelligence".into(),
            stat_growth: HashMap::from([("intelligence".into(), 1)]),
            special_ability_perk: "lucky".into(),
            special_ability_level: 4,
            saving_throw_proficiencies: vec!["intelligence".into(), "wisdom".into()],
            armor_proficiencies: vec![],
            weapon_proficiencies: vec![],
            features: HashMap::new(),
            spell_slots: HashMap::new(),
        },
    );
    out.insert(
        "rogue".into(),
        ClassDef {
            id: "rogue".into(),
            name: "Rogue".into(),
            hit_die: 8,
            primary_ability: "dexterity".into(),
            stat_growth: HashMap::from([("dexterity".into(), 1)]),
            special_ability_perk: "mobile".into(),
            special_ability_level: 3,
            saving_throw_proficiencies: vec!["dexterity".into(), "intelligence".into()],
            armor_proficiencies: vec![],
            weapon_proficiencies: vec![],
            features: HashMap::new(),
            spell_slots: HashMap::new(),
        },
    );
    out
}

pub fn sample_race_defs() -> HashMap<String, RaceDef> {
    let mut out = HashMap::new();
    out.insert(
        "human".into(),
        RaceDef {
            id: "human".into(),
            name: "Human".into(),
            speed: 30,
            size: "medium".into(),
            ability_score_increases: HashMap::from([
                ("strength".into(), 1),
                ("dexterity".into(), 1),
                ("constitution".into(), 1),
                ("intelligence".into(), 1),
                ("wisdom".into(), 1),
                ("charisma".into(), 1),
            ]),
            level_growth_every: 4,
            level_growth: HashMap::from([("charisma".into(), 1)]),
            special_ability_perk: "alert".into(),
            special_ability_level: 5,
            traits: vec![],
            languages: vec!["common".into()],
        },
    );
    out.insert(
        "elf".into(),
        RaceDef {
            id: "elf".into(),
            name: "Elf".into(),
            speed: 30,
            size: "medium".into(),
            ability_score_increases: HashMap::from([
                ("dexterity".into(), 2),
                ("intelligence".into(), 1),
            ]),
            level_growth_every: 3,
            level_growth: HashMap::from([("dexterity".into(), 1)]),
            special_ability_perk: "lucky".into(),
            special_ability_level: 3,
            traits: vec![],
            languages: vec!["common".into(), "elvish".into()],
        },
    );
    out.insert(
        "dwarf".into(),
        RaceDef {
            id: "dwarf".into(),
            name: "Dwarf".into(),
            speed: 25,
            size: "medium".into(),
            ability_score_increases: HashMap::from([
                ("constitution".into(), 2),
                ("wisdom".into(), 1),
            ]),
            level_growth_every: 3,
            level_growth: HashMap::from([("constitution".into(), 1)]),
            special_ability_perk: "toughness".into(),
            special_ability_level: 3,
            traits: vec![],
            languages: vec!["common".into(), "dwarvish".into()],
        },
    );
    out
}
