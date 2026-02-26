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

pub fn sample_region_bundle() -> (Region, HashMap<String, NpcDef>, HashMap<String, DialogTree>) {
    let loaded = load_region("assets", "valley-of-ash").ok();
    if let Some(loaded) = loaded {
        return (Region::from_loaded(&loaded), loaded.npcs, loaded.dialogs);
    }

    let fallback = crate::data::loader::LoadedRegion {
        manifest: crate::data::types::RegionManifest {
            slug: "fallback".into(),
            name: "Fallback Region".into(),
            description: "Fallback region when assets are unavailable.".into(),
            entry_room: "start".into(),
            ambient: "".into(),
            region_type: "dungeon".into(),
            weather: "none".into(),
            rooms: vec![crate::data::types::RoomRef {
                id: "start".into(),
                file: "rooms/start.toml".into(),
            }],
            connections: vec![],
        },
        rooms: {
            let mut map = HashMap::new();
            map.insert(
                "start".into(),
                crate::data::types::RoomDef {
                    id: "start".into(),
                    name: "Start".into(),
                    description: "Fallback room".into(),
                    landmark: "Fallback Campfire".into(),
                    grid: "#####\n#...#\n#.@.#\n#####\n".into(),
                    terminal: false,
                    npcs: vec![],
                    items: vec![],
                    triggers: vec![],
                    exits: crate::data::types::RoomExits::default(),
                },
            );
            map
        },
        npcs: HashMap::new(),
        dialogs: HashMap::new(),
    };
    (
        Region::from_loaded(&fallback),
        HashMap::new(),
        HashMap::new(),
    )
}
