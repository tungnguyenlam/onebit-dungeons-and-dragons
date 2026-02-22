#[cfg(test)]
mod tests {
use super::super::types::*;
use super::super::validate::*;
use super::super::io::*;
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
            format_version: super::types::SAVE_FORMAT_VERSION,
            turn: 42,
            player,
            world_state: WorldState::new(),
            journal: Journal::default(),
            region_slug: "valley-of-ash".into(),
            room_id: "ash_gate".into(),
            player_pos: (3, 4),
            state: AppState::WorldMap,
            menu_ui: Default::default(),
            char_creation_ui: Default::default(),
            journal_ui: Default::default(),
            settings_ui: Default::default(),
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
        assert_eq!(loaded.format_version, super::types::SAVE_FORMAT_VERSION);
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
            loaded.format_version = super::types::SAVE_FORMAT_VERSION;
        }
        assert_eq!(loaded.format_version, super::types::SAVE_FORMAT_VERSION);
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