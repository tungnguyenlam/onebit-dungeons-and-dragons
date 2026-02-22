use crate::app::{App, AppState};
use crate::game::combat::EnemyAiRole;
use crate::ui::tui::theme::{self, progress_bar, theme};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
pub fn style_for_log_line(line: &str) -> Style {
    let base = if line.contains("CRITS") || line.contains("critical") {
        Style::default()
            .fg(theme().warning)
            .add_modifier(Modifier::BOLD)
    } else if line.contains("miss") {
        Style::default().fg(theme().text_muted)
    } else if line.contains("recovers") || line.contains("restoring") || line.contains("heals") {
        Style::default().fg(theme().success)
    } else if line.contains("drops to 0 HP") || line.contains("dies") {
        Style::default()
            .fg(theme().danger)
            .add_modifier(Modifier::BOLD)
    } else if line.contains("hits") || line.contains("damage") {
        Style::default().fg(theme().danger)
    } else {
        Style::default().fg(theme().text_primary)
    };
    if theme::reduced_motion() {
        base.remove_modifier(Modifier::RAPID_BLINK | Modifier::SLOW_BLINK)
    } else {
        base
    }
}