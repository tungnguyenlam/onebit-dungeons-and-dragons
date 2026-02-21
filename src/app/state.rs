use crate::data::types::DialogTree;
use crate::game::{
    combat::{CombatState, CombatantState},
    dice::DiceExpr,
    story::{dialog::ResolvedNode, journal::Category as JournalCategory, WorldState},
};

// ---------------------------------------------------------------------------
// AppState
// ---------------------------------------------------------------------------

/// Which screen / mode is currently active. The renderer inspects this to
/// decide which screen module to call.
#[derive(Debug, Clone, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    CharacterCreation,
    WorldMap,
    Combat(CombatContext),
    Dialog(DialogContext),
    Journal,
    Inventory,
    Spellbook,
    Settings,
    GameOver,
}

/// Placeholder — will be expanded in `src/game/combat/`.
#[derive(Debug, Clone)]
pub struct CombatContext {
    pub state: CombatState,
    pub world_state: WorldState,
    pub log: Vec<String>,
    pub seed: u32,
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
        }
    }
}

/// Placeholder — will be expanded in `src/game/story/dialog.rs`.
#[derive(Debug, Clone)]
pub struct DialogContext {
    pub npc_name: String,
    pub tree: DialogTree,
    pub current_node: String,
    pub resolved: ResolvedNode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JournalUiState {
    pub category: JournalCategory,
    pub selected: usize,
    pub detail_scroll: u16,
}

impl Default for JournalUiState {
    fn default() -> Self {
        Self {
            category: JournalCategory::Quest,
            selected: 0,
            detail_scroll: 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MainMenuUiState {
    pub selected: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct SettingsConfig {
    pub enemy_hp_multiplier: f32,
    pub player_damage_multiplier: f32,
    pub reduced_motion: bool,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        Self {
            enemy_hp_multiplier: 1.0,
            player_damage_multiplier: 1.0,
            reduced_motion: crate::ui::tui::theme::reduced_motion(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SettingsUiState {
    pub selected: usize,
}

#[derive(Debug, Clone)]
pub struct CharacterCreationUiState {
    pub selected: usize,
    pub name: String,
    pub class_options: Vec<String>,
    pub class_index: usize,
    pub race_options: Vec<String>,
    pub race_index: usize,
}

impl Default for CharacterCreationUiState {
    fn default() -> Self {
        Self {
            selected: 0,
            name: "Theron".into(),
            class_options: vec!["fighter".into(), "wizard".into(), "rogue".into()],
            class_index: 0,
            race_options: vec!["human".into(), "elf".into(), "dwarf".into()],
            race_index: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusedPane {
    #[default]
    Main,
    Side,
}
