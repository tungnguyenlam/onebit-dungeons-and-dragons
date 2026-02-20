/// Level-up and proficiency bonus helpers.
///
/// See [docs/gameplay/character.md] for full 5e SRD leveling rules.
use crate::game::dice::DiceExpr;

/// Standard 5e SRD proficiency bonus by character level.
///
/// | Level | Bonus |
/// |-------|-------|
/// | 1–4   | +2    |
/// | 5–8   | +3    |
/// | 9–12  | +4    |
/// | 13–16 | +5    |
/// | 17–20 | +6    |
pub fn proficiency_bonus(level: u8) -> i32 {
    match level {
        1..=4   => 2,
        5..=8   => 3,
        9..=12  => 4,
        13..=16 => 5,
        _       => 6,
    }
}

/// XP required to **reach** the given level (from the 5e SRD table).
pub fn xp_threshold(level: u8) -> u32 {
    match level {
        1  => 0,
        2  => 300,
        3  => 900,
        4  => 2_700,
        5  => 6_500,
        6  => 14_000,
        7  => 23_000,
        8  => 34_000,
        9  => 48_000,
        10 => 64_000,
        11 => 85_000,
        12 => 100_000,
        13 => 120_000,
        14 => 140_000,
        15 => 165_000,
        16 => 195_000,
        17 => 225_000,
        18 => 265_000,
        19 => 305_000,
        20 => 355_000,
        _  => u32::MAX,
    }
}

/// Compute the new level for a character with `xp` total XP.
pub fn level_for_xp(xp: u32) -> u8 {
    (1u8..=20)
        .rev()
        .find(|&lvl| xp >= xp_threshold(lvl))
        .unwrap_or(1)
}

/// Roll or take average HP for a hit die at leveling up.
///
/// - Level 1: always max hit die value.
/// - Other levels: roll or take average (floor(sides/2) + 1) + CON modifier.
pub fn hp_on_level_up(hit_die_sides: u8, con_modifier: i32, level: u8, rolled: bool) -> i32 {
    let base = if level == 1 || !rolled {
        if level == 1 {
            hit_die_sides as i32
        } else {
            // Average: floor(sides/2) + 1
            (hit_die_sides as i32 / 2) + 1
        }
    } else {
        DiceExpr::new(1, hit_die_sides as u32, 0).roll()
    };
    (base + con_modifier).max(1)
}

/// Whether the given level awards an Ability Score Improvement.
pub fn is_asi_level(level: u8) -> bool {
    matches!(level, 4 | 8 | 12 | 16 | 19)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proficiency_bonus_values() {
        assert_eq!(proficiency_bonus(1), 2);
        assert_eq!(proficiency_bonus(4), 2);
        assert_eq!(proficiency_bonus(5), 3);
        assert_eq!(proficiency_bonus(20), 6);
    }

    #[test]
    fn level_for_xp_boundaries() {
        assert_eq!(level_for_xp(0), 1);
        assert_eq!(level_for_xp(300), 2);
        assert_eq!(level_for_xp(299), 1);
        assert_eq!(level_for_xp(6_500), 5);
    }

    #[test]
    fn hp_level1_always_max() {
        assert_eq!(hp_on_level_up(8, 0, 1, false), 8);
        assert_eq!(hp_on_level_up(8, 2, 1, false), 10);
    }

    #[test]
    fn asi_levels() {
        assert!(is_asi_level(4));
        assert!(!is_asi_level(3));
        assert!(is_asi_level(19));
    }
}
