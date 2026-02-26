/// Character module — ability scores, skills, conditions, level progression.
///
/// This module has no UI or data-loading dependencies.
/// It receives already-parsed `ClassDef` / `RaceDef` values from `src/data/`.
pub mod conditions;
pub mod feats;
pub mod progression;
pub mod skills;
pub mod stats;

// pub use conditions::Condition;
// pub use progression::proficiency_bonus;
// pub use skills::{Skill, SkillSet};
pub use feats::FeatRegistry;
pub use stats::{AbilityScores, Character};
