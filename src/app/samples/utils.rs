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

pub fn find_spawn_pos_for_room(room: &crate::game::world::room::Room) -> (u32, u32) {
    if let Some((col, row, _)) = room
        .grid
        .iter()
        .find(|(_, _, tile)| *tile == crate::game::world::map::Tile::NpcSpawn)
    {
        return (col, row);
    }
    (1, 1)
}
