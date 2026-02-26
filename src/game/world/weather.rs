use crate::game::story::WorldState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeatherType {
    Clear,
    Rain,
    Fog,
    Ash,
    Snow,
}

impl WeatherType {
    pub fn from_region_tag(tag: &str) -> Self {
        match tag.to_ascii_lowercase().as_str() {
            "rain" => Self::Rain,
            "fog" => Self::Fog,
            "ash" => Self::Ash,
            "snow" => Self::Snow,
            _ => Self::Clear,
        }
    }

    pub fn fov_radius(self) -> u32 {
        match self {
            Self::Fog => 4,
            _ => 8,
        }
    }

    pub fn apply_world_flags(self, world: &mut WorldState) {
        world.set_flag("weather_active");

        if matches!(self, Self::Rain) {
            world.set_flag("weather_rain");
        } else {
            world.clear_flag("weather_rain");
        }

        if matches!(self, Self::Fog) {
            world.set_flag("weather_fog");
        } else {
            world.clear_flag("weather_fog");
        }

        if matches!(self, Self::Ash) {
            world.set_flag("weather_ash");
        } else {
            world.clear_flag("weather_ash");
        }
    }
}
