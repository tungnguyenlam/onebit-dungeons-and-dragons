use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(8), Constraint::Length(4)])
        .split(area);

    let title = Paragraph::new("OneBit Dungeons & Dragons")
        .alignment(Alignment::Center)
        .block(Block::default().title("Main Menu").borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    let items = ["New Game", "Continue", "Load Save", "Quit (press q)"];
    let mut lines = Vec::new();
    for (idx, item) in items.iter().enumerate() {
        let marker = if idx == app.menu_ui.selected { ">" } else { " " };
        lines.push(
            Line::from(format!("{marker} {item}")).style(if idx == app.menu_ui.selected {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            }),
        );
    }
    frame.render_widget(
        Paragraph::new(lines).block(Block::default().title("Options").borders(Borders::ALL)),
        chunks[1],
    );

    frame.render_widget(
        Paragraph::new("↑/↓ select  Enter confirm  q quit\nb toggle sound  p save  o load")
            .block(Block::default().title("Controls").borders(Borders::ALL)),
        chunks[2],
    );
}
