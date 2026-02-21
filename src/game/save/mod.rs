use crate::{
    app::AppState,
    game::{
        character::Character,
        story::{Journal, WorldState},
    },
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const SAVE_FORMAT_VERSION: u32 = 1;
/// Maximum save schema version this binary understands.
/// Files with a higher version are rejected with a clear message.
const SAVE_FORMAT_MAX_VERSION: u32 = 9;

// ---------------------------------------------------------------------------
// SaveGame — the on-disk structure
// ---------------------------------------------------------------------------

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
    #[serde(default)]
    pub state: AppState,
    #[serde(default)]
    pub menu_ui: crate::app::state::MainMenuUiState,
    #[serde(default)]
    pub char_creation_ui: crate::app::state::CharacterCreationUiState,
    #[serde(default)]
    pub journal_ui: crate::app::state::JournalUiState,
    #[serde(default)]
    pub settings_ui: crate::app::state::SettingsUiState,
}

fn default_save_version() -> u32 {
    // Legacy saves omitted `format_version`; serde default gives 0, which
    // we normalise to SAVE_FORMAT_VERSION after parsing.
    0
}

// ---------------------------------------------------------------------------
// Invariant validation
// ---------------------------------------------------------------------------

/// A collection of invariant violations found in a `SaveGame`.
#[derive(Debug, Default)]
pub struct SaveDriftReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl SaveDriftReport {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn error(&mut self, msg: impl Into<String>) {
        self.errors.push(msg.into());
    }

    fn warn(&mut self, msg: impl Into<String>) {
        self.warnings.push(msg.into());
    }
}

/// Check whether a loaded `SaveGame` satisfies all structural invariants.
/// Returns a report; call `report.has_errors()` to decide whether to abort.
pub fn check_save_invariants(save: &SaveGame) -> SaveDriftReport {
    let mut report = SaveDriftReport::default();

    // Version bounds
    if save.format_version == 0 {
        report.warn("save has legacy format_version 0; treated as version 1");
    } else if save.format_version > SAVE_FORMAT_MAX_VERSION {
        report.error(format!(
            "save format_version {} exceeds maximum known version {}",
            save.format_version, SAVE_FORMAT_MAX_VERSION
        ));
    }

    // Player invariants
    let p = &save.player;
    if p.name.is_empty() {
        report.error("player name is empty");
    }
    if p.class_id.is_empty() {
        report.error("player class_id is empty");
    }
    if p.race_id.is_empty() {
        report.error("player race_id is empty");
    }
    if p.max_hp == 0 {
        report.error("player max_hp is 0");
    }
    if p.current_hp > p.max_hp {
        report.error(format!(
            "player current_hp ({}) exceeds max_hp ({})",
            p.current_hp, p.max_hp
        ));
    }
    if p.level == 0 {
        report.error("player level is 0");
    }
    if p.level > 20 {
        report.error(format!("player level {} exceeds 20", p.level));
    }
    for (i, (&slot, &max)) in p
        .spell_slots
        .iter()
        .zip(p.spell_slots_max.iter())
        .enumerate()
    {
        if slot > max {
            report.error(format!(
                "spell_slots[{i}] ({slot}) exceeds spell_slots_max[{i}] ({max})"
            ));
        }
    }

    // World state invariants
    if save.region_slug.is_empty() {
        report.error("region_slug is empty");
    }
    if save.room_id.is_empty() {
        report.error("room_id is empty");
    }

    // Position sanity (pos must fit in a reasonable map)
    if save.player_pos.0 > 512 || save.player_pos.1 > 512 {
        report.warn(format!(
            "player_pos ({}, {}) is unusually large",
            save.player_pos.0, save.player_pos.1
        ));
    }

    // Turn monotonicity hint
    if save.turn == 0 {
        report.warn("turn counter is 0 — save may be from before first tick");
    }

    report
}

// ---------------------------------------------------------------------------
// I/O helpers
// ---------------------------------------------------------------------------

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
        // Legacy saves missing version metadata — normalise to v1.
        save.format_version = SAVE_FORMAT_VERSION;
    }
    Ok(save)
}

