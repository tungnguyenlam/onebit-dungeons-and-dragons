/// Combat systems: initiative, action economy, attack resolution, combat state.
pub mod action;
pub mod ai;
pub mod attack;
#[allow(clippy::module_inception)]
pub mod initiative;
pub mod spells;

// pub use action::ActionSlots;
// pub use ai::{choose_target, EncounterTier};
pub use attack::{
    apply_damage, roll_attack, roll_attack_with_seed, AttackProfile, DefenseProfile, HitType,
    RollMode,
};
pub use combatant::{CombatantState, EnemyAiRole};
pub use state::CombatState;
// pub use initiative::{
//    roll_initiative, roll_initiative_with_seed, InitiativeCombatant, InitiativeOrder,
// };
pub use spells::{can_cast, expend_slot, resolve_effect as resolve_spell_effect, SpellEffect};

pub mod combatant;
pub mod state;
#[cfg(test)]
mod tests;

pub use combatant::*;
pub use state::*;
