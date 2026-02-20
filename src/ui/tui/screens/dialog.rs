use crate::app::{App, AppState};
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(5), Constraint::Length(7)])
        .split(area);

    let Some(ctx) = (match &app.state {
        AppState::Dialog(ctx) => Some(ctx),
        _ => None,
    }) else {
        frame.render_widget(
            Paragraph::new("No dialog active.")
                .block(Block::default().title("Dialog").borders(Borders::ALL)),
            area,
        );
        return;
    };

    frame.render_widget(
        Paragraph::new(ctx.npc_name.as_str())
            .alignment(Alignment::Center)
            .block(Block::default().title("NPC").borders(Borders::ALL)),
        chunks[0],
    );

    frame.render_widget(
        Paragraph::new(ctx.resolved.text.as_str())
            .wrap(Wrap { trim: true })
            .block(Block::default().title("Dialog").borders(Borders::ALL)),
        chunks[1],
    );

    let mut lines = Vec::new();
    for (idx, choice) in ctx.resolved.choices.iter().enumerate().take(9) {
        lines.push(Line::from(format!("[{}] {}", idx + 1, choice.text)));
    }
    if lines.is_empty() {
        lines.push(Line::from("No available choices."));
    }
    lines.push(Line::from("Esc: close dialog"));

    frame.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .block(Block::default().title("Choices").borders(Borders::ALL)),
        chunks[2],
    );
}
