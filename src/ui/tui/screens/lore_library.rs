use crate::app::App;
use ratatui::{
    layout::Constraint,
    layout::Layout,
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::vertical([Constraint::Min(6), Constraint::Length(3)]).split(area);

    let mut lines = vec![Line::from("Collected lore entries:")];
    let mut discovered = app.world_state.discovered_lore();
    discovered.sort();

    if discovered.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from("No entries yet. Inspect lore triggers in rooms to unlock entries."));
    } else {
        for lore_id in discovered {
            if let Some(lore) = app.lore_defs.get(&lore_id) {
                lines.push(Line::from(""));
                lines.push(Line::from(format!("- {} ({})", lore.title, lore.id)));
                if lore.tags.is_empty() {
                    lines.push(Line::from("  Tags: uncategorized"));
                } else {
                    lines.push(Line::from(format!("  Tags: {}", lore.tags.join(", "))));
                }
                lines.push(Line::from(format!("  {}", lore.text)));
            }
        }
    }

    frame.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .block(Block::default().title("Lore Library").borders(Borders::ALL)),
        chunks[0],
    );
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("Esc/backspace: return to Journal"),
            Line::from("Tip: press 'v' from world/journal to open Bestiary"),
        ])
        .block(Block::default().title("Controls").borders(Borders::ALL)),
        chunks[1],
    );
}
