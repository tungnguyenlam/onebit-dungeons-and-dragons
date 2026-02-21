/// Condition (status effect) types.
///
/// Stored as a `HashSet<Condition>` on each entity.
/// Applied and removed via `conditions.rs`. Evaluated at start/end of turn.
///
/// See [docs/gameplay/character.md] for the full list and effects.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Condition {
    Blinded,
    Charmed,
    Deafened,
    Exhaustion(u8), // 1–6
    Frightened,
    Grappled,
    Incapacitated,
    Invisible,
    Paralyzed,
    Petrified,
    Poisoned,
    Prone,
    Restrained,
    Stunned,
    Unconscious,
}

impl Condition {
    /// Human-readable name (for UI display).
    pub fn name(&self) -> String {
        match self {
            Condition::Blinded => "Blinded".into(),
            Condition::Charmed => "Charmed".into(),
            Condition::Deafened => "Deafened".into(),
            Condition::Exhaustion(n) => format!("Exhaustion {n}"),
            Condition::Frightened => "Frightened".into(),
            Condition::Grappled => "Grappled".into(),
            Condition::Incapacitated => "Incapacitated".into(),
            Condition::Invisible => "Invisible".into(),
            Condition::Paralyzed => "Paralyzed".into(),
            Condition::Petrified => "Petrified".into(),
            Condition::Poisoned => "Poisoned".into(),
            Condition::Prone => "Prone".into(),
            Condition::Restrained => "Restrained".into(),
            Condition::Stunned => "Stunned".into(),
            Condition::Unconscious => "Unconscious".into(),
        }
    }

    /// Whether this condition grants advantage on attacks against the entity.
    pub fn grants_advantage_to_attackers(&self) -> bool {
        matches!(
            self,
            Condition::Paralyzed
                | Condition::Petrified
                | Condition::Stunned
                | Condition::Unconscious
        )
    }

    /// Whether this condition imposes disadvantage on the entity's attack rolls.
    pub fn imposes_attack_disadvantage(&self) -> bool {
        matches!(
            self,
            Condition::Blinded
                | Condition::Frightened
                | Condition::Poisoned
                | Condition::Prone
                | Condition::Restrained
        )
    }

    /// Whether the entity cannot take actions.
    pub fn is_incapacitating(&self) -> bool {
        matches!(
            self,
            Condition::Incapacitated
                | Condition::Paralyzed
                | Condition::Petrified
                | Condition::Stunned
                | Condition::Unconscious
        )
    }
}
