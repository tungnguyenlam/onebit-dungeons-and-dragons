/// Typed serde structs for all TOML asset files.
///
/// These are the **raw deserialization** shapes. Game modules consume them
/// after they are loaded by `src/data/loader.rs`. No game logic lives here.
use crate::game::dice::DiceExpr;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Shared primitives
// ---------------------------------------------------------------------------

/// A 2D tile position [col, row] (0-indexed).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TilePos(pub u32, pub u32);

// ---------------------------------------------------------------------------
// Region & Room
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegionManifest {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub entry_room: String,
    #[serde(default)]
    pub ambient: String,
    #[serde(default)]
    pub region_type: String,
    #[serde(default)]
    pub weather: String,
    #[serde(default)]
    pub rooms: Vec<RoomRef>,
    #[serde(default)]
    pub connections: Vec<RegionConnection>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomRef {
    pub id: String,
    pub file: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegionConnection {
    pub from_room: String,
    pub to_region: String,
    pub to_room: String,
    pub label: String,
    #[serde(default)]
    pub condition: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub grid: String,
    /// If true, this room is an intentional dead-end and the validator will not
    /// flag it for lacking an outbound travel trigger.
    #[serde(default)]
    pub terminal: bool,
    #[serde(default)]
    pub npcs: Vec<RoomNpc>,
    #[serde(default)]
    pub items: Vec<RoomItem>,
    #[serde(default)]
    pub triggers: Vec<TriggerDef>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomNpc {
    pub id: String,
    pub position: [u32; 2],
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomItem {
    pub id: String,
    pub position: [u32; 2],
    #[serde(default = "one")]
    pub quantity: u32,
    #[serde(default)]
    pub condition: String,
}

fn one() -> u32 {
    1
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TriggerDef {
    pub position: [u32; 2],
    #[serde(rename = "type")]
    pub kind: TriggerKind,
    pub target_id: String,
    #[serde(default)]
    pub condition: String,
    #[serde(default)]
    pub once: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerKind {
    Dialog,
    Encounter,
    Lore,
    QuestStage,
    Travel,
}

// ---------------------------------------------------------------------------
// NPC (per-region)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NpcDef {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub monster_ref: String,
    #[serde(default)]
    pub faction: String,
    #[serde(default)]
    pub dialog_ref: String,
    #[serde(default)]
    pub overrides: NpcStatOverrides,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NpcStatOverrides {
    pub hp: Option<u32>,
    pub ac: Option<u32>,
}

// ---------------------------------------------------------------------------
// Dialog Tree
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DialogTree {
    pub npc_id: String,
    #[serde(default)]
    pub nodes: Vec<DialogNode>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DialogNode {
    pub id: String,
    pub text: String,
    #[serde(default)]
    pub effect: Vec<DialogEffect>,
    #[serde(default)]
    pub choices: Vec<DialogChoice>,
    // Skill-check node fields (present when text == "__SKILL_CHECK__")
    pub skill: Option<String>,
    pub dc: Option<u32>,
    pub on_pass: Option<String>,
    pub on_fail: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DialogChoice {
    pub text: String,
    #[serde(default)]
    pub condition: String,
    #[serde(default)]
    pub effect: Vec<DialogEffect>,
    pub next: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DialogEffect {
    SetFlag { set_flag: String },
    ClearFlag { clear_flag: String },
    DeltaCounter { delta_counter: CounterDelta },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CounterDelta {
    pub key: String,
    pub delta: i32,
}

// ---------------------------------------------------------------------------
// Monster stat block
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonsterDef {
    pub id: String,
    pub name: String,
    pub cr: f32,
    pub size: String,
    pub monster_type: String,
    pub alignment: String,
    pub hp: DiceExpr,
    pub ac: u32,
    pub speed: u32,
    pub str_score: u8,
    pub dex_score: u8,
    pub con_score: u8,
    pub int_score: u8,
    pub wis_score: u8,
    pub cha_score: u8,
    pub xp: u32,
    #[serde(default)]
    pub actions: Vec<MonsterAction>,
    #[serde(default)]
    pub traits: Vec<MonsterTrait>,
    #[serde(default)]
    pub resistances: Vec<String>,
    #[serde(default)]
    pub vulnerabilities: Vec<String>,
    #[serde(default)]
    pub immunities: Vec<String>,
    #[serde(default)]
    pub condition_immunities: Vec<String>,
    #[serde(default)]
    pub loot: Vec<MonsterLoot>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonsterLoot {
    pub item_id: String,
    pub chance: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonsterAction {
    pub name: String,
    pub description: String,
    pub attack_bonus: Option<i32>,
    pub damage: Option<DiceExpr>,
    pub damage_type: Option<String>,
    #[serde(default)]
    pub on_hit_condition: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonsterTrait {
    pub name: String,
    pub description: String,
}

// ---------------------------------------------------------------------------
// Character class
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClassDef {
    pub id: String,
    pub name: String,
    pub hit_die: u8,
    pub primary_ability: String,
    #[serde(default)]
    pub saving_throw_proficiencies: Vec<String>,
    #[serde(default)]
    pub armor_proficiencies: Vec<String>,
    #[serde(default)]
    pub weapon_proficiencies: Vec<String>,
    #[serde(default)]
    pub features: HashMap<u8, Vec<ClassFeatureDef>>,
    #[serde(default)]
    pub spell_slots: HashMap<u8, Vec<u8>>, // level → [slots per spell level]
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClassFeatureDef {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub mechanical_effect: String,
}

// ---------------------------------------------------------------------------
// Race
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RaceDef {
    pub id: String,
    pub name: String,
    pub speed: u32,
    pub size: String,
    #[serde(default)]
    pub ability_score_increases: HashMap<String, i8>,
    #[serde(default)]
    pub traits: Vec<RaceTraitDef>,
    #[serde(default)]
    pub languages: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RaceTraitDef {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub mechanical_effect: String,
}

// ---------------------------------------------------------------------------
// Item
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemDef {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: ItemType,
    pub weight: f32,
    pub value_gp: u32,
    pub description: String,
    pub weapon: Option<WeaponDef>,
    pub armor: Option<ArmorDef>,
    #[serde(default)]
    pub bonuses: ItemBonuses,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ItemBonuses {
    #[serde(default)]
    pub attack_bonus: i32,
    #[serde(default)]
    pub damage_bonus: i32,
    #[serde(default)]
    pub armor_class_bonus: i32,
    #[serde(default)]
    pub spell_attack_bonus: i32,
    #[serde(default)]
    pub spell_damage_bonus: i32,
    #[serde(default)]
    pub max_hp_bonus: i32,
    #[serde(default)]
    pub resistances: Vec<String>,
    #[serde(default)]
    pub immunities: Vec<String>,
    #[serde(default)]
    pub condition_immunities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Weapon,
    Armor,
    Consumable,
    Misc,
    Quest,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeaponDef {
    pub damage: DiceExpr,
    pub damage_type: String,
    #[serde(default)]
    pub properties: Vec<String>,
    pub versatile_damage: Option<DiceExpr>,
    pub range: Option<WeaponRange>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeaponRange {
    pub normal: u32,
    pub long: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArmorDef {
    pub base_ac: u32,
    #[serde(rename = "type")]
    pub armor_type: ArmorType,
    #[serde(default)]
    pub stealth_disadvantage: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
    Shield,
}

// ---------------------------------------------------------------------------
// Spell
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpellDef {
    pub id: String,
    pub name: String,
    pub level: u8, // 0 = cantrip
    pub school: String,
    pub casting_time: String,
    pub range: String,
    pub components: Vec<String>,
    pub duration: String,
    pub description: String,
    pub damage: Option<DiceExpr>,
    pub damage_type: Option<String>,
    pub save: Option<String>,
    pub heal: Option<DiceExpr>,
    #[serde(default)]
    pub classes: Vec<String>,
}

// ---------------------------------------------------------------------------
// Quest
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuestDef {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: QuestKind,
    pub stages: Vec<QuestStageDef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum QuestKind {
    Main,
    Side,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuestStageDef {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub condition: String,
    #[serde(default)]
    pub on_enter: Vec<DialogEffect>,
    pub next: Vec<QuestTransition>,
    pub journal_entry: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuestTransition {
    pub condition: String,
    pub stage: String,
}

// ---------------------------------------------------------------------------
// Lore
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoreEntry {
    pub id: String,
    pub title: String,
    pub text: String,
    #[serde(default)]
    pub tags: Vec<String>,
}
