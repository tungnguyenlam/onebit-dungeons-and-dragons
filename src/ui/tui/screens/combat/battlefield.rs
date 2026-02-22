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
use super::timeline::*;
use super::utils::*;
pub fn render_battlefield(ctx: &crate::app::state::CombatContext) -> Paragraph<'static> {
    let t = theme();

    // Build combatant list with positions
    let mut lines = Vec::new();
    lines.push(Line::from(vec![ratatui::text::Span::styled(
        "Combat Positions:",
        Style::default()
            .fg(t.text_emphasis)
            .add_modifier(Modifier::BOLD),
    )]));
    lines.push(Line::from(""));

    for (idx, c) in ctx.state.turn_queue.iter().enumerate() {
        if let Some(comb) = ctx.state.combatants.get(c) {
            let is_selected = ctx.selected_enemy_id.as_ref() == Some(c);
            let marker = if comb.is_player {
                "▶"
            } else if is_selected {
                "*"
            } else {
                "●"
            };
            let color = if comb.is_player {
                t.player
            } else if is_selected {
                t.warning
            } else {
                t.enemy
            };
            let hp_bar = progress_bar(comb.current_hp, comb.max_hp.max(1), 8);
            let hp_color = theme::health_color(comb.current_hp, comb.max_hp);

            lines.push(Line::from(vec![
                ratatui::text::Span::raw(format!("{} ", marker)),
                ratatui::text::Span::styled(
                    format!("{:12}", comb.name),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
                ratatui::text::Span::raw(" "),
                ratatui::text::Span::styled(hp_bar, Style::default().fg(hp_color)),
                ratatui::text::Span::raw(format!(" {}/{}", comb.current_hp, comb.max_hp)),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![ratatui::text::Span::styled(
        "Controls:",
        Style::default().fg(t.text_emphasis),
    )]));
    lines.push(Line::from("  a/1 attack  2 heal  3 second wind  4/f flee"));
    lines.push(Line::from("  . wait  Esc leave"));

    Paragraph::new(lines).style(Style::default().fg(t.text_primary))
}