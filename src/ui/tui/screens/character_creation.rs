use crate::app::App;
use ratatui::{
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let ui = &app.char_creation_ui;

    let fields = [
        format!("Name: {}", ui.name),
        format!("Class: {}", ui.class_options[ui.class_index]),
        format!("Race: {}", ui.race_options[ui.race_index]),
        "Start Adventure".to_string(),
    ];

    let mut lines = Vec::new();
    for (idx, field) in fields.iter().enumerate() {
        let marker = if idx == ui.selected { ">" } else { " " };
        lines.push(
            Line::from(format!("{marker} {field}")).style(if idx == ui.selected {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            }),
        );
    }
    lines.push(Line::from(""));
    lines.push(Line::from("↑/↓ choose field  ←/→ cycle class/race"));
    lines.push(Line::from("When Name is selected, type letters/numbers, Backspace to delete"));
    lines.push(Line::from("Enter on Start Adventure to continue"));
    lines.push(Line::from("Esc to return"));

    frame.render_widget(
        Paragraph::new(lines).block(
            Block::default()
                .title("Character Creation")
                .borders(Borders::ALL),
        ),
        area,
    );
}
