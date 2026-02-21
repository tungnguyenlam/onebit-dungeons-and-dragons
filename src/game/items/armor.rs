/// Armor class calculation.
///
/// See [docs/gameplay/items.md] for the full rules.
use crate::data::types::ArmorType;

/// Calculate total AC given equipped armor and DEX modifier.
///
/// - No armor: 10 + DEX mod
/// - Light: base_ac + DEX mod
/// - Medium: base_ac + min(DEX mod, 2)
/// - Heavy: base_ac (no DEX)
/// - Shield: +2 (stacks, handled separately)
pub fn armor_class(
    armor_base_ac: Option<(u32, &ArmorType)>,
    shield_equipped: bool,
    dex_modifier: i8,
) -> i32 {
    let base = match armor_base_ac {
        None => 10 + dex_modifier as i32,
        Some((base, ArmorType::Light)) => base as i32 + dex_modifier as i32,
        Some((base, ArmorType::Medium)) => base as i32 + (dex_modifier as i32).min(2),
        Some((base, ArmorType::Heavy)) => base as i32,
        Some((_, ArmorType::Shield)) => 10 + dex_modifier as i32, // shouldn't happen
    };
    base + if shield_equipped { 2 } else { 0 }
}
