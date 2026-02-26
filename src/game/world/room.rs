/// Runtime room built from a deserialized `RoomDef` asset.
///
/// `Room` wraps a `TileGrid` (parsed from the inline `grid` string) together
/// with the NPC, item, and trigger metadata declared in the same TOML file.
/// It owns its grid and all spawn lists so that multiple rooms can live in
/// memory at the same time without cross-borrow issues.
///
/// Conversion: `Room::from_def(&RoomDef)` — cheap clone, done once at
/// region-load time.
use crate::data::types::{RoomDef, RoomExits, RoomItem, RoomNpc, TriggerDef};
use crate::game::world::map::TileGrid;

// ---------------------------------------------------------------------------
// Room
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub description: String,
    pub landmark: String,
    /// The parsed tile grid for this room.
    pub grid: TileGrid,
    /// NPC spawn points declared in the TOML, in load order.
    pub npcs: Vec<RoomNpc>,
    /// Item spawn points.
    pub items: Vec<RoomItem>,
    /// Trigger zones (dialog, encounter, lore, quest_stage, travel).
    pub triggers: Vec<TriggerDef>,
    pub exits: RoomExits,
    /// When true this is a deliberate dead-end; no outbound travel trigger required.
    pub terminal: bool,
}

impl Room {
    /// Build a `Room` from a fully-deserialized `RoomDef`.
    ///
    /// The grid string is parsed eagerly; all metadata vecs are cloned from
    /// the def so that the def can be dropped afterwards.
    pub fn from_def(def: &RoomDef) -> Self {
        Room {
            id: def.id.clone(),
            name: def.name.clone(),
            description: def.description.clone(),
            landmark: def.landmark.clone(),
            grid: TileGrid::from_str(&def.grid),
            npcs: def.npcs.clone(),
            items: def.items.clone(),
            triggers: def.triggers.clone(),
            exits: def.exits.clone(),
            terminal: def.terminal,
        }
    }

    /// Width of the tile grid in columns.
    pub fn width(&self) -> u32 {
        self.grid.width
    }

    /// Height of the tile grid in rows.
    pub fn height(&self) -> u32 {
        self.grid.height
    }

    /// Find the trigger at `(col, row)`, if any.
    pub fn trigger_at(&self, col: u32, row: u32) -> Option<&TriggerDef> {
        self.triggers
            .iter()
            .find(|t| t.position[0] == col && t.position[1] == row)
    }

    /// Find the NPC spawn at `(col, row)`, if any.
    pub fn npc_at(&self, col: u32, row: u32) -> Option<&RoomNpc> {
        self.npcs
            .iter()
            .find(|n| n.position[0] == col && n.position[1] == row)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::TriggerKind;

    fn make_def() -> RoomDef {
        RoomDef {
            id: "test-room".into(),
            name: "Test Room".into(),
            description: "A room for testing.".into(),
            landmark: "Ancient Test Obelisk".into(),
            grid: "##########\n#........#\n#..@..!..#\n#........#\n##########\n".into(),
            terminal: false,
            npcs: vec![RoomNpc {
                id: "guard".into(),
                position: [3, 2],
            }],
            items: vec![],
            triggers: vec![TriggerDef {
                position: [6, 2],
                kind: TriggerKind::Dialog,
                target_id: "npc_dialog".into(),
                condition: String::new(),
                once: true,
            }],
            exits: RoomExits::default(),
        }
    }

    #[test]
    fn dimensions_parsed() {
        let room = Room::from_def(&make_def());
        assert_eq!(room.width(), 10);
        assert_eq!(room.height(), 5);
    }

    #[test]
    fn npc_at_lookup() {
        let room = Room::from_def(&make_def());
        assert!(room.npc_at(3, 2).is_some());
        assert!(room.npc_at(0, 0).is_none());
    }

    #[test]
    fn trigger_at_lookup() {
        let room = Room::from_def(&make_def());
        let trig = room.trigger_at(6, 2);
        assert!(trig.is_some());
        assert_eq!(trig.unwrap().kind, TriggerKind::Dialog);
    }
}
