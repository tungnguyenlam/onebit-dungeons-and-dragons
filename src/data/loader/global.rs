use crate::data::types::{
    ClassDef, DialogTree, ItemDef, LoreEntry, MonsterDef, NpcDef, QuestDef, RaceDef, SpellDef,
};
use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use super::dir::*;
use super::core::*;
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