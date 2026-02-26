use crate::data::types::DialogTree;
use crate::game::{
    combat::{CombatState, CombatantState},
    dice::DiceExpr,
    story::{dialog::ResolvedNode, journal::Category as JournalCategory, WorldState},
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatContext {
    pub state: CombatState,
    pub world_state: WorldState,
    pub log: Vec<String>,
    pub seed: u32,
    pub selected_enemy_id: Option<String>,
}

impl Default for CombatContext {
    fn default() -> Self {
        let seed = 1337u32;
        let state = CombatState::new_with_seed(
            vec![
                CombatantState::new(
                    "player",
                    "Theron",
                    true,
                    24,
                    16,
                    30,
                    2,
                    5,
                    DiceExpr::new(1, 8, 3),
                ),
                CombatantState::new(
                    "goblin_a",
                    "Goblin A",
                    false,
                    10,
                    13,
                    30,
                    2,
                    4,
                    DiceExpr::new(1, 6, 2),
                ),
                CombatantState::new(
                    "goblin_b",
                    "Goblin B",
                    false,
                    10,
                    12,
                    30,
                    2,
                    4,
                    DiceExpr::new(1, 6, 2),
                ),
            ],
            seed as u64,
        );
        let mut state = state;
        if let Some(goblin_a) = state.combatants.get_mut("goblin_a") {
            goblin_a.on_hit_condition =
                Some(crate::game::character::conditions::Condition::Poisoned);
        }
        Self {
            state,
            world_state: WorldState::new(),
            log: vec![
                "Combat started.".into(),
                "Press 'a' to attack.".into(),
                "Press '.' to advance turn.".into(),
                "Press Esc to leave combat.".into(),
            ],
            seed,
            selected_enemy_id: None,
        }
    }
}
