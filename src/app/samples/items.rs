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

pub fn sample_item_defs() -> HashMap<String, ItemDef> {
    let mut map = HashMap::new();
    map.insert(
        "longsword".into(),
        ItemDef {
            id: "longsword".into(),
            name: "Longsword".into(),
            item_type: ItemType::Weapon,
            weight: 3.0,
            value_gp: 15,
            description: "A standard steel longsword.".into(),
            weapon: Some(WeaponDef {
                damage: DiceExpr::new(1, 8, 0),
                damage_type: "slashing".into(),
                properties: vec!["versatile".into()],
                versatile_damage: Some(DiceExpr::new(1, 10, 0)),
                range: None,
            }),
            armor: None,
            bonuses: ItemBonuses::default(),
        },
    );
    map.insert(
        "leather_armor".into(),
        ItemDef {
            id: "leather_armor".into(),
            name: "Leather Armor".into(),
            item_type: ItemType::Armor,
            weight: 10.0,
            value_gp: 10,
            description: "Flexible light armor.".into(),
            weapon: None,
            armor: Some(ArmorDef {
                base_ac: 11,
                armor_type: ArmorType::Light,
                stealth_disadvantage: false,
            }),
            bonuses: ItemBonuses::default(),
        },
    );
    map.insert(
        "shield".into(),
        ItemDef {
            id: "shield".into(),
            name: "Shield".into(),
            item_type: ItemType::Armor,
            weight: 6.0,
            value_gp: 10,
            description: "Wooden shield.".into(),
            weapon: None,
            armor: Some(ArmorDef {
                base_ac: 2,
                armor_type: ArmorType::Shield,
                stealth_disadvantage: false,
            }),
            bonuses: ItemBonuses {
                armor_class_bonus: 0,
                ..ItemBonuses::default()
            },
        },
    );
    map.insert(
        "healing_potion".into(),
        ItemDef {
            id: "healing_potion".into(),
            name: "Healing Potion".into(),
            item_type: ItemType::Consumable,
            weight: 0.5,
            value_gp: 50,
            description: "Restores health.".into(),
            weapon: None,
            armor: None,
            bonuses: ItemBonuses::default(),
        },
    );
    map
}