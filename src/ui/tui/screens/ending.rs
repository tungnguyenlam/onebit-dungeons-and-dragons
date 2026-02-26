use crate::app::App;
use crate::game::story::ending;
use ratatui::{
    layout::Constraint,
    layout::Layout,
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::vertical([Constraint::Length(5), Constraint::Min(8), Constraint::Length(3)])
        .split(area);

    let ending = ending::calculate(&app.world_state);
    let header = Paragraph::new(vec![
        Line::from("Finale").style(Style::default().add_modifier(Modifier::BOLD)),
        Line::from(format!("{} (Score: {})", ending.title, ending.score)),
        Line::from(""),
        Line::from(ending.body),
    ])
    .wrap(Wrap { trim: true })
    .block(Block::default().title("Ending").borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);

    let credits = ending::credits_lines()
        .into_iter()
        .map(Line::from)
        .collect::<Vec<_>>();
    let credits_widget = Paragraph::new(credits)
        .scroll((app.ending_scroll, 0))
        .wrap(Wrap { trim: true })
        .block(Block::default().title("Credits").borders(Borders::ALL));
    frame.render_widget(credits_widget, chunks[1]);

    frame.render_widget(
        Paragraph::new(vec![
            Line::from("Up/Down: scroll credits"),
            Line::from("Enter/Esc: return to menu (New Game+ unlocked)"),
        ])
        .block(Block::default().title("Controls").borders(Borders::ALL)),
        chunks[2],
    );
}
