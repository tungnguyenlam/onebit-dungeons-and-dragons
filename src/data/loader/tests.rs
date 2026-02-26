#[cfg(test)]
mod tests {
    use crate::data::loader::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

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
        assert!(count >= 7); // Should include tidewatch-coast now
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
