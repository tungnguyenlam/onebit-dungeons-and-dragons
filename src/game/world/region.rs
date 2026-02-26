/// Runtime region built from a `LoadedRegion`.
///
/// A `Region` holds every room in memory (as `Room` structs with parsed
/// `TileGrid`s) together with the region manifest metadata and the list of
/// exit connections to neighbouring regions.
///
/// Construction: `Region::from_loaded(&LoadedRegion)` — called once per
/// region transition; rooms are stored in a `HashMap` keyed by room id.
use std::collections::HashMap;

use crate::data::loader::LoadedRegion;
use crate::data::types::RegionConnection;
use crate::game::world::room::Room;

// ---------------------------------------------------------------------------
// Region
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Region {
    /// The unique folder-name slug, e.g. `"valley-of-ash"`.
    pub slug: String,
    pub name: String,
    pub description: String,
    /// Id of the room where the player spawns when entering this region.
    pub entry_room: String,
    /// Optional ambient flavour tag (passed to the audio sub-system).
    pub ambient: String,
    /// Region type for visual themes (volcanic, forest, underwater, underground, mountain)
    pub region_type: String,
    /// Weather effect (ash, fog, rain, none)
    pub weather: String,
    /// Exit connections to neighbouring regions.
    pub connections: Vec<RegionConnection>,
    /// All rooms in this region, keyed by room id.
    pub rooms: HashMap<String, Room>,
}

impl Region {
    /// Build a `Region` from a fully-loaded `LoadedRegion` bundle.
    ///
    /// Every `RoomDef` in `lr.rooms` is converted to a `Room` (which parses
    /// its tile grid eagerly). The loader bundle can be dropped afterwards.
    pub fn from_loaded(lr: &LoadedRegion) -> Self {
        let rooms: HashMap<String, Room> = lr
            .rooms
            .values()
            .map(|def| (def.id.clone(), Room::from_def(def)))
            .collect();

        Region {
            slug: lr.manifest.slug.clone(),
            name: lr.manifest.name.clone(),
            description: lr.manifest.description.clone(),
            entry_room: lr.manifest.entry_room.clone(),
            ambient: lr.manifest.ambient.clone(),
            region_type: lr.manifest.region_type.clone(),
            weather: lr.manifest.weather.clone(),
            connections: lr.manifest.connections.clone(),
            rooms,
        }
    }

    /// Get a shared reference to a room by id.
    pub fn room(&self, id: &str) -> Option<&Room> {
        self.rooms.get(id)
    }

    /// Get a mutable reference to a room by id.
    pub fn room_mut(&mut self, id: &str) -> Option<&mut Room> {
        self.rooms.get_mut(id)
    }

    /// Get the entry room.
    pub fn entry(&self) -> Option<&Room> {
        self.rooms.get(&self.entry_room)
    }

    /// List all exit connections from a specific room id.
    pub fn exits_from<'a>(&'a self, room_id: &str) -> Vec<&'a RegionConnection> {
        self.connections
            .iter()
            .filter(|c| c.from_room == room_id)
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::loader::LoadedRegion;
    use crate::data::types::{RegionManifest, RoomDef, RoomRef};
    use std::collections::HashMap;

    fn make_loaded_region() -> LoadedRegion {
        let manifest = RegionManifest {
            slug: "test-region".into(),
            name: "Test Region".into(),
            description: "For testing.".into(),
            entry_room: "start".into(),
            ambient: String::new(),
            region_type: "dungeon".into(),
            weather: "none".into(),
            rooms: vec![RoomRef {
                id: "start".into(),
                file: "rooms/start.toml".into(),
            }],
            connections: vec![],
        };

        let mut rooms = HashMap::new();
        rooms.insert(
            "start".into(),
            RoomDef {
                id: "start".into(),
                name: "Start Room".into(),
                description: "The beginning.".into(),
                landmark: "The Cracked Waystone".into(),
                grid: "#####\n#...#\n#####\n".into(),
                terminal: false,
                npcs: vec![],
                items: vec![],
                triggers: vec![],
            },
        );

        LoadedRegion {
            manifest,
            rooms,
            npcs: HashMap::new(),
            dialogs: HashMap::new(),
        }
    }

    #[test]
    fn region_built_from_loader() {
        let lr = make_loaded_region();
        let reg = Region::from_loaded(&lr);
        assert_eq!(reg.slug, "test-region");
        assert!(reg.room("start").is_some());
        assert!(reg.room("nonexistent").is_none());
    }

    #[test]
    fn entry_room_accessible() {
        let lr = make_loaded_region();
        let reg = Region::from_loaded(&lr);
        assert!(reg.entry().is_some());
        assert_eq!(reg.entry().unwrap().id, "start");
    }

    #[test]
    fn exits_from_empty_when_no_connections() {
        let lr = make_loaded_region();
        let reg = Region::from_loaded(&lr);
        assert!(reg.exits_from("start").is_empty());
    }
}