/// Load a save file and run invariant checks.
/// Prints warnings; returns `Err` if structural errors are found.
pub fn validate_save_file(path: impl AsRef<Path>) -> Result<SaveDriftReport> {
    let path = path.as_ref();
    let save = load_from_path(path)?;
    let report = check_save_invariants(&save);
    if report.has_errors() {
        bail!(
            "save file '{}' failed invariant checks: {}",
            path.display(),
            report.errors.join("; ")
        );
    }
    Ok(report)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::character::AbilityScores;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn ts() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    }

    fn base_save() -> SaveGame {
        let player = Character::new(
            "Tester".into(),
            "fighter".into(),
            "human".into(),
            AbilityScores::standard_array(),
        );
        SaveGame {
            format_version: SAVE_FORMAT_VERSION,
            turn: 42,
            player,
            world_state: WorldState::new(),
            journal: Journal::default(),
            region_slug: "valley-of-ash".into(),
            room_id: "ash_gate".into(),
            player_pos: (3, 4),
            state: AppState::WorldMap,
        }
    }

    #[test]
    fn save_roundtrip_toml_file() {
        let mut save = base_save();
        save.player.current_hp = 7;
        let path = std::env::temp_dir().join(format!("dnd-save-{}.toml", ts()));
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
        let save = base_save();
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
        assert_eq!(loaded.turn, 42);
    }

    #[test]
    fn repeated_roundtrip_no_drift() {
        let path = std::env::temp_dir().join(format!("dnd-nodrift-{}.toml", ts()));
        let mut save = base_save();
        save.player.current_hp = 15;
        save.world_state.set_flag("test_flag");
        save.world_state.set_faction_rep("town_guard", 3);

        // 5 consecutive save/load cycles must produce identical data.
        for i in 0..5u64 {
            save.turn = i;
            save_to_path(&path, &save).unwrap();
            save = load_from_path(&path).unwrap();
        }
        assert_eq!(save.player.current_hp, 15);
        assert!(save.world_state.flag("test_flag"));
        assert_eq!(save.world_state.faction_rep("town_guard"), 3);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn invariant_check_passes_valid_save() {
        let report = check_save_invariants(&base_save());
        assert!(!report.has_errors(), "errors: {:?}", report.errors);
    }

    #[test]
    fn invariant_check_catches_hp_overflow() {
        let mut save = base_save();
        save.player.current_hp = save.player.max_hp + 10;
        let report = check_save_invariants(&save);
        assert!(report.has_errors());
        assert!(report.errors.iter().any(|e| e.contains("exceeds max_hp")));
    }

    #[test]
    fn invariant_check_catches_empty_region_slug() {
        let mut save = base_save();
        save.region_slug = String::new();
        let report = check_save_invariants(&save);
        assert!(report.has_errors());
        assert!(report.errors.iter().any(|e| e.contains("region_slug")));
    }

    #[test]
    fn invariant_check_warns_on_legacy_version() {
        let mut save = base_save();
        save.format_version = 0;
        let report = check_save_invariants(&save);
        assert!(!report.has_errors());
        assert!(report.warnings.iter().any(|w| w.contains("legacy")));
    }

    #[test]
    fn invariant_check_catches_future_version() {
        let mut save = base_save();
        save.format_version = SAVE_FORMAT_MAX_VERSION + 1;
        let report = check_save_invariants(&save);
        assert!(report.has_errors());
        assert!(report.errors.iter().any(|e| e.contains("exceeds maximum")));
    }

    #[test]
    fn invariant_check_catches_slot_overflow() {
        let mut save = base_save();
        save.player.spell_slots_max[0] = 2;
        save.player.spell_slots[0] = 5;
        let report = check_save_invariants(&save);
        assert!(report.has_errors());
        assert!(report.errors.iter().any(|e| e.contains("spell_slots")));
    }

    #[test]
    fn validate_save_file_roundtrip() {
        let path = std::env::temp_dir().join(format!("dnd-validate-{}.toml", ts()));
        let save = base_save();
        save_to_path(&path, &save).unwrap();
        let report = validate_save_file(&path).unwrap();
        assert!(!report.has_errors());
        let _ = std::fs::remove_file(&path);
    }
}
