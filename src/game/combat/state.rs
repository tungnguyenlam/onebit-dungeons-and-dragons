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
use super::combatant::*;
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