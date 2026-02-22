use super::combat_context::CombatContext;
use super::dialog_context::DialogContext;
use serde::{Deserialize, Serialize};

/// Which screen / mode is currently active. The renderer inspects this to
/// decide which screen module to call.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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