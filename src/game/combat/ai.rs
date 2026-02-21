/// M23: Combat AI — role-based decision making and focus-fire targeting.
///
/// Each enemy combatant has an `EnemyAiRole`. This module resolves which
/// action and target the AI chooses on its turn, given the full combat state.
use crate::game::combat::combat::{CombatState, CombatantState, EnemyAiRole};

// ---------------------------------------------------------------------------
// Targeting strategies
// ---------------------------------------------------------------------------

/// Which enemy combatant should `actor_id` attack, according to its AI role?
///
/// - **Melee**: prefer the lowest-HP player (focus-fire / finish-off).
/// - **Ranged**: prefer the highest-HP player (softening for later rounds).
/// - **Spellcaster**: prefer the player with the lowest AC (easiest to hit).
///
/// Falls back to `CombatState::next_enemy_id` if no better target is found.
pub fn choose_target<'a>(state: &'a CombatState, actor_id: &str) -> Option<&'a str> {
    let actor = state.combatants.get(actor_id)?;
    if actor.is_player {
        return None; // only resolve AI targets for non-player combatants
    }

    let living_players: Vec<&CombatantState> = state
        .combatants
        .values()
        .filter(|c| c.is_player && c.is_alive())
        .collect();

    if living_players.is_empty() {
        return None;
    }

    let chosen = match actor.enemy_role {
        EnemyAiRole::Melee => {
            // Focus-fire: lowest current HP  → finish them off
            living_players.iter().min_by_key(|c| c.current_hp).copied()
        }
        EnemyAiRole::Ranged => {
            // Soften: highest current HP → spread damage
            living_players.iter().max_by_key(|c| c.current_hp).copied()
        }
        EnemyAiRole::Spellcaster => {
            // Easiest to hit: lowest AC
            living_players.iter().min_by_key(|c| c.armor_class).copied()
        }
    };

    chosen.map(|c| c.id.as_str())
}

// ---------------------------------------------------------------------------
// Encounter tier
// ---------------------------------------------------------------------------

/// Difficulty tier for an encounter, derived from total monster CR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EncounterTier {
    /// Single low-CR monster (CR ≤ 0.25).
    Trivial,
    /// Moderate threat — a single monster or small group.
    Easy,
    /// Challenging group; requires resource spending.
    Medium,
    /// Boss-tier or elite group; threatens TPK.
    Hard,
    /// Deadly — multiple high-CR monsters.
    Deadly,
}

impl EncounterTier {
    /// Derive a tier from the summed challenge rating of all monsters.
    pub fn from_total_cr(total_cr: f32) -> Self {
        if total_cr <= 0.25 {
            Self::Trivial
        } else if total_cr <= 1.0 {
            Self::Easy
        } else if total_cr <= 3.0 {
            Self::Medium
        } else if total_cr <= 6.0 {
            Self::Hard
        } else {
            Self::Deadly
        }
    }

    /// Human-readable label used in UI/journal entries.
    pub fn label(self) -> &'static str {
        match self {
            Self::Trivial => "Trivial",
            Self::Easy => "Easy",
            Self::Medium => "Medium",
            Self::Hard => "Hard",
            Self::Deadly => "Deadly",
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{combat::combat::CombatantState, dice::DiceExpr};

    fn make_player(id: &str, hp: i32, ac: i32) -> CombatantState {
        let mut c = CombatantState::new(id, id, true, hp, ac, 30, 2, 4, DiceExpr::new(1, 6, 2));
        c.current_hp = hp;
        c
    }

    fn make_enemy(id: &str, role: EnemyAiRole) -> CombatantState {
        let mut c = CombatantState::new(id, id, false, 10, 12, 30, 0, 3, DiceExpr::new(1, 4, 0));
        c.enemy_role = role;
        c
    }

    fn combat_with(combatants: Vec<CombatantState>) -> CombatState {
        CombatState::new_with_seed(combatants, 42)
    }

    #[test]
    fn melee_ai_targets_lowest_hp_player() {
        let c = combat_with(vec![
            make_player("p_low", 3, 15),
            make_player("p_high", 20, 15),
            make_enemy("m1", EnemyAiRole::Melee),
        ]);
        let target = choose_target(&c, "m1").unwrap();
        assert_eq!(target, "p_low");
    }

    #[test]
    fn ranged_ai_targets_highest_hp_player() {
        let c = combat_with(vec![
            make_player("p_low", 3, 15),
            make_player("p_high", 20, 15),
            make_enemy("m1", EnemyAiRole::Ranged),
        ]);
        let target = choose_target(&c, "m1").unwrap();
        assert_eq!(target, "p_high");
    }

    #[test]
    fn spellcaster_ai_targets_lowest_ac_player() {
        let c = combat_with(vec![
            make_player("p_soft", 15, 10),
            make_player("p_tank", 15, 18),
            make_enemy("m1", EnemyAiRole::Spellcaster),
        ]);
        let target = choose_target(&c, "m1").unwrap();
        assert_eq!(target, "p_soft");
    }

    #[test]
    fn player_actor_returns_none() {
        let c = combat_with(vec![
            make_player("p1", 15, 14),
            make_enemy("m1", EnemyAiRole::Melee),
        ]);
        assert!(choose_target(&c, "p1").is_none());
    }

    #[test]
    fn no_target_when_all_players_dead() {
        let mut c = combat_with(vec![
            make_player("p1", 10, 14),
            make_enemy("m1", EnemyAiRole::Melee),
        ]);
        c.combatants.get_mut("p1").unwrap().current_hp = 0;
        assert!(choose_target(&c, "m1").is_none());
    }

    #[test]
    fn encounter_tier_ranges() {
        assert_eq!(EncounterTier::from_total_cr(0.0), EncounterTier::Trivial);
        assert_eq!(EncounterTier::from_total_cr(0.25), EncounterTier::Trivial);
        assert_eq!(EncounterTier::from_total_cr(0.5), EncounterTier::Easy);
        assert_eq!(EncounterTier::from_total_cr(1.0), EncounterTier::Easy);
        assert_eq!(EncounterTier::from_total_cr(2.0), EncounterTier::Medium);
        assert_eq!(EncounterTier::from_total_cr(4.0), EncounterTier::Hard);
        assert_eq!(EncounterTier::from_total_cr(10.0), EncounterTier::Deadly);
    }

    #[test]
    fn tier_ordering_is_consistent() {
        assert!(EncounterTier::Trivial < EncounterTier::Easy);
        assert!(EncounterTier::Easy < EncounterTier::Medium);
        assert!(EncounterTier::Medium < EncounterTier::Hard);
        assert!(EncounterTier::Hard < EncounterTier::Deadly);
    }

    #[test]
    fn tier_labels_are_nonempty() {
        for tier in [
            EncounterTier::Trivial,
            EncounterTier::Easy,
            EncounterTier::Medium,
            EncounterTier::Hard,
            EncounterTier::Deadly,
        ] {
            assert!(!tier.label().is_empty());
        }
    }
}
