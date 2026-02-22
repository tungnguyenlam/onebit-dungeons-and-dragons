use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
            reduced_motion: false,
        }
    }
}