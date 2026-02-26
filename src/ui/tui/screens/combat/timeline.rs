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
pub fn compact_timeline(
    turn_queue: &[String],
    current_id: &str,
    state: &crate::game::combat::CombatState,
    _t: &crate::ui::tui::theme::Theme,
) -> String {
    turn_queue
        .iter()
        .map(|id| {
            if id == current_id {
                format!("[{}]", id)
            } else if let Some(c) = state.combatants.get(id) {
                if c.is_player {
                    format!("({})", id)
                } else {
                    id.clone()
                }
            } else {
                id.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" → ")
}
