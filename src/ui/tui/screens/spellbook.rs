use crate::app::App;
use crate::ui::tui::theme;
use ratatui::{
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::vertical([Constraint::Length(5), Constraint::Min(8)]).split(area);

    let slots_line = format!(
        "{} Slots L1: {}/{}",
        theme::icon("magic"),
        app.player.spell_slots[0], app.player.spell_slots_max[0]
    );
    let header = Paragraph::new(vec![
        Line::from(slots_line),
        Line::from("Cast with [1]-[9], Esc to close"),
    ])
    .style(theme::muted_style())
    .block(
        Block::default()
            .title("Spellbook")
            .borders(Borders::ALL)
            .style(theme::panel_style()),
    );
    frame.render_widget(header, chunks[0]);

    let mut lines = Vec::new();
    for (idx, spell_id) in app.known_spells.iter().enumerate().take(9) {
        if let Some(spell) = app.spell_defs.get(spell_id) {
            lines.push(Line::from(format!(
                "[{}] {} (lvl {})",
                idx + 1,
                spell.name,
                spell.level
            )));
        } else {
            lines.push(Line::from(format!("[{}] {}", idx + 1, spell_id)));
        }
    }
    if lines.is_empty() {
        lines.push(Line::from("(no known spells)"));
    }

    let body = Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title("Known Spells")
                .borders(Borders::ALL)
                .style(theme::panel_style()),
        );
    frame.render_widget(body, chunks[1]);
}
