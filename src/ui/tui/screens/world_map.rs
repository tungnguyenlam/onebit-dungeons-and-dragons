use crate::app::App;
use crate::game::world::map::Tile;
use crate::ui::tui::theme;
use ratatui::{
    layout::{Constraint, Layout},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::vertical([
        Constraint::Length(5),
        Constraint::Min(7),
        Constraint::Length(4),
    ])
    .split(area);

    let header = Paragraph::new(vec![
        Line::from(format!("Region: {} ({})", app.region.name, app.region.slug))
            .style(theme::accent_style()),
        Line::from(format!("Room: {}", app.current_room_id)),
        Line::from(format!(
            "Player: {} {} at ({}, {})",
            theme::icon("health"),
            app.player.name,
            app.player_pos.0,
            app.player_pos.1
        )),
    ])
    .style(Style::default().fg(theme::theme().text_primary))
    .block(
        Block::default()
            .title("World")
            .borders(Borders::ALL)
            .style(theme::panel_style()),
    );
    frame.render_widget(header, chunks[0]);

    let map_text = if let Some(room) = app.region.room(&app.current_room_id) {
        let mut rows = Vec::new();
        for r in 0..room.height() {
            let mut line = String::new();
            for c in 0..room.width() {
                if (c, r) == app.player_pos {
                    line.push('@');
                } else {
                    line.push(room.grid.get(c, r).map(render_map_glyph).unwrap_or(' '));
                }
            }
            rows.push(Line::from(line));
        }
        rows
    } else {
        vec![Line::from("No active room.")]
    };

    frame.render_widget(
        Paragraph::new(map_text).block(
            Block::default()
                .title("Map")
                .borders(Borders::ALL)
                .style(theme::panel_style()),
        ),
        chunks[1],
    );

    frame.render_widget(
        Paragraph::new(vec![
            Line::from("Move: arrows/hjkl  Interact: Enter"),
            Line::from("a combat  i inventory  s spellbook  n journal"),
            Line::from("p save  o load  b toggle sound  q quit"),
        ])
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

fn render_map_glyph(tile: Tile) -> char {
    match tile {
        // Avoid visual confusion with player marker '@'.
        Tile::NpcSpawn => 'n',
        _ => tile.glyph(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn npc_spawn_is_not_rendered_as_player_glyph() {
        assert_eq!(render_map_glyph(Tile::NpcSpawn), 'n');
        assert_eq!(render_map_glyph(Tile::Floor), '.');
    }
}
