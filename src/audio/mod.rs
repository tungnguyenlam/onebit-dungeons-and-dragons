use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundCategory {
    Ui,
    Combat,
    Ambient,
    Environment,
    Magic,
    Item,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmbientType {
    None,
    Cave,
    Forest,
    Dungeon,
    Water,
    Fire,
    Wind,
    City,
    Combat,
}

impl AmbientType {
    pub fn from_region(region_slug: &str) -> Self {
        match region_slug {
            s if s.contains("cave") || s.contains("mine") => AmbientType::Cave,
            s if s.contains("forest") || s.contains("woods") => AmbientType::Forest,
            s if s.contains("dungeon") || s.contains("ruined") => AmbientType::Dungeon,
            s if s.contains("water") || s.contains("sunken") || s.contains("lake") => {
                AmbientType::Water
            }
            s if s.contains("fire") || s.contains("volcanic") || s.contains("ember") => {
                AmbientType::Fire
            }
            s if s.contains("mountain") || s.contains("peak") || s.contains("wind") => {
                AmbientType::Wind
            }
            s if s.contains("city") || s.contains("town") => AmbientType::City,
            _ => AmbientType::None,
        }
    }
}

pub struct AudioConfig {
    pub enabled: bool,
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub ambient_volume: f32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            master_volume: 0.7,
            music_volume: 0.5,
            sfx_volume: 0.8,
            ambient_volume: 0.4,
        }
    }
}

impl AudioConfig {
    pub fn from_env() -> Self {
        let enabled = std::env::var("DND_SOUND")
            .or_else(|_| std::env::var("DND_AUDIO"))
            .unwrap_or_default()
            .to_lowercase();

        let enabled = !matches!(enabled.as_str(), "0" | "false" | "no" | "off");

        let master = std::env::var("DND_VOLUME_MASTER")
            .or_else(|_| std::env::var("DND_VOLUME"))
            .ok()
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(0.7);

        let music = std::env::var("DND_VOLUME_MUSIC")
            .ok()
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(0.5);

        let sfx = std::env::var("DND_VOLUME_SFX")
            .ok()
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(0.8);

        let ambient = std::env::var("DND_VOLUME_AMBIENT")
            .ok()
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(0.4);

        Self {
            enabled,
            master_volume: master.clamp(0.0, 1.0),
            music_volume: music.clamp(0.0, 1.0),
            sfx_volume: sfx.clamp(0.0, 1.0),
            ambient_volume: ambient.clamp(0.0, 1.0),
        }
    }

    pub fn effective_volume(&self, category: SoundCategory) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        let category_volume = match category {
            SoundCategory::Ui => self.sfx_volume,
            SoundCategory::Combat => self.sfx_volume,
            SoundCategory::Ambient => self.ambient_volume,
            SoundCategory::Environment => self.sfx_volume,
            SoundCategory::Magic => self.sfx_volume,
            SoundCategory::Item => self.sfx_volume,
        };

        self.master_volume * category_volume
    }
}

pub struct AudioEngine {
    pub config: AudioConfig,
    pub current_ambient: AmbientType,
    pub is_combat_music_playing: bool,
    last_ambient_change: u64,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self {
            config: AudioConfig::from_env(),
            current_ambient: AmbientType::None,
            is_combat_music_playing: false,
            last_ambient_change: 0,
        }
    }

    pub fn set_ambient(&mut self, ambient: AmbientType, turn: u64) {
        if self.current_ambient != ambient {
            self.current_ambient = ambient;
            self.last_ambient_change = turn;
        }
    }

    pub fn start_combat_music(&mut self) {
        self.is_combat_music_playing = true;
    }

    pub fn stop_combat_music(&mut self) {
        self.is_combat_music_playing = false;
    }

    pub fn play_sound(&self, sound_id: &str, category: SoundCategory) -> Option<String> {
        let volume = self.config.effective_volume(category);
        if volume <= 0.0 {
            return None;
        }

        Some(format!("{}@{:.2}", sound_id, volume))
    }

    pub fn ui_confirm(&self) -> Option<String> {
        self.play_sound("ui_confirm", SoundCategory::Ui)
    }

    pub fn ui_cancel(&self) -> Option<String> {
        self.play_sound("ui_cancel", SoundCategory::Ui)
    }

    pub fn ui_move(&self) -> Option<String> {
        self.play_sound("ui_move", SoundCategory::Ui)
    }

    pub fn combat_attack(&self) -> Option<String> {
        self.play_sound("combat_attack", SoundCategory::Combat)
    }

    pub fn combat_hit(&self) -> Option<String> {
        self.play_sound("combat_hit", SoundCategory::Combat)
    }

    pub fn combat_critical(&self) -> Option<String> {
        self.play_sound("combat_critical", SoundCategory::Combat)
    }

    pub fn combat_miss(&self) -> Option<String> {
        self.play_sound("combat_miss", SoundCategory::Combat)
    }

    pub fn item_pickup(&self) -> Option<String> {
        self.play_sound("item_pickup", SoundCategory::Item)
    }

    pub fn item_equip(&self) -> Option<String> {
        self.play_sound("item_equip", SoundCategory::Item)
    }

    pub fn magic_cast(&self) -> Option<String> {
        self.play_sound("magic_cast", SoundCategory::Magic)
    }

    pub fn step(&self) -> Option<String> {
        self.play_sound("step", SoundCategory::Environment)
    }

    pub fn door_open(&self) -> Option<String> {
        self.play_sound("door_open", SoundCategory::Environment)
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_ambient_sound(ambient: AmbientType) -> &'static str {
    match ambient {
        AmbientType::None => "",
        AmbientType::Cave => "cave_drip",
        AmbientType::Forest => "forest_birds",
        AmbientType::Dungeon => "dungeon_echo",
        AmbientType::Water => "water_lap",
        AmbientType::Fire => "fire_crackle",
        AmbientType::Wind => "wind_how",
        AmbientType::City => "city_murmur",
        AmbientType::Combat => "combat_drums",
    }
}
