use crate::app::{App, AppState};
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
        Constraint::Length(3),
        Constraint::Min(8),
        Constraint::Length(8),
        Constraint::Length(6),
    ])
    .split(area);

    let Some(ctx) = (match &app.state {
        AppState::Combat(ctx) => Some(ctx),
        _ => None,
    }) else {
        let p = Paragraph::new("Combat screen requested outside combat state.")
            .block(Block::default().title("Combat").borders(Borders::ALL));
        frame.render_widget(p, area);
        return;
    };

    // Turn order banner.
    let current_id = ctx.state.current_combatant_id().unwrap_or("?");
    let banner_text = ctx
        .state
        .turn_queue
        .iter()
        .map(|id| {
            if id == current_id {
                format!("[{id}]")
            } else {
                id.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" -> ");
    let active = ctx
        .state
        .current_combatant()
        .map(|c| format!("{} ({})", c.name, if c.is_player { "PLAYER" } else { "ENEMY" }))
        .unwrap_or_else(|| "Unknown".into());
    let turn_banner = Paragraph::new(Line::from(format!(
        "Round {} | Active: {} | Turn Order: {banner_text}",
        ctx.state.round, active
    )))
    .block(Block::default().title("Initiative").borders(Borders::ALL));
    frame.render_widget(turn_banner, chunks[0]);

    // Battlefield placeholder.
    let battlefield = Paragraph::new(
        "Battlefield view is placeholder for now.\n\nControls:\n  - 'a' attack\n  - '.' wait/end turn\n  - Esc leave combat",
    )
    .wrap(Wrap { trim: true })
    .block(Block::default().title("Battlefield").borders(Borders::ALL));
    frame.render_widget(battlefield, chunks[1]);

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
                let mut labels: Vec<String> = c.conditions.iter().map(|cond| cond.name()).collect();
                labels.sort();
                labels.join(",")
            };
            Line::from(format!(
                "{marker} {:10} HP {:>2}/{:<2} AC {:>2} Slots {action}{bonus}{reaction} Move {:>2} Cond {}",
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
        .block(Block::default().title("Combatants").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(hud, chunks[2]);

    // Log panel (latest lines only).
    let log_height = chunks[3].height.saturating_sub(2) as usize;
    let start = ctx.log.len().saturating_sub(log_height);
    let log_lines: Vec<Line<'_>> = ctx.log[start..]
        .iter()
        .map(|line| Line::from(line.clone()))
        .collect();
    let log = Paragraph::new(log_lines)
        .block(Block::default().title("Combat Log").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(log, chunks[3]);
}
