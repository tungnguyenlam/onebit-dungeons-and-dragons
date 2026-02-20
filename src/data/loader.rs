/// Asset loading utilities.
///
/// All TOML file I/O is confined to this module. Game modules receive
/// fully-typed structs; they never see raw file paths or TOML strings.
///
/// See [docs/architecture/data-pipeline.md] for the full loading strategy.
use crate::data::types::*;
use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Generic loader
// ---------------------------------------------------------------------------

/// Read and deserialize a single TOML file.
pub fn load<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let path = path.as_ref();
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?;
    toml::from_str(&raw)
        .with_context(|| format!("parsing {}", path.display()))
}

// ---------------------------------------------------------------------------
// Loaded region bundle
// ---------------------------------------------------------------------------

/// All data for a single region, eagerly loaded into memory.
#[derive(Debug, Clone)]
pub struct LoadedRegion {
    pub manifest: RegionManifest,
    /// Rooms keyed by room id.
    pub rooms:    std::collections::HashMap<String, RoomDef>,
    /// NPCs keyed by npc id.
    pub npcs:     std::collections::HashMap<String, NpcDef>,
    /// Dialog trees keyed by npc id.
    pub dialogs:  std::collections::HashMap<String, DialogTree>,
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
    let npc_dir    = region_dir.join("npcs");
    let dialog_dir = region_dir.join("dialog");
    let mut npcs    = std::collections::HashMap::new();
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

    Ok(LoadedRegion { manifest, rooms, npcs, dialogs })
}

// ---------------------------------------------------------------------------
// Global assets (loaded once at startup)
// ---------------------------------------------------------------------------

/// All globally-shared asset data, loaded from `assets/`.
#[derive(Debug, Default)]
pub struct GlobalAssets {
    pub monsters: std::collections::HashMap<String, MonsterDef>,
    pub classes:  std::collections::HashMap<String, ClassDef>,
    pub races:    std::collections::HashMap<String, RaceDef>,
    pub items:    std::collections::HashMap<String, ItemDef>,
    pub spells:   std::collections::HashMap<String, SpellDef>,
    pub quests:   std::collections::HashMap<String, QuestDef>,
    pub lore:     std::collections::HashMap<String, LoreEntry>,
}

/// Load all global assets from `assets/`.
/// Missing directories are treated as empty (not an error).
pub fn load_global_assets(base: impl AsRef<Path>) -> Result<GlobalAssets> {
    let base = base.as_ref();
    let mut ga = GlobalAssets::default();

    ga.monsters = load_dir(base.join("monsters"))?;
    ga.classes  = load_dir(base.join("classes"))?;
    ga.races    = load_dir(base.join("races"))?;
    ga.items    = load_dir(base.join("items"))?;
    ga.spells   = load_dir(base.join("spells"))?;
    ga.quests   = load_dir_nested(base.join("quests"))?;
    ga.lore     = load_dir(base.join("lore"))?;

    Ok(ga)
}

/// Load all quest definitions from `assets/quests/` (supports nested folders).
pub fn load_quests(base: impl AsRef<Path>) -> Result<std::collections::HashMap<String, QuestDef>> {
    load_dir_nested(base.as_ref().join("quests"))
}

/// Load all lore entries from `assets/lore/`.
pub fn load_lore(base: impl AsRef<Path>) -> Result<std::collections::HashMap<String, LoreEntry>> {
    load_dir(base.as_ref().join("lore"))
}

/// Load all `*.toml` files in a directory into a `HashMap<id, T>`.
/// `T` must have an `id: String` field (accessed via a helper trait).
fn load_dir<T: DeserializeOwned + HasId>(
    dir: impl AsRef<Path>,
) -> Result<std::collections::HashMap<String, T>> {
    let dir = dir.as_ref();
    let mut map = std::collections::HashMap::new();
    if !dir.is_dir() {
        return Ok(map);
    }
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("toml") {
            let item: T = load(&path)?;
            map.insert(item.id().to_string(), item);
        }
    }
    Ok(map)
}

/// Like `load_dir` but recurses one level (for `assets/quests/main/` etc.)
fn load_dir_nested<T: DeserializeOwned + HasId>(
    dir: impl AsRef<Path>,
) -> Result<std::collections::HashMap<String, T>> {
    let dir = dir.as_ref();
    let mut map = std::collections::HashMap::new();
    if !dir.is_dir() {
        return Ok(map);
    }
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            map.extend(load_dir::<T>(&path)?);
        } else if path.extension().and_then(|e| e.to_str()) == Some("toml") {
            let item: T = load(&path)?;
            map.insert(item.id().to_string(), item);
        }
    }
    Ok(map)
}

// ---------------------------------------------------------------------------
// HasId helper trait
// ---------------------------------------------------------------------------

pub trait HasId {
    fn id(&self) -> &str;
}

macro_rules! impl_has_id {
    ($t:ty) => {
        impl HasId for $t {
            fn id(&self) -> &str { &self.id }
        }
    };
}

impl_has_id!(MonsterDef);
impl_has_id!(ClassDef);
impl_has_id!(RaceDef);
impl_has_id!(ItemDef);
impl_has_id!(SpellDef);
impl_has_id!(QuestDef);
impl_has_id!(LoreEntry);
