use crate::data::loader::{load_monsters, load_region};
use crate::data::types::{
    ArmorDef, ArmorType, DialogTree, ItemBonuses, ItemDef, ItemType, MonsterAction, MonsterDef,
    NpcDef, SpellDef, WeaponDef,
}; // Careful: check types here
use crate::game::{
    character::{conditions::Condition, AbilityScores},
    combat::CombatantState,
    combat::EnemyAiRole,
    dice::DiceExpr,
    story::events::{EventEngine, EventTrigger, WorldEvent},
    story::journal::Category as JournalCategory,
    world::region::Region,
};
use std::collections::HashMap;

pub fn sample_spell_defs() -> HashMap<String, SpellDef> {
    let mut map = HashMap::new();
    map.insert(
        "cure_wounds".into(),
        SpellDef {
            id: "cure_wounds".into(),
            name: "Cure Wounds".into(),
            level: 1,
            school: "evocation".into(),
            casting_time: "action".into(),
            range: "touch".into(),
            components: vec!["V".into(), "S".into()],
            duration: "instantaneous".into(),
            description: "Healing energy restores HP.".into(),
            damage: None,
            damage_type: None,
            save: None,
            heal: Some(DiceExpr::new(1, 8, 2)),
            classes: vec!["cleric".into()],
        },
    );
    map.insert(
        "fire_bolt".into(),
        SpellDef {
            id: "fire_bolt".into(),
            name: "Fire Bolt".into(),
            level: 0,
            school: "evocation".into(),
            casting_time: "action".into(),
            range: "120ft".into(),
            components: vec!["V".into(), "S".into()],
            duration: "instantaneous".into(),
            description: "A mote of fire.".into(),
            damage: Some(DiceExpr::new(1, 10, 0)),
            damage_type: Some("fire".into()),
            save: None,
            heal: None,
            classes: vec!["wizard".into()],
        },
    );
    map.insert(
        "poison_spray".into(),
        SpellDef {
            id: "poison_spray".into(),
            name: "Poison Spray".into(),
            level: 0,
            school: "conjuration".into(),
            casting_time: "action".into(),
            range: "10ft".into(),
            components: vec!["V".into(), "S".into()],
            duration: "instantaneous".into(),
            description: "Noxious gas.".into(),
            damage: None,
            damage_type: None,
            save: Some("constitution".into()),
            heal: None,
            classes: vec!["wizard".into()],
        },
    );
    map
}