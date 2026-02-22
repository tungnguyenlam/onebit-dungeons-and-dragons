use crate::game::story::journal::Category as JournalCategory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MainMenuUiState {
    pub selected: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsUiState {
    pub selected: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FocusedPane {
    #[default]
    Main,
    Side,
}