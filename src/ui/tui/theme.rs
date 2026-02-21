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

static TIER: OnceLock<TerminalTier> = OnceLock::new();

pub fn init_terminal_tier() -> TerminalTier {
    let no_color = std::env::var("NO_COLOR").is_ok();
    let term = std::env::var("TERM").unwrap_or_default();
    let colorterm = std::env::var("COLORTERM")
        .unwrap_or_default()
        .to_lowercase();
    let lang = std::env::var("LANG").unwrap_or_default().to_lowercase();

    let utf8 = lang.contains("utf-8") || lang.contains("utf8");
    let truecolor = colorterm.contains("truecolor") || colorterm.contains("24bit");
    let color256 = term.contains("256color");

    let tier = if no_color {
        TerminalTier::T0
    } else if truecolor {
        TerminalTier::T3
    } else if color256 {
        TerminalTier::T2
    } else if utf8 {
        TerminalTier::T1
    } else {
        TerminalTier::T0
    };
    *TIER.get_or_init(|| tier)
}

pub fn terminal_tier() -> TerminalTier {
    *TIER.get_or_init(init_terminal_tier)
}

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

pub fn icon(key: &str) -> &'static str {
    match terminal_tier() {
        TerminalTier::T0 => match key {
            "health" => "HP",
            "quest" => "Q",
            "magic" => "SP",
            "warning" => "!",
            _ => "*",
        },
        _ => match key {
            "health" => "♥",
            "quest" => "📜",
            "magic" => "✦",
            "warning" => "⚠",
            _ => "•",
        },
    }
}

pub fn reduced_motion() -> bool {
    let value = std::env::var("DND_REDUCED_MOTION")
        .or_else(|_| std::env::var("REDUCED_MOTION"))
        .unwrap_or_default()
        .to_lowercase();
    matches!(value.as_str(), "1" | "true" | "yes" | "on")
}

pub fn health_color(current: i32, maximum: i32) -> Color {
    if maximum == 0 {
        return theme().text_muted;
    }
    let ratio = current as f32 / maximum as f32;
    let t = theme();
    if ratio > 0.6 {
        t.health_full
    } else if ratio > 0.3 {
        t.health_medium
    } else {
        t.health_low
    }
}

pub fn mana_color(current: i32, maximum: i32) -> Color {
    if maximum == 0 {
        return theme().text_muted;
    }
    theme().mana
}

pub fn xp_color() -> Color {
    theme().xp
}

pub fn progress_bar(current: i32, maximum: i32, width: usize) -> String {
    if maximum == 0 {
        return "=".repeat(width);
    }
    let filled = ((current as f32 / maximum as f32) * width as f32) as usize;
    let filled = filled.min(width);
    let empty = width - filled;
    "=".repeat(filled) + &"-".repeat(empty)
}

pub fn progress_bar_with_value(current: i32, maximum: i32, width: usize) -> String {
    let bar = progress_bar(current, maximum, width);
    format!("{}[{}/{}]", bar, current, maximum)
}

pub fn gradient_color(progress: f32, colors: &[Color]) -> Color {
    if colors.is_empty() {
        return theme().text_primary;
    }
    if colors.len() == 1 {
        return colors[0];
    }

    let progress = progress.clamp(0.0, 1.0);
    let segment = progress * (colors.len() - 1) as f32;
    let index = segment as usize;
    let t = segment - index as f32;

    match (index, colors.get(index), colors.get(index + 1)) {
        (_, Some(c1), Some(c2)) => lerp_color(*c1, *c2, t),
        _ => *colors.last().unwrap_or(&theme().text_primary),
    }
}

fn lerp_color(c1: Color, c2: Color, t: f32) -> Color {
    match (c1, c2) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8;
            let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8;
            let b = (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8;
            Color::Rgb(r, g, b)
        }
        _ => c2,
    }
}

pub fn color_blind_mode() -> Option<&'static str> {
    let mode = std::env::var("DND_COLOR_BLIND")
        .or_else(|_| std::env::var("COLORBLIND_MODE"))
        .unwrap_or_default()
        .to_lowercase();

    match mode.as_str() {
        "protanopia" | "protan" => Some("protanopia"),
        "deuteranopia" | "deutan" => Some("deuteranopia"),
        "tritanopia" | "tritan" => Some("tritanopia"),
        "monochrome" | "mono" => Some("monochrome"),
        _ => None,
    }
}
