use crate::data::types::{
    ClassDef, DialogTree, ItemDef, LoreEntry, MonsterDef, NpcDef, QuestDef, RaceDef, SpellDef,
};
use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
/// Read and deserialize a single TOML file.
pub fn load<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let path = path.as_ref();
    let raw =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    toml::from_str(&raw).with_context(|| format!("parsing {}", path.display()))
}