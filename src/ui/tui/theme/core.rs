use super::helpers::*;
use super::icons::*;
use super::tier::*;
use super::types::*;
use ratatui::style::{Color, Modifier, Style};
use std::sync::OnceLock;
pub fn theme() -> Theme {
    let color_theme = ColorTheme::from_env();
    match (terminal_tier(), color_theme) {
        (TerminalTier::T0, _) => Theme {
            panel_border: Color::Reset,
            text_primary: Color::Reset,
            text_muted: Color::Reset,
            text_emphasis: Color::Reset,
            accent_primary: Color::Reset,
            success: Color::Reset,
            warning: Color::Reset,
            danger: Color::Reset,
            health_full: Color::Reset,
            health_medium: Color::Reset,
            health_low: Color::Reset,
            mana: Color::Reset,
            xp: Color::Reset,
            player: Color::Reset,
            enemy: Color::Reset,
            npc: Color::Reset,
            item: Color::Reset,
            quest_active: Color::Reset,
            quest_complete: Color::Reset,
            connection: Color::Reset,
            wall: Color::Reset,
            floor: Color::Reset,
        },
        (TerminalTier::T1, _) => Theme {
            panel_border: Color::Gray,
            text_primary: Color::White,
            text_muted: Color::Gray,
            text_emphasis: Color::White,
            accent_primary: Color::Cyan,
            success: Color::Green,
            warning: Color::Yellow,
            danger: Color::Red,
            health_full: Color::Green,
            health_medium: Color::Yellow,
            health_low: Color::Red,
            mana: Color::Blue,
            xp: Color::Magenta,
            player: Color::Green,
            enemy: Color::Red,
            npc: Color::Cyan,
            item: Color::Yellow,
            quest_active: Color::Yellow,
            quest_complete: Color::Green,
            connection: Color::Blue,
            wall: Color::Gray,
            floor: Color::DarkGray,
        },
        (TerminalTier::T2, _) => Theme {
            panel_border: Color::Indexed(246),
            text_primary: Color::Indexed(255),
            text_muted: Color::Indexed(245),
            text_emphasis: Color::Indexed(231),
            accent_primary: Color::Indexed(81),
            success: Color::Indexed(77),
            warning: Color::Indexed(220),
            danger: Color::Indexed(203),
            health_full: Color::Indexed(77),
            health_medium: Color::Indexed(220),
            health_low: Color::Indexed(203),
            mana: Color::Indexed(75),
            xp: Color::Indexed(171),
            player: Color::Indexed(77),
            enemy: Color::Indexed(203),
            npc: Color::Indexed(81),
            item: Color::Indexed(220),
            quest_active: Color::Indexed(220),
            quest_complete: Color::Indexed(77),
            connection: Color::Indexed(75),
            wall: Color::Indexed(238),
            floor: Color::Indexed(236),
        },
        (TerminalTier::T3, ColorTheme::Dark) => Theme {
            panel_border: Color::Rgb(114, 127, 141),
            text_primary: Color::Rgb(236, 240, 241),
            text_muted: Color::Rgb(149, 165, 166),
            text_emphasis: Color::Rgb(255, 255, 255),
            accent_primary: Color::Rgb(52, 152, 219),
            success: Color::Rgb(46, 204, 113),
            warning: Color::Rgb(241, 196, 15),
            danger: Color::Rgb(231, 76, 60),
            health_full: Color::Rgb(46, 204, 113),
            health_medium: Color::Rgb(241, 196, 15),
            health_low: Color::Rgb(231, 76, 60),
            mana: Color::Rgb(52, 152, 219),
            xp: Color::Rgb(155, 89, 182),
            player: Color::Rgb(46, 204, 113),
            enemy: Color::Rgb(231, 76, 60),
            npc: Color::Rgb(52, 152, 219),
            item: Color::Rgb(241, 196, 15),
            quest_active: Color::Rgb(241, 196, 15),
            quest_complete: Color::Rgb(46, 204, 113),
            connection: Color::Rgb(52, 152, 219),
            wall: Color::Rgb(127, 140, 141),
            floor: Color::Rgb(189, 195, 199),
        },
        (TerminalTier::T3, ColorTheme::Light) => Theme {
            panel_border: Color::Rgb(100, 100, 100),
            text_primary: Color::Rgb(50, 50, 50),
            text_muted: Color::Rgb(100, 100, 100),
            text_emphasis: Color::Rgb(0, 0, 0),
            accent_primary: Color::Rgb(0, 100, 200),
            success: Color::Rgb(0, 150, 50),
            warning: Color::Rgb(200, 150, 0),
            danger: Color::Rgb(200, 50, 50),
            health_full: Color::Rgb(0, 150, 50),
            health_medium: Color::Rgb(200, 150, 0),
            health_low: Color::Rgb(200, 50, 50),
            mana: Color::Rgb(0, 100, 200),
            xp: Color::Rgb(130, 50, 150),
            player: Color::Rgb(0, 150, 50),
            enemy: Color::Rgb(200, 50, 50),
            npc: Color::Rgb(0, 100, 200),
            item: Color::Rgb(200, 150, 0),
            quest_active: Color::Rgb(200, 150, 0),
            quest_complete: Color::Rgb(0, 150, 50),
            connection: Color::Rgb(0, 100, 200),
            wall: Color::Rgb(150, 150, 150),
            floor: Color::Rgb(200, 200, 200),
        },
        (TerminalTier::T3, ColorTheme::HighContrast) => Theme {
            panel_border: Color::White,
            text_primary: Color::White,
            text_muted: Color::White,
            text_emphasis: Color::White,
            accent_primary: Color::Cyan,
            success: Color::Green,
            warning: Color::Yellow,
            danger: Color::Red,
            health_full: Color::Green,
            health_medium: Color::Yellow,
            health_low: Color::Red,
            mana: Color::Cyan,
            xp: Color::Magenta,
            player: Color::Green,
            enemy: Color::Red,
            npc: Color::Cyan,
            item: Color::Yellow,
            quest_active: Color::Yellow,
            quest_complete: Color::Green,
            connection: Color::Cyan,
            wall: Color::White,
            floor: Color::White,
        },
    }
}

pub fn panel_style() -> Style {
    Style::default().fg(theme().panel_border)
}

pub fn panel_style_focused() -> Style {
    Style::default()
        .fg(theme().text_emphasis)
        .add_modifier(Modifier::BOLD)
}

pub fn emph_style() -> Style {
    Style::default()
        .fg(theme().text_emphasis)
        .add_modifier(Modifier::BOLD)
}

pub fn accent_style() -> Style {
    Style::default()
        .fg(theme().accent_primary)
        .add_modifier(Modifier::BOLD)
}

pub fn muted_style() -> Style {
    Style::default().fg(theme().text_muted)
}
