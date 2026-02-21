use crate::app::App;
use crate::ui::tui::theme;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(8),
        Constraint::Length(4),
    ])
    .split(area);

    let title = Paragraph::new("OneBit Dungeons & Dragons")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("Main Menu")
                .borders(Borders::ALL)
                .style(theme::panel_style()),
        );
    frame.render_widget(title, chunks[0]);

    let items = ["New Game", "Continue", "Load Save", "Quit (press q)"];
    let mut lines = Vec::new();
    for (idx, item) in items.iter().enumerate() {
        let marker = if idx == app.menu_ui.selected {
            ">"
        } else {
            " "
        };
        lines.push(
            Line::from(format!("{marker} {item}")).style(if idx == app.menu_ui.selected {
                theme::accent_style()
            } else {
                Style::default().fg(theme::theme().text_primary)
            }),
        );
    }
    frame.render_widget(
        Paragraph::new(lines).block(
            Block::default()
                .title("Options")
                .borders(Borders::ALL)
                .style(theme::panel_style()),
        ),
        chunks[1],
    );

    frame.render_widget(
        Paragraph::new(format!(
            "{} ↑/↓ select  Enter confirm  q quit\nb toggle sound  p save  o load  Tier: {:?}",
            theme::icon("warning"),
            theme::terminal_tier()
        ))
        .style(theme::muted_style())
        .block(
            Block::default()
                .title("Controls")
                .borders(Borders::ALL)
                .style(theme::panel_style()),
        ),
        chunks[2],
    );
}
