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

fn render_battlefield(ctx: &crate::app::state::CombatContext) -> Paragraph<'static> {
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
            let marker = if comb.is_player { "▶" } else { "●" };
            let color = if comb.is_player { t.player } else { t.enemy };
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
    lines.push(Line::from("  a/1 attack  2 heal  3 second wind"));
    lines.push(Line::from("  . wait  Esc leave"));

    Paragraph::new(lines).style(Style::default().fg(t.text_primary))
}

fn compact_timeline(
    turn_queue: &[String],
    current_id: &str,
    state: &crate::game::combat::CombatState,
    t: &crate::ui::tui::theme::Theme,
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

fn style_for_log_line(line: &str) -> Style {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_prefers_high_signal_events() {
        let log = vec![
            "Goblin now has 3 HP.".to_string(),
            "Player hits Goblin for 7 damage (d20=15 total=19).".to_string(),
        ];
        assert!(last_turn_summary(&log).contains("hits Goblin"));
    }

    fn last_turn_summary(log: &[String]) -> String {
        for line in log.iter().rev() {
            if line.contains("CRITS")
                || line.contains("critical")
                || line.contains("drops to 0 HP")
                || line.contains("hits")
                || line.contains("miss")
                || line.contains("recovers")
                || line.contains("restoring")
                || line.contains("expired")
            {
                return line.clone();
            }
        }
        "No major action yet.".to_string()
    }
}
