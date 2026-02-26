use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
