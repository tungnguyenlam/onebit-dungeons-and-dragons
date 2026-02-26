use crate::game::{story::WorldState, world::region::Region};

#[derive(Debug, Clone)]
pub struct ExitView {
    pub to_region: String,
    pub to_room: String,
    pub label: String,
    pub available: bool,
}

#[derive(Debug, Clone)]
pub struct RegionOverview {
    pub region_name: String,
    pub region_slug: String,
    pub current_room: String,
    pub room_ids: Vec<String>,
    pub current_landmark: String,
    pub connected_rooms: Vec<String>,
    pub exits: Vec<ExitView>,
}

pub fn build_region_overview(region: &Region, current_room: &str, world: &WorldState) -> RegionOverview {
    let mut room_ids = region.rooms.keys().cloned().collect::<Vec<_>>();
    room_ids.sort();

    let exits = region
        .connections
        .iter()
        .filter(|c| c.from_room == current_room)
        .map(|c| ExitView {
            to_region: c.to_region.clone(),
            to_room: c.to_room.clone(),
            label: c.label.clone(),
            available: c.condition.is_empty() || world.evaluate(&c.condition),
        })
        .collect::<Vec<_>>();

    let current_landmark = region
        .room(current_room)
        .map(|r| r.landmark.clone())
        .unwrap_or_default();

    let mut connected_rooms = region
        .room(current_room)
        .map(|r| {
            [&r.exits.north, &r.exits.east, &r.exits.south, &r.exits.west]
                .iter()
                .filter(|id| !id.is_empty() && region.rooms.contains_key(id.as_str()))
                .map(|id| (*id).clone())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    connected_rooms.sort();
    connected_rooms.dedup();

    RegionOverview {
        region_name: region.name.clone(),
        region_slug: region.slug.clone(),
        current_room: current_room.to_string(),
        room_ids,
        current_landmark,
        connected_rooms,
        exits,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::{RegionConnection, RegionManifest, RoomDef, RoomRef};
    use crate::game::world::region::Region;
    use std::collections::HashMap;

    fn mk_region() -> Region {
        let manifest = RegionManifest {
            slug: "test".into(),
            name: "Test".into(),
            description: "d".into(),
            entry_room: "a".into(),
            ambient: "none".into(),
            region_type: "test".into(),
            weather: "none".into(),
            rooms: vec![
                RoomRef { id: "a".into(), file: "rooms/a.toml".into() },
                RoomRef { id: "b".into(), file: "rooms/b.toml".into() },
            ],
            connections: vec![
                RegionConnection {
                    from_room: "a".into(),
                    to_region: "next".into(),
                    to_room: "gate".into(),
                    label: "Go next".into(),
                    condition: "flag:ok".into(),
                },
            ],
        };
        let mut rooms = HashMap::new();
        rooms.insert(
            "a".into(),
            RoomDef {
                id: "a".into(),
                name: "A".into(),
                description: "A".into(),
                landmark: "A-Landmark".into(),
                grid: "###\n#.#\n###\n".into(),
                terminal: false,
                npcs: vec![],
                items: vec![],
                triggers: vec![crate::data::types::TriggerDef {
                    position: [1, 1],
                    kind: crate::data::types::TriggerKind::Travel,
                    target_id: "b".into(),
                    condition: String::new(),
                    once: false,
                }],
                exits: crate::data::types::RoomExits {
                    east: "b".into(),
                    ..Default::default()
                },
            },
        );
        rooms.insert(
            "b".into(),
            RoomDef {
                id: "b".into(),
                name: "B".into(),
                description: "B".into(),
                landmark: "B-Landmark".into(),
                grid: "###\n#.#\n###\n".into(),
                terminal: false,
                npcs: vec![],
                items: vec![],
                triggers: vec![],
                exits: crate::data::types::RoomExits::default(),
            },
        );
        let loaded = crate::data::loader::LoadedRegion {
            manifest,
            rooms,
            npcs: HashMap::new(),
            dialogs: HashMap::new(),
        };
        Region::from_loaded(&loaded)
    }

    #[test]
    fn overview_respects_exit_conditions() {
        let region = mk_region();
        let mut ws = WorldState::new();
        let out = build_region_overview(&region, "a", &ws);
        assert_eq!(out.room_ids.len(), 2);
        assert_eq!(out.current_landmark, "A-Landmark");
        assert_eq!(out.connected_rooms, vec!["b".to_string()]);
        assert_eq!(out.exits.len(), 1);
        assert!(!out.exits[0].available);

        ws.set_flag("ok");
        let out2 = build_region_overview(&region, "a", &ws);
        assert!(out2.exits[0].available);
    }
}
