use crate::app::{App, AppState};
use crate::game::combat::EnemyAiRole;
use crate::ui::tui::theme;
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
        Constraint::Length(3),
        Constraint::Min(8),
        Constraint::Length(8),
        Constraint::Length(7),
    ])
    .split(area);

    let Some(ctx) = (match &app.state {
        AppState::Combat(ctx) => Some(ctx),
        _ => None,
    }) else {
        let p = Paragraph::new("Combat screen requested outside combat state.")
            .block(
                Block::default()
                    .title("Combat")
                    .borders(Borders::ALL)
                    .style(theme::panel_style()),
            );
        frame.render_widget(p, area);
        return;
    };

    // Turn order banner.
    let current_id = ctx.state.current_combatant_id().unwrap_or("?");
    let active = ctx
        .state
        .current_combatant()
        .map(|c| {
            format!(
                "{} ({})",
                c.name,
                if c.is_player { "PLAYER" } else { "ENEMY" }
            )
        })
        .unwrap_or_else(|| "Unknown".into());
    let timeline = compact_timeline(&ctx.state.turn_queue, current_id);
    let turn_banner = Paragraph::new(vec![
        Line::from(format!("Round {} | Active: {}", ctx.state.round, active)),
        Line::from(format!("Timeline: {timeline}")),
    ])
    .style(Style::default().fg(theme::theme().text_primary))
    .block(
        Block::default()
            .title("Initiative")
            .borders(Borders::ALL)
            .style(theme::panel_style()),
    );
    frame.render_widget(turn_banner, chunks[0]);

    let summary = Paragraph::new(Line::from(last_turn_summary(&ctx.log)))
        .style(Style::default().fg(theme::theme().accent_primary))
        .block(
            Block::default()
                .title("Last Turn Summary")
                .borders(Borders::ALL)
                .style(theme::panel_style()),
        );
    frame.render_widget(summary, chunks[1]);

    // Battlefield placeholder.
    let battlefield = Paragraph::new(
        "Battlefield view is placeholder for now.\n\nControls:\n  - 'a' or '1' attack\n  - '2' drink healing potion (action)\n  - '3' second wind (bonus action)\n  - '.' wait/end turn\n  - Esc leave combat\n\nLegend: A=Action B=Bonus R=Reaction",
    )
    .style(theme::muted_style())
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .title("Battlefield")
            .borders(Borders::ALL)
            .style(theme::panel_style()),
    );
    frame.render_widget(battlefield, chunks[2]);

    // Combatants HUD.
    let hud_lines: Vec<Line<'_>> = ctx
        .state
        .turn_queue
        .iter()
        .filter_map(|id| ctx.state.combatants.get(id))
        .map(|c| {
            let marker = if c.id == current_id { ">" } else { " " };
            let action = if c.action_slots.action { "A" } else { "-" };
            let bonus = if c.action_slots.bonus_action { "B" } else { "-" };
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
            let role = if c.is_player {
                "player".to_string()
            } else {
                match c.enemy_role {
                    EnemyAiRole::Melee => "melee".to_string(),
                    EnemyAiRole::Ranged => "ranged".to_string(),
                    EnemyAiRole::Spellcaster => "spellcaster".to_string(),
                }
            };
            Line::from(format!(
                "{marker} {:10} HP {:>2}/{:<2} AC {:>2} [{role}] Slots {action}{bonus}{reaction} Move {:>2} Cond {}",
                c.name, c.current_hp, c.max_hp, c.armor_class, c.action_slots.movement_remaining, conditions
            ))
            .style(if c.id == current_id {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            })
        })
        .collect();
    let hud = Paragraph::new(hud_lines)
        .block(
            Block::default()
                .title(format!("Combatants {}", theme::icon("health")))
                .borders(Borders::ALL)
                .style(theme::panel_style()),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(hud, chunks[3]);

    // Log panel (latest lines only).
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
                .style(theme::panel_style()),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(log, chunks[4]);
}

fn compact_timeline(turn_queue: &[String], current_id: &str) -> String {
    turn_queue
        .iter()
        .map(|id| {
            if id == current_id {
                format!("[{id}]")
            } else {
                id.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" -> ")
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

fn style_for_log_line(line: &str) -> Style {
    let base = if line.contains("CRITS") || line.contains("critical") {
        Style::default().fg(theme::theme().warning).add_modifier(Modifier::BOLD)
    } else if line.contains("miss") {
        Style::default().fg(theme::theme().text_muted)
    } else if line.contains("recovers") || line.contains("restoring") {
        Style::default().fg(theme::theme().success)
    } else if line.contains("drops to 0 HP") {
        Style::default().fg(theme::theme().danger).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::theme().text_primary)
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

    #[test]
    fn timeline_marks_current_actor() {
        let t = compact_timeline(&["a".into(), "b".into(), "c".into()], "b");
        assert_eq!(t, "a -> [b] -> c");
    }
}
