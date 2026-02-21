/// Asset loading utilities.
///
/// All TOML file I/O is confined to this module. Game modules receive
/// fully-typed structs; they never see raw file paths or TOML strings.
///
/// See [docs/architecture/data-pipeline.md] for the full loading strategy.
use crate::data::types::*;
use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use std::path::Path;

// ---------------------------------------------------------------------------
// Generic loader
// ---------------------------------------------------------------------------

/// Read and deserialize a single TOML file.
pub fn load<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let path = path.as_ref();
    let raw =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    toml::from_str(&raw).with_context(|| format!("parsing {}", path.display()))
}

// ---------------------------------------------------------------------------
// Loaded region bundle
// ---------------------------------------------------------------------------

/// All data for a single region, eagerly loaded into memory.
#[derive(Debug, Clone)]
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

// ---------------------------------------------------------------------------
// Global assets (loaded once at startup)
// ---------------------------------------------------------------------------

/// All globally-shared asset data, loaded from `assets/`.
#[derive(Debug, Default)]
pub struct GlobalAssets {
    pub monsters: std::collections::HashMap<String, MonsterDef>,
    pub classes: std::collections::HashMap<String, ClassDef>,
    pub races: std::collections::HashMap<String, RaceDef>,
    pub items: std::collections::HashMap<String, ItemDef>,
    pub spells: std::collections::HashMap<String, SpellDef>,
    pub quests: std::collections::HashMap<String, QuestDef>,
    pub lore: std::collections::HashMap<String, LoreEntry>,
}

/// Load all global assets from `assets/`.
/// Missing directories are treated as empty (not an error).
pub fn load_global_assets(base: impl AsRef<Path>) -> Result<GlobalAssets> {
    let base = base.as_ref();
    Ok(GlobalAssets {
        monsters: load_dir(base.join("monsters"))?,
        classes: load_dir(base.join("classes"))?,
        races: load_dir(base.join("races"))?,
        items: load_dir(base.join("items"))?,
        spells: load_dir(base.join("spells"))?,
        quests: load_dir_nested(base.join("quests"))?,
        lore: load_dir(base.join("lore"))?,
    })
}

/// Load all quest definitions from `assets/quests/` (supports nested folders).
pub fn load_quests(base: impl AsRef<Path>) -> Result<std::collections::HashMap<String, QuestDef>> {
    load_dir_nested(base.as_ref().join("quests"))
}

/// Load all lore entries from `assets/lore/`.
pub fn load_lore(base: impl AsRef<Path>) -> Result<std::collections::HashMap<String, LoreEntry>> {
    load_dir(base.as_ref().join("lore"))
}

/// Load all monster definitions from `assets/monsters/`.
pub fn load_monsters(
    base: impl AsRef<Path>,
) -> Result<std::collections::HashMap<String, MonsterDef>> {
    load_dir(base.as_ref().join("monsters"))
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
            fn id(&self) -> &str {
                &self.id
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn unique_tmp_dir(tag: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        std::env::temp_dir().join(format!("dnd-loader-{tag}-{nanos}"))
    }

    #[test]
    fn load_monsters_reads_toml_files() {
        let dir = unique_tmp_dir("monsters");
        let monsters_dir = dir.join("monsters");
        fs::create_dir_all(&monsters_dir).unwrap();
        fs::write(
            monsters_dir.join("goblin.toml"),
            r#"
id = "goblin"
name = "Goblin"
cr = 0.25
size = "small"
monster_type = "humanoid"
alignment = "neutral_evil"
hp = "2d6"
ac = 13
speed = 30
str_score = 8
dex_score = 14
con_score = 10
int_score = 10
wis_score = 8
cha_score = 8
xp = 50

[[actions]]
name = "Scimitar"
description = "Melee Weapon Attack"
attack_bonus = 4
damage = "1d6+2"
damage_type = "slashing"
"#,
        )
        .unwrap();

        let loaded = load_monsters(&dir).unwrap();
        assert_eq!(loaded.len(), 1);
        assert!(loaded.contains_key("goblin"));

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn load_global_assets_includes_monsters() {
        let dir = unique_tmp_dir("global-assets");
        let monsters_dir = dir.join("monsters");
        fs::create_dir_all(&monsters_dir).unwrap();
        fs::write(
            monsters_dir.join("bandit.toml"),
            r#"
id = "bandit"
name = "Bandit"
cr = 0.125
size = "medium"
monster_type = "humanoid"
alignment = "chaotic_neutral"
hp = "2d8"
ac = 12
speed = 30
str_score = 11
dex_score = 12
con_score = 12
int_score = 10
wis_score = 10
cha_score = 10
xp = 25
"#,
        )
        .unwrap();

        let loaded = load_global_assets(&dir).unwrap();
        assert_eq!(loaded.monsters.len(), 1);
        assert!(loaded.monsters.contains_key("bandit"));

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn load_authored_valley_of_ash_region() {
        let loaded = load_region("assets", "valley-of-ash").unwrap();
        assert_eq!(loaded.manifest.slug, "valley-of-ash");
        assert!(loaded.rooms.contains_key("ash_gate"));
        assert!(loaded.rooms.contains_key("ember_square"));
        assert!(loaded.npcs.contains_key("captain_kael"));
        assert!(loaded.dialogs.contains_key("captain_kael"));
    }

    #[test]
    fn load_all_authored_regions() {
        let regions_dir = std::path::Path::new("assets/regions");
        let entries = std::fs::read_dir(regions_dir).unwrap();
        let mut count = 0usize;
        for entry in entries {
            let path = entry.unwrap().path();
            if !path.is_dir() {
                continue;
            }
            let slug = path.file_name().unwrap().to_string_lossy().to_string();
            let loaded = load_region("assets", &slug).unwrap();
            assert_eq!(loaded.manifest.slug, slug);
            assert!(!loaded.rooms.is_empty());
            count += 1;
        }
        assert!(count >= 3);
    }

    #[test]
    fn load_all_authored_quests() {
        let quests = load_quests("assets").unwrap();
        assert!(quests.contains_key("valley_contract"));
        assert!(quests.contains_key("volcanic_curse"));
        assert!(quests.contains_key("dwarven_relic"));
        assert!(quests.contains_key("gnome_debt"));
    }

    #[test]
    fn profile_asset_load_smoke() {
        let start = std::time::Instant::now();
        let global = load_global_assets("assets").unwrap();
        let _region = load_region("assets", "valley-of-ash").unwrap();
        let elapsed = start.elapsed();
        eprintln!(
            "asset-load profile: {:?} (monsters={}, items={}, regions rooms={})",
            elapsed,
            global.monsters.len(),
            global.items.len(),
            2
        );
        assert!(elapsed < std::time::Duration::from_secs(5));
    }
}
