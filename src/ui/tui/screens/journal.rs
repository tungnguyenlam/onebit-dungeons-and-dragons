use crate::{
    app::App,
    game::story::journal::Category as JournalCategory,
};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::horizontal([Constraint::Length(32), Constraint::Min(20)]).split(area);

    let category = app.journal_ui.category;
    let entries = app.journal.entries_by_category(category);

    let mut left = vec![Line::from(format!("Category: {}", category_label(category)))];
    left.push(Line::from("←/→ switch  ↑/↓ select"));
    left.push(Line::from("Esc close"));
    left.push(Line::from(""));
    for (idx, entry) in entries.iter().enumerate() {
        let marker = if idx == app.journal_ui.selected { ">" } else { " " };
        left.push(Line::from(format!("{marker} {}", entry.title)));
    }

    let selected = entries
        .get(app.journal_ui.selected)
        .or_else(|| entries.first());
    let right = if let Some(entry) = selected {
        vec![
            Line::from(format!("[{}] {}", entry.timestamp, entry.title))
                .style(Style::default().add_modifier(Modifier::BOLD)),
            Line::from(""),
            Line::from(entry.body.clone()),
        ]
    } else {
        vec![Line::from("No journal entries in this category.")]
    };

    frame.render_widget(
        Paragraph::new(left)
            .wrap(Wrap { trim: true })
            .block(Block::default().title("Journal").borders(Borders::ALL)),
        chunks[0],
    );
    frame.render_widget(
        Paragraph::new(right)
            .wrap(Wrap { trim: true })
            .block(Block::default().title("Entry").borders(Borders::ALL)),
        chunks[1],
    );
}

fn category_label(c: JournalCategory) -> &'static str {
    match c {
        JournalCategory::Quest  => "Quest",
        JournalCategory::Lore   => "Lore",
        JournalCategory::World  => "World",
        JournalCategory::Combat => "Combat",
        JournalCategory::Dialog => "Dialog",
        JournalCategory::System => "System",
    }
}
