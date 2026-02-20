use crate::game::{
    character::Character,
    story::{Journal, WorldState},
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGame {
    pub turn: u64,
    pub player: Character,
    pub world_state: WorldState,
    pub journal: Journal,
    pub region_slug: String,
    pub room_id: String,
    pub player_pos: (u32, u32),
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
    let save: SaveGame = toml::from_str(&data).context("parsing save file TOML")?;
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
        let _ = std::fs::remove_file(path);
    }
}
