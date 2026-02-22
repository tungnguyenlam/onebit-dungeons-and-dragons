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

pub fn demo_world_events() -> EventEngine {
    EventEngine {
        triggers: vec![
            EventTrigger {
                condition: "counter:faction_town_guard_rep >= 3".into(),
                event: WorldEvent::AddJournalEntry {
                    id: "faction-town-guard-friendly".into(),
                    category: JournalCategory::World,
                    title: "Town Guard Trust".into(),
                    body: "The town guard now recognizes your service and offers support.".into(),
                },
                once: true,
                fired: false,
            },
            EventTrigger {
                condition: "counter:faction_town_guard_rep >= 2".into(),
                event: WorldEvent::SetFlag {
                    key: "town_guard_trusted".into(),
                },
                once: true,
                fired: false,
            },
            EventTrigger {
                condition: "counter:faction_goblin_tribe_rep <= -4".into(),
                event: WorldEvent::SetFlag {
                    key: "goblin_tribe_hostile".into(),
                },
                once: true,
                fired: false,
            },
            EventTrigger {
                condition: "flag:town_guard_trusted && flag:read_ember_rune".into(),
                event: WorldEvent::AddJournalEntry {
                    id: "chain-emberpeak-briefing".into(),
                    category: JournalCategory::World,
                    title: "Joint War Council".into(),
                    body: "The guard and summit wardens coordinate supply lines through Emberpeak."
                        .into(),
                },
                once: true,
                fired: false,
            },
            EventTrigger {
                condition: "flag:town_guard_trusted && counter:faction_goblin_tribe_rep <= -3"
                    .into(),
                event: WorldEvent::SetFlag {
                    key: "valley_warfront".into(),
                },
                once: true,
                fired: false,
            },
        ],
    }
}