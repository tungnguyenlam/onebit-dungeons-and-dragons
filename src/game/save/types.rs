use crate::{
    app::AppState,
    game::{
        character::Character,
        story::{Journal, WorldState},
    },
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const SAVE_FORMAT_VERSION: u32 = 1;
pub const SAVE_FORMAT_MAX_VERSION: u32 = 1;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGame {
    #[serde(default = "default_save_version")]
    pub format_version: u32,
    pub turn: u64,
    pub player: Character,
    pub world_state: WorldState,
    pub journal: Journal,
    pub region_slug: String,
    pub room_id: String,
    pub player_pos: (u32, u32),
    #[serde(default)]
    pub state: AppState,
    #[serde(default)]
    pub menu_ui: crate::app::state::MainMenuUiState,
    #[serde(default)]
    pub char_creation_ui: crate::app::state::CharacterCreationUiState,
    #[serde(default)]
    pub journal_ui: crate::app::state::JournalUiState,
    #[serde(default)]
    pub settings_ui: crate::app::state::SettingsUiState,
}

fn default_save_version() -> u32 {
    // Legacy saves omitted `format_version`; serde default gives 0, which
    // we normalise to SAVE_FORMAT_VERSION after parsing.
    0
}