use crate::data::types::{
    ClassDef, DialogTree, ItemDef, LoreEntry, MonsterDef, NpcDef, QuestDef, RaceDef, SpellDef, RegionManifest, RoomDef
};
use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use super::dir::*;
use super::core::*;
pub struct LoadedRegion {
    pub manifest: RegionManifest,
    /// Rooms keyed by room id.
    pub rooms: std::collections::HashMap<String, RoomDef>,
    /// NPCs keyed by npc id.
    pub npcs: std::collections::HashMap<String, NpcDef>,
    /// Dialog trees keyed by npc id.
    pub dialogs: std::collections::HashMap<String, DialogTree>,
}

/// Load a full region from `assets/regions/<slug>/`.
pub fn load_region(base: impl AsRef<Path>, slug: &str) -> Result<LoadedRegion> {
    let region_dir = base.as_ref().join("regions").join(slug);

    // 1. Parse region manifest
    let manifest: RegionManifest = load(region_dir.join("region.toml"))?;

    // 2. Load each room
    let mut rooms = std::collections::HashMap::new();
    for room_ref in &manifest.rooms {
        let room_path = region_dir.join(&room_ref.file);
        let room: RoomDef = load(&room_path)
            .with_context(|| format!("room '{}' in region '{slug}'", room_ref.id))?;
        rooms.insert(room_ref.id.clone(), room);
    }

    // 3. Load NPCs and their dialog trees
    let npc_dir = region_dir.join("npcs");
    let dialog_dir = region_dir.join("dialog");
    let mut npcs = std::collections::HashMap::new();
    let mut dialogs = std::collections::HashMap::new();

    if npc_dir.is_dir() {
        for entry in std::fs::read_dir(&npc_dir)
            .with_context(|| format!("reading npcs/ for region '{slug}'"))?
        {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                let npc: NpcDef = load(&path)?;
                let id = npc.id.clone();

                // Load dialog if available
                let dialog_path = dialog_dir.join(format!("{id}.toml"));
                if dialog_path.exists() {
                    let dialog: DialogTree = load(&dialog_path)?;
                    dialogs.insert(id.clone(), dialog);
                }
                npcs.insert(id, npc);
            }
        }
    }

    Ok(LoadedRegion {
        manifest,
        rooms,
        npcs,
        dialogs,
    })
}