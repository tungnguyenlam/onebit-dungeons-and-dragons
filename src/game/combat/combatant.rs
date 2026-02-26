use crate::game::{
    character::conditions::Condition,
    combat::{
        action::ActionSlots,
        initiative::{roll_initiative_with_seed, InitiativeCombatant},
    },
    dice::DiceExpr,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EnemyAiRole {
    #[default]
    Melee,
    Ranged,
    Spellcaster,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatantState {
    pub id: String,
    pub name: String,
    pub is_player: bool,
    pub max_hp: i32,
    pub current_hp: i32,
    pub armor_class: i32,
    pub speed: u32,
    pub initiative_mod: i32,
    pub attack_bonus: i32,
    pub damage_dice: DiceExpr,
    pub enemy_role: EnemyAiRole,
    pub ranged_attack_bonus: Option<i32>,
    pub ranged_damage_dice: Option<DiceExpr>,
    pub spell_attack_bonus: Option<i32>,
    pub spell_damage_dice: Option<DiceExpr>,
    pub spell_on_hit_condition: Option<Condition>,
    pub ranged_on_hit_condition: Option<Condition>,
    pub on_hit_condition: Option<Condition>,
    pub damage_type: String,
    pub ranged_damage_type: Option<String>,
    pub spell_damage_type: Option<String>,
    pub resistances: HashSet<String>,
    pub vulnerabilities: HashSet<String>,
    pub immunities: HashSet<String>,
    pub condition_immunities: HashSet<String>,
    pub conditions: HashSet<Condition>,
    pub condition_durations: HashMap<Condition, u8>,
    pub action_slots: ActionSlots,
}

impl CombatantState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        is_player: bool,
        max_hp: i32,
        armor_class: i32,
        speed: u32,
        initiative_mod: i32,
        attack_bonus: i32,
        damage_dice: DiceExpr,
    ) -> Self {
        let max_hp = max_hp.max(1);
        Self {
            id: id.into(),
            name: name.into(),
            is_player,
            max_hp,
            current_hp: max_hp,
            armor_class,
            speed,
            initiative_mod,
            attack_bonus,
            damage_dice,
            enemy_role: EnemyAiRole::Melee,
            ranged_attack_bonus: None,
            ranged_damage_dice: None,
            spell_attack_bonus: None,
            spell_damage_dice: None,
            spell_on_hit_condition: None,
            ranged_on_hit_condition: None,
            on_hit_condition: None,
            damage_type: "bludgeoning".into(),
            ranged_damage_type: None,
            spell_damage_type: None,
            resistances: HashSet::new(),
            vulnerabilities: HashSet::new(),
            immunities: HashSet::new(),
            condition_immunities: HashSet::new(),
            conditions: HashSet::new(),
            condition_durations: HashMap::new(),
            action_slots: ActionSlots::new(speed),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.current_hp > 0
    }

    pub fn take_damage(&mut self, amount: u32) -> i32 {
        self.current_hp = (self.current_hp - amount as i32).max(0);
        if self.current_hp == 0 {
            self.conditions.insert(Condition::Unconscious);
        }
        self.current_hp
    }

    pub fn start_turn(&mut self) {
        let actual_speed = if self.conditions.contains(&Condition::Hasted) {
            self.speed * 2
        } else {
            self.speed
        };
        self.action_slots.reset_turn(actual_speed);
        if self.conditions.contains(&Condition::Hasted) {
            self.action_slots.extra_attacks += 1;
        }
    }

    pub fn can_take_actions(&self) -> bool {
        self.is_alive() && !self.conditions.iter().any(Condition::is_incapacitating)
    }

    pub fn apply_condition(&mut self, condition: Condition, duration_rounds: Option<u8>) {
        if self
            .condition_immunities
            .contains(&condition.name().to_lowercase())
        {
            return;
        }
        self.conditions.insert(condition.clone());
        if let Some(rounds) = duration_rounds {
            if rounds > 0 {
                self.condition_durations.insert(condition, rounds);
            }
        }
    }

    pub fn condition_duration(&self, condition: &Condition) -> Option<u8> {
        self.condition_durations.get(condition).copied()
    }

    /// Decrement timed conditions and remove those that expire.
    pub fn tick_condition_durations(&mut self) -> Vec<Condition> {
        let mut expired = Vec::new();
        let mut pending = Vec::new();

        for (cond, rounds) in &self.condition_durations {
            if *rounds <= 1 {
                expired.push(cond.clone());
            } else {
                pending.push((cond.clone(), rounds - 1));
            }
        }

        for cond in &expired {
            self.condition_durations.remove(cond);
            self.conditions.remove(cond);
        }
        for (cond, rounds) in pending {
            self.condition_durations.insert(cond, rounds);
        }
        expired
    }
}
