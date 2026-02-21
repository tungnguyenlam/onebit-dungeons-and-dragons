/// Runtime combat state and turn progression.
use crate::game::{
    character::conditions::Condition,
    combat::{
        action::ActionSlots,
        initiative::{roll_initiative_with_seed, InitiativeCombatant},
    },
    dice::DiceExpr,
};
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatState {
    pub combatants: HashMap<String, CombatantState>,
    pub turn_queue: Vec<String>,
    pub active_turn: usize,
    pub round: u32,
}

impl CombatState {
    /// Build combat state and deterministic turn queue from participants.
    pub fn new_with_seed(combatants: Vec<CombatantState>, seed: u64) -> Self {
        let initiative_input: Vec<InitiativeCombatant> = combatants
            .iter()
            .enumerate()
            .map(|(index, c)| InitiativeCombatant {
                entity_id: c.id.clone(),
                dex_modifier: c.initiative_mod,
                is_player: c.is_player,
                index,
            })
            .collect();
        let order = roll_initiative_with_seed(&initiative_input, seed);

        let mut by_id = HashMap::with_capacity(combatants.len());
        for c in combatants {
            by_id.insert(c.id.clone(), c);
        }

        let mut state = Self {
            combatants: by_id,
            turn_queue: order.queue,
            active_turn: 0,
            round: 1,
        };
        state.reset_current_turn_slots();
        state
    }

    pub fn current_combatant_id(&self) -> Option<&str> {
        self.turn_queue.get(self.active_turn).map(|s| s.as_str())
    }

    pub fn current_combatant(&self) -> Option<&CombatantState> {
        self.current_combatant_id()
            .and_then(|id| self.combatants.get(id))
    }

    pub fn current_combatant_mut(&mut self) -> Option<&mut CombatantState> {
        let id = self.turn_queue.get(self.active_turn)?.clone();
        self.combatants.get_mut(&id)
    }

    /// Advance to the next living combatant and return their id.
    pub fn next_turn(&mut self) -> Option<&str> {
        if self.turn_queue.is_empty() {
            return None;
        }

        let len = self.turn_queue.len();
        for _ in 0..len {
            self.active_turn = (self.active_turn + 1) % len;
            if self.active_turn == 0 {
                self.round += 1;
            }
            let id = &self.turn_queue[self.active_turn];
            if self
                .combatants
                .get(id)
                .is_some_and(CombatantState::is_alive)
            {
                self.reset_current_turn_slots();
                return self.current_combatant_id();
            }
        }

        None
    }

    /// Advance one full turn transition:
    /// - decrement/end previous actor timed conditions
    /// - advance to next living actor
    /// - return expired conditions and the new active id
    pub fn advance_turn_with_condition_tick(
        &mut self,
    ) -> (Option<String>, Vec<Condition>, Option<String>) {
        let leaving_id = self.current_combatant_id().map(str::to_string);
        let expired = if let Some(id) = &leaving_id {
            self.combatants
                .get_mut(id)
                .map(CombatantState::tick_condition_durations)
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        let next = self.next_turn().map(str::to_string);
        (leaving_id, expired, next)
    }

    pub fn is_over(&self) -> bool {
        let any_players = self
            .combatants
            .values()
            .any(|c| c.is_player && c.is_alive());
        let any_monsters = self
            .combatants
            .values()
            .any(|c| !c.is_player && c.is_alive());
        !(any_players && any_monsters)
    }

    /// First living enemy id for `actor_id` scanning from initiative order.
    pub fn next_enemy_id(&self, actor_id: &str) -> Option<&str> {
        let actor = self.combatants.get(actor_id)?;
        if self.turn_queue.is_empty() {
            return None;
        }

        let start = self
            .turn_queue
            .iter()
            .position(|id| id == actor_id)
            .unwrap_or(self.active_turn);

        for step in 1..=self.turn_queue.len() {
            let idx = (start + step) % self.turn_queue.len();
            let id = &self.turn_queue[idx];
            let Some(candidate) = self.combatants.get(id) else {
                continue;
            };
            if candidate.is_alive() && candidate.is_player != actor.is_player {
                return Some(id.as_str());
            }
        }
        None
    }

    fn reset_current_turn_slots(&mut self) {
        if let Some(c) = self.current_combatant_mut() {
            c.start_turn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn actor(id: &str, is_player: bool, init_mod: i32) -> CombatantState {
        CombatantState::new(
            id,
            id,
            is_player,
            10,
            14,
            30,
            init_mod,
            4,
            DiceExpr::new(1, 6, 2),
        )
    }

    #[test]
    fn next_turn_advances_and_wraps_round() {
        let mut c = CombatState::new_with_seed(
            vec![
                actor("p1", true, 2),
                actor("m1", false, 1),
                actor("m2", false, 0),
            ],
            42,
        );

        let start = c.current_combatant_id().unwrap().to_string();
        c.next_turn();
        let second = c.current_combatant_id().unwrap().to_string();
        assert_ne!(start, second);

        let round_before = c.round;
        c.next_turn();
        c.next_turn(); // wraps
        assert!(c.round >= round_before + 1);
    }

    #[test]
    fn next_turn_skips_dead_combatants() {
        let mut c =
            CombatState::new_with_seed(vec![actor("p1", true, 0), actor("m1", false, 0)], 7);
        c.combatants.get_mut("m1").unwrap().current_hp = 0;
        for _ in 0..5 {
            let id = c.next_turn().unwrap();
            assert_ne!(id, "m1");
        }
    }

    #[test]
    fn can_take_actions_false_when_stunned() {
        let mut c = actor("p1", true, 1);
        c.conditions.insert(Condition::Stunned);
        assert!(!c.can_take_actions());
    }

    #[test]
    fn next_enemy_prefers_opposing_side() {
        let c = CombatState::new_with_seed(
            vec![
                actor("p1", true, 2),
                actor("m1", false, 1),
                actor("m2", false, 0),
            ],
            42,
        );
        assert!(matches!(c.next_enemy_id("p1"), Some("m1" | "m2")));
    }

    #[test]
    fn timed_condition_expires_after_tick() {
        let mut c = actor("p1", true, 1);
        c.apply_condition(Condition::Poisoned, Some(1));
        assert!(c.conditions.contains(&Condition::Poisoned));
        let expired = c.tick_condition_durations();
        assert_eq!(expired, vec![Condition::Poisoned]);
        assert!(!c.conditions.contains(&Condition::Poisoned));
    }
}
