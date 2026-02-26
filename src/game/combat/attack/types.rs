use crate::game::{
    character::conditions::Condition, combat::CombatantState, dice::DiceExpr,
    story::world_state::WorldState,
};
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HitType {
    Miss,
    Hit,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollMode {
    Normal,
    Advantage,
    Disadvantage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttackOutcome {
    pub attacker_id: String,
    pub target_id: String,
    pub d20: i32,
    pub total: i32,
    pub roll_mode: RollMode,
    pub hit_type: HitType,
    pub damage: u32,
    pub damage_type: String,
    pub inflicted_condition: Option<Condition>,
}

#[derive(Debug, Clone)]
pub struct AttackProfile<'a> {
    pub id: &'a str,
    pub attack_bonus: i32,
    pub is_ranged: bool,
    pub damage_dice: &'a DiceExpr,
    pub damage_type: &'a str,
    pub conditions: &'a HashSet<Condition>,
    pub on_hit_condition: Option<Condition>,
}

#[derive(Debug, Clone)]
pub struct DefenseProfile<'a> {
    pub id: &'a str,
    pub armor_class: i32,
    pub conditions: &'a HashSet<Condition>,
    pub resistances: &'a HashSet<String>,
    pub vulnerabilities: &'a HashSet<String>,
    pub immunities: &'a HashSet<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SaveOutcome {
    Success { d20: i32, total: i32 },
    Failure { d20: i32, total: i32 },
}
