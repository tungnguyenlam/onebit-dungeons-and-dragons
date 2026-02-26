use super::core::*;
use super::helpers::*;
use super::icons::*;
use super::tier::*;
use ratatui::style::{Color, Modifier, Style};
use std::sync::OnceLock;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalTier {
    T0,
    T1,
    T2,
    T3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorTheme {
    Dark,
    Light,
    HighContrast,
}

impl ColorTheme {
    pub fn from_env() -> Self {
        let theme_var = std::env::var("DND_THEME")
            .or_else(|_| std::env::var("DND_COLOR_SCHEME"))
            .unwrap_or_default()
            .to_lowercase();

        match theme_var.as_str() {
            "light" => ColorTheme::Light,
            "high" | "high-contrast" | "contrast" => ColorTheme::HighContrast,
            _ => ColorTheme::Dark,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub panel_border: Color,
    pub text_primary: Color,
    pub text_muted: Color,
    pub text_emphasis: Color,
    pub accent_primary: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub health_full: Color,
    pub health_medium: Color,
    pub health_low: Color,
    pub mana: Color,
    pub xp: Color,
    pub player: Color,
    pub enemy: Color,
    pub npc: Color,
    pub item: Color,
    pub quest_active: Color,
    pub quest_complete: Color,
    pub connection: Color,
    pub wall: Color,
    pub floor: Color,
}

impl Theme {
    pub fn wall(&self) -> &'static str {
        "#"
    }
    pub fn floor(&self) -> &'static str {
        "."
    }
}
