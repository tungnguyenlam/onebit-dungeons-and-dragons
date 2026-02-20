use crate::app::App;
use ratatui::{
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let summary = format!(
        "Game Over\n\nTurn: {}\nPlayer: {}\nHP: {}/{}\n\nPress q to quit or o to load save.",
        app.turn, app.player.name, app.player.current_hp, app.player.max_hp
    );
    frame.render_widget(
        Paragraph::new(summary)
            .alignment(Alignment::Center)
            .block(Block::default().title("Game Over").borders(Borders::ALL)),
        area,
    );
}
