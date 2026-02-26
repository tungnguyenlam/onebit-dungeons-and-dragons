use crate::{
    app::{App, FocusedPane},
    game::story::journal::Category as JournalCategory,
    ui::tui::theme,
};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::horizontal([Constraint::Length(32), Constraint::Min(20)]).split(area);

    let category = app.journal_ui.category;
    let entries = app.journal.entries_by_category(category);

    let mut left = vec![Line::from(format!(
        "Category: {}",
        category_label(category)
    ))];
    left.push(Line::from("←/→ switch  ↑/↓ select"));
    left.push(Line::from("v bestiary  y lore library"));
    left.push(Line::from("Esc close"));
    left.push(Line::from(""));
    for (idx, entry) in entries.iter().enumerate() {
        let marker = if idx == app.journal_ui.selected {
            ">"
        } else {
            " "
        };
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

    let left_block = Block::default()
        .title("Journal")
        .borders(Borders::ALL)
        .style(if app.focused_pane == FocusedPane::Main {
            theme::panel_style_focused()
        } else {
            theme::panel_style()
        });

    let right_block = Block::default().title("Entry").borders(Borders::ALL).style(
        if app.focused_pane == FocusedPane::Side {
            theme::panel_style_focused()
        } else {
            theme::panel_style()
        },
    );

    let detail_lines = right.len() as u16;
    let right_p = Paragraph::new(right)
        .wrap(Wrap { trim: true })
        .scroll((app.journal_ui.detail_scroll, 0))
        .block(right_block);

    frame.render_widget(
        Paragraph::new(left)
            .wrap(Wrap { trim: true })
            .block(left_block),
        chunks[0],
    );
    frame.render_widget(right_p, chunks[1]);

    if app.focused_pane == FocusedPane::Side && detail_lines > chunks[1].height.saturating_sub(2) {
        let mut state = ScrollbarState::default()
            .content_length(detail_lines as usize)
            .position(app.journal_ui.detail_scroll as usize);
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            chunks[1],
            &mut state,
        );
    }
}

fn category_label(c: JournalCategory) -> &'static str {
    match c {
        JournalCategory::Quest => "Quest",
        JournalCategory::Lore => "Lore",
        JournalCategory::World => "World",
        JournalCategory::Combat => "Combat",
        JournalCategory::Dialog => "Dialog",
        JournalCategory::System => "System",
    }
}
