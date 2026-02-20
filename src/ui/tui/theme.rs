use ratatui::style::{Color, Modifier, Style};
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalTier {
    T0,
    T1,
    T2,
    T3,
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
}

static TIER: OnceLock<TerminalTier> = OnceLock::new();

pub fn init_terminal_tier() -> TerminalTier {
    let no_color = std::env::var("NO_COLOR").is_ok();
    let term = std::env::var("TERM").unwrap_or_default();
    let colorterm = std::env::var("COLORTERM").unwrap_or_default().to_lowercase();
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
    match terminal_tier() {
        TerminalTier::T0 => Theme {
            panel_border: Color::Reset,
            text_primary: Color::Reset,
            text_muted: Color::Reset,
            text_emphasis: Color::Reset,
            accent_primary: Color::Reset,
            success: Color::Reset,
            warning: Color::Reset,
            danger: Color::Reset,
        },
        TerminalTier::T1 => Theme {
            panel_border: Color::Gray,
            text_primary: Color::White,
            text_muted: Color::Gray,
            text_emphasis: Color::White,
            accent_primary: Color::Cyan,
            success: Color::Green,
            warning: Color::Yellow,
            danger: Color::Red,
        },
        TerminalTier::T2 => Theme {
            panel_border: Color::Indexed(246),
            text_primary: Color::Indexed(255),
            text_muted: Color::Indexed(245),
            text_emphasis: Color::Indexed(231),
            accent_primary: Color::Indexed(81),
            success: Color::Indexed(77),
            warning: Color::Indexed(220),
            danger: Color::Indexed(203),
        },
        TerminalTier::T3 => Theme {
            panel_border: Color::Rgb(114, 127, 141),
            text_primary: Color::Rgb(236, 240, 241),
            text_muted: Color::Rgb(149, 165, 166),
            text_emphasis: Color::Rgb(255, 255, 255),
            accent_primary: Color::Rgb(52, 152, 219),
            success: Color::Rgb(46, 204, 113),
            warning: Color::Rgb(241, 196, 15),
            danger: Color::Rgb(231, 76, 60),
        },
    }
}

pub fn panel_style() -> Style {
    Style::default().fg(theme().panel_border)
}

pub fn emph_style() -> Style {
    Style::default().fg(theme().text_emphasis).add_modifier(Modifier::BOLD)
}

pub fn accent_style() -> Style {
    Style::default().fg(theme().accent_primary).add_modifier(Modifier::BOLD)
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
