use crate::data::types::{
    ClassDef, DialogTree, ItemDef, LoreEntry, MonsterDef, NpcDef, QuestDef, RaceDef, SpellDef,
};
use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use super::core::*;
/// Load all `*.toml` files in a directory into a `HashMap<id, T>`.
/// `T` must have an `id: String` field (accessed via a helper trait).
pub fn load_dir<T: DeserializeOwned + HasId>(
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
pub fn load_dir_nested<T: DeserializeOwned + HasId>(
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
