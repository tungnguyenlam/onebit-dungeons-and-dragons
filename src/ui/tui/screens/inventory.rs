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
    let chunks = Layout::vertical([Constraint::Length(7), Constraint::Min(10)]).split(area);

    let controls = Paragraph::new(vec![
        Line::from(format!("[1] {} Toggle longsword", theme::icon("warning"))),
        Line::from("[2] Toggle leather armor"),
        Line::from("[3] Toggle shield"),
        Line::from("[4] Use healing potion (+8 HP)"),
        Line::from("Esc: close"),
    ])
    .style(theme::muted_style())
    .block(
        Block::default()
            .title("Inventory Actions")
            .borders(Borders::ALL)
            .style(theme::panel_style()),
    );
    frame.render_widget(controls, chunks[0]);

    let mut lines = Vec::new();
    lines.push(Line::from(format!(
        "Equipped: main={:?}, off={:?}, armor={:?}",
        app.player.equipment.main_hand, app.player.equipment.off_hand, app.player.equipment.armor
    )));
    lines.push(Line::from(""));
    for item in &app.player.inventory.items {
        let marker = if item.equipped { "*" } else { " " };
        lines.push(Line::from(format!(
            "{marker} {:16} x{}",
            item.item_id, item.quantity
        )));
    }
    if lines.len() <= 2 {
        lines.push(Line::from("(no items)"));
    }

    let list = Paragraph::new(lines).wrap(Wrap { trim: true }).block(
        Block::default()
            .title(format!("Items {}", theme::icon("quest")))
            .borders(Borders::ALL)
            .style(theme::panel_style()),
    );
    frame.render_widget(list, chunks[1]);
}
