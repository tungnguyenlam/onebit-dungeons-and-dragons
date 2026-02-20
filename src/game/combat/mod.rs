/// Combat systems: initiative, action economy, attack resolution, combat state.
pub mod action;
pub mod attack;
pub mod combat;
pub mod initiative;
pub mod spells;

pub use action::ActionSlots;
pub use attack::{
    apply_damage,
    roll_attack,
    roll_attack_with_seed,
    roll_saving_throw,
    AttackOutcome,
    AttackProfile,
    DefenseProfile,
    HitType,
    RollMode,
    SaveOutcome,
};
pub use combat::{CombatState, CombatantState};
pub use initiative::{
    roll_initiative,
    roll_initiative_with_seed,
    InitiativeCombatant,
    InitiativeOrder,
};
pub use spells::{can_cast, expend_slot, resolve_effect as resolve_spell_effect, SpellEffect};
