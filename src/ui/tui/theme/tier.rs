use ratatui::style::{Color, Modifier, Style};
use std::sync::OnceLock;
use super::types::*;
use super::core::*;
use super::icons::*;
use super::helpers::*;
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
    tier
}

pub fn terminal_tier() -> TerminalTier {
    *TIER.get_or_init(init_terminal_tier)
}
