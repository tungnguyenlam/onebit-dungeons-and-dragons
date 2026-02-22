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
use super::battlefield::render_battlefield;
use super::utils::style_for_log_line;
use super::timeline::compact_timeline;
pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::vertical([
        Constraint::Length(4),
        Constraint::Length(2),
        Constraint::Min(10),
        Constraint::Length(8),
        Constraint::Length(6),
    ])
    .split(area);

    let t = theme();

    let Some(ctx) = (match &app.state {
        AppState::Combat(ctx) => Some(ctx),
        _ => None,
    }) else {
        let p = Paragraph::new("Combat screen requested outside combat state.").block(
            Block::default()
                .title("Combat")
                .borders(Borders::ALL)
                .style(Style::default().fg(t.panel_border)),
        );
        frame.render_widget(p, area);
        return;
    };

    // Turn order banner with semantic colors
    let current_id = ctx.state.current_combatant_id().unwrap_or("?");
    let active = ctx
        .state
        .current_combatant()
        .map(|c| {
            let role_label = if c.is_player { "PLAYER" } else { "ENEMY" };
            let color = if c.is_player { t.player } else { t.enemy };
            (format!("{} ({})", c.name, role_label), color)
        })
        .unwrap_or_else(|| ("Unknown".into(), t.text_muted));

    let turn_banner = Paragraph::new(vec![
        Line::from(format!("Round {} | Active: ", ctx.state.round)),
        Line::from(active.0),
    ])
    .style(Style::default().fg(t.text_primary))
    .block(
        Block::default()
            .title("Initiative")
            .borders(Borders::ALL)
            .style(Style::default().fg(t.panel_border)),
    );
    frame.render_widget(turn_banner, chunks[0]);

    // Timeline with current highlighted
    let timeline = compact_timeline(&ctx.state.turn_queue, current_id, &ctx.state, &t);
    let timeline_text = Paragraph::new(Line::from(timeline))
        .style(Style::default().fg(t.text_muted))
        .block(
            Block::default()
                .title("Timeline")
                .borders(Borders::ALL)
                .style(Style::default().fg(t.panel_border)),
        );
    frame.render_widget(timeline_text, chunks[1]);

    // Battlefield grid with semantic colors
    let battlefield = render_battlefield(&ctx);
    frame.render_widget(
        battlefield.block(
            Block::default()
                .title("Battlefield")
                .borders(Borders::ALL)
                .style(Style::default().fg(t.panel_border)),
        ),
        chunks[2],
    );

    // Combatants HUD with semantic colors
    let hud_lines: Vec<Line<'_>> = ctx
        .state
        .turn_queue
        .iter()
        .filter_map(|id| ctx.state.combatants.get(id))
        .map(|c| {
            let marker = if c.id == current_id { ">" } else { " " };
            let action = if c.action_slots.action { "A" } else { "-" };
            let bonus = if c.action_slots.bonus_action {
                "B"
            } else {
                "-"
            };
            let reaction = if c.action_slots.reaction { "R" } else { "-" };

            let conditions = if c.conditions.is_empty() {
                "-".to_string()
            } else {
                let mut labels: Vec<String> = c
                    .conditions
                    .iter()
                    .map(|cond| {
                        if let Some(rounds) = c.condition_duration(cond) {
                            format!("{}({rounds})", cond.name())
                        } else {
                            cond.name()
                        }
                    })
                    .collect();
                labels.sort();
                labels.join(",")
            };

            let color = if c.is_player { t.player } else { t.enemy };
            let hp_color = theme::health_color(c.current_hp, c.max_hp);

            Line::from(vec![
                ratatui::text::Span::raw(format!("{} ", marker)),
                ratatui::text::Span::styled(
                    format!("{:10}", c.name),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
                ratatui::text::Span::raw(" HP "),
                ratatui::text::Span::styled(
                    progress_bar(c.current_hp, c.max_hp.max(1), 6),
                    Style::default().fg(hp_color),
                ),
                ratatui::text::Span::raw(format!(
                    " {}/{} AC{} ",
                    c.current_hp, c.max_hp, c.armor_class
                )),
                ratatui::text::Span::raw(format!(
                    "[{}] Slots {}{}{} Move{:>2} Cond {}",
                    if c.is_player { "player" } else { "enemy" },
                    action,
                    bonus,
                    reaction,
                    c.action_slots.movement_remaining,
                    conditions
                )),
            ])
        })
        .collect();

    let hud = Paragraph::new(hud_lines)
        .block(
            Block::default()
                .title(format!("Combatants {}", theme::icon("health")))
                .borders(Borders::ALL)
                .style(Style::default().fg(t.panel_border)),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(hud, chunks[3]);

    // Log panel with styled entries
    let log_height = chunks[4].height.saturating_sub(2) as usize;
    let start = ctx.log.len().saturating_sub(log_height);
    let log_lines: Vec<Line<'_>> = ctx.log[start..]
        .iter()
        .map(|line| Line::from(line.clone()).style(style_for_log_line(line)))
        .collect();
    let log = Paragraph::new(log_lines)
        .block(
            Block::default()
                .title("Combat Log")
                .borders(Borders::ALL)
                .style(Style::default().fg(t.panel_border)),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(log, chunks[4]);
}