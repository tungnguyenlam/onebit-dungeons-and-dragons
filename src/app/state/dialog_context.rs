use crate::data::types::DialogTree;
use crate::game::{
    combat::{CombatState, CombatantState},
    dice::DiceExpr,
    story::{dialog::ResolvedNode, journal::Category as JournalCategory, WorldState},
};
use serde::{Deserialize, Serialize};
/// Placeholder — will be expanded in `src/game/story/dialog.rs`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogContext {
    pub npc_name: String,
    pub tree: DialogTree,
    pub current_node: String,
    pub resolved: ResolvedNode,
}
