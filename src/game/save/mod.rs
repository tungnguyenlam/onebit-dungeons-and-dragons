use crate::game::{
    character::Character,
    story::{Journal, WorldState},
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const SAVE_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGame {
    #[serde(default = "default_save_version")]
    pub format_version: u32,
    pub turn: u64,
    pub player: Character,
    pub world_state: WorldState,
    pub journal: Journal,
    pub region_slug: String,
    pub room_id: String,
    pub player_pos: (u32, u32),
}

fn default_save_version() -> u32 {
    SAVE_FORMAT_VERSION
}

pub fn save_to_path(path: impl AsRef<Path>, save: &SaveGame) -> Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating save directory {}", parent.display()))?;
    }
    let data = toml::to_string_pretty(save).context("serializing save file")?;
    std::fs::write(path, data).with_context(|| format!("writing save file {}", path.display()))?;
    Ok(())
}

pub fn load_from_path(path: impl AsRef<Path>) -> Result<SaveGame> {
    let path = path.as_ref();
    let data = std::fs::read_to_string(path)
        .with_context(|| format!("reading save file {}", path.display()))?;
    let mut save: SaveGame = toml::from_str(&data).context("parsing save file TOML")?;
    if save.format_version == 0 {
        // Legacy saves missing version metadata deserialize with 0; treat as v1.
        save.format_version = SAVE_FORMAT_VERSION;
    }
    Ok(save)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::character::AbilityScores;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn save_roundtrip_toml_file() {
        let mut player = Character::new(
            "Tester".into(),
            "fighter".into(),
            "human".into(),
            AbilityScores::standard_array(),
        );
        player.current_hp = 7;

        let save = SaveGame {
            format_version: SAVE_FORMAT_VERSION,
            turn: 42,
            player,
            world_state: WorldState::new(),
            journal: Journal::default(),
            region_slug: "valley-of-ash".into(),
            room_id: "ash_gate".into(),
            player_pos: (3, 4),
        };

        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("dnd-save-{nanos}.toml"));
        save_to_path(&path, &save).unwrap();
        let loaded = load_from_path(&path).unwrap();
        assert_eq!(loaded.turn, 42);
        assert_eq!(loaded.player.name, "Tester");
        assert_eq!(loaded.player_pos, (3, 4));
        assert_eq!(loaded.format_version, SAVE_FORMAT_VERSION);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn load_legacy_save_without_version() {
        let save = SaveGame {
            format_version: SAVE_FORMAT_VERSION,
            turn: 5,
            player: Character::new(
                "Legacy".into(),
                "fighter".into(),
                "human".into(),
                AbilityScores::standard_array(),
            ),
            world_state: WorldState::new(),
            journal: Journal::default(),
            region_slug: "valley-of-ash".into(),
            room_id: "ash_gate".into(),
            player_pos: (1, 2),
        };
        let raw = toml::to_string(&save).unwrap();
        let raw_legacy = raw
            .lines()
            .filter(|line| !line.starts_with("format_version"))
            .collect::<Vec<_>>()
            .join("\n");
        let mut loaded: SaveGame = toml::from_str(&raw_legacy).unwrap();
        if loaded.format_version == 0 {
            loaded.format_version = SAVE_FORMAT_VERSION;
        }
        assert_eq!(loaded.format_version, SAVE_FORMAT_VERSION);
        assert_eq!(loaded.turn, 5);
    }
}
