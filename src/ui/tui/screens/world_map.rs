use crate::app::App;
use crate::game::world::map::Tile;
use crate::ui::tui::theme::{self, progress_bar, theme};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::vertical([
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Min(7),
        Constraint::Length(4),
    ])
    .split(area);

    let t = theme();

    // Header with region info
    let room_name = app
        .current_room()
        .map(|r| r.name.clone())
        .unwrap_or_else(|| app.current_room_id.clone());
    let region_weather = if app.region.weather.is_empty() || app.region.weather == "none" {
        String::new()
    } else {
        format!(" [{}]", app.region.weather)
    };
    let header = Paragraph::new(vec![
        Line::from(if app.region.ambient.is_empty() {
            format!(
                "Region: {} ({}){}",
                app.region.name, app.region.slug, region_weather
            )
        } else {
            format!(
                "Region: {} ({}) [{}]{}",
                app.region.name, app.region.slug, app.region.ambient, region_weather
            )
        })
        .style(Style::default().fg(t.text_primary)),
        Line::from(format!("Room: {} ({}) | Turn: {}", room_name, app.current_room_id, app.turn))
            .style(Style::default().fg(t.text_muted)),
    ])
    .block(
        Block::default()
            .title("World")
            .borders(Borders::ALL)
            .style(Style::default().fg(t.panel_border)),
    );
    frame.render_widget(header, chunks[0]);

    // Player stats bar with progress
    let stats_lines = vec![
        Line::from(vec![
            Span::raw(format!("{} ", theme::icon("health"))),
            Span::raw(format!("{} ", app.player.name)),
            Span::raw(format!("HP ")),
            Span::styled(
                progress_bar(app.player.current_hp, app.player.max_hp, 10),
                Style::default().fg(theme::health_color(
                    app.player.current_hp,
                    app.player.max_hp,
                )),
            ),
            Span::raw(format!(" {}/{} ", app.player.current_hp, app.player.max_hp)),
            Span::raw(format!("Gold:{} ", app.player.gold)),
        ]),
        Line::from(vec![
            Span::raw(format!("Lv{} ", app.player.level)),
            Span::raw("XP "),
            Span::styled(
                progress_bar(app.player.xp as i32, 6500, 10),
                Style::default().fg(t.xp),
            ),
            Span::raw(format!(" {} ", app.player.xp)),
            Span::raw(format!("Skill Pts:{} ", app.player.skill_points)),
        ]),
    ];
    let stats = Paragraph::new(stats_lines)
        .style(Style::default().fg(t.text_primary))
        .block(
            Block::default()
                .title("Status")
                .borders(Borders::ALL)
                .style(Style::default().fg(t.panel_border)),
        );
    frame.render_widget(stats, chunks[1]);

    // Map with semantic coloring
    let map_text = if let Some(room) = app.region.room(&app.current_room_id) {
        let mut rows = Vec::new();
        for r in 0..room.height() {
            let mut line = Vec::new();
            for c in 0..room.width() {
                let pos = (c, r);
                if pos == app.player_pos {
                    line.push(Span::styled(
                        "@",
                        Style::default().fg(t.player).add_modifier(Modifier::BOLD),
                    ));
                } else if let Some(_npc) = room.npc_at(c, r) {
                    line.push(Span::styled("n", Style::default().fg(t.npc)));
                } else if let Some(_item) = room.items.iter().find(|i| i.position == [c, r]) {
                    line.push(Span::styled("c", Style::default().fg(t.item)));
                } else if let Some(trigger) = room.trigger_at(c, r) {
                    let (glyph, style) = render_trigger_tile(trigger, &t);
                    line.push(Span::styled(glyph.to_string(), style));
                } else {
                    let (glyph, style) = render_map_tile(room.grid.get(c, r));
                    line.push(Span::styled(glyph.to_string(), style));
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
                .style(Style::default().fg(t.panel_border)),
        ),
        chunks[2],
    );

    // Controls footer
    let feedback = app.get_feedback();
    let footer_lines = if app.show_help {
        vec![
            Line::from("LEGEND: @=player  n=NPC  !=trigger  /=door  >=stairs down  <=stairs up"),
            Line::from("       c=chest  +=door closed  ~=deep water  ,=shallow water"),
            Line::from("       HP=health  MP=mana  XP=experience  $[=gold"),
            Line::from("Press ? to close help. Move: arrows/hjkl Interact: Enter"),
        ]
    } else if let Some(msg) = feedback {
        vec![
            Line::from(msg).style(Style::default().fg(t.warning)),
            Line::from("Move: arrows/hjkl  Interact: Enter  ?: help"),
            Line::from("a combat  i inventory  s spellbook  n journal"),
            Line::from("p save  o load  b toggle sound  q quit"),
        ]
    } else {
        vec![
            Line::from("Move: arrows/hjkl  Interact: Enter  ?: help"),
            Line::from("a combat  i inventory  s spellbook  n journal"),
            Line::from("p save  o load  b toggle sound  q quit"),
        ]
    };
    frame.render_widget(
        Paragraph::new(footer_lines)
            .style(Style::default().fg(t.text_muted))
            .block(
                Block::default()
                    .title(if app.show_help { "Legend" } else { "Controls" })
                    .borders(Borders::ALL)
                    .style(Style::default().fg(t.panel_border)),
            ),
        chunks[3],
    );
}

use crate::data::types::TriggerKind;
use ratatui::text::Span;

fn render_trigger_tile(trigger: &crate::data::types::TriggerDef, t: &crate::ui::tui::theme::Theme) -> (char, Style) {
    match trigger.kind {
        TriggerKind::Travel => ('>', Style::default().fg(t.success)),
        TriggerKind::Dialog => ('!', Style::default().fg(t.warning)),
        TriggerKind::Encounter => ('!', Style::default().fg(t.danger)),
        _ => ('!', Style::default().fg(t.warning)),
    }
}

fn render_map_tile(tile: Option<Tile>) -> (char, Style) {
    let t = theme();
    match tile {
        Some(Tile::Wall) => (
            t.wall().chars().next().unwrap_or('#'),
            Style::default().fg(t.wall),
        ),
        Some(Tile::Floor) => (
            t.floor().chars().next().unwrap_or('.'),
            Style::default().fg(t.floor),
        ),
        Some(Tile::NpcSpawn) => ('n', Style::default().fg(t.npc)),
        Some(Tile::DoorOpen) => ('/', Style::default().fg(t.connection)),
        Some(Tile::DoorClosed) => ('+', Style::default().fg(t.connection)),
        Some(Tile::DeepWater) => ('~', Style::default().fg(t.mana)),
        Some(Tile::ShallowWater) => ('.', Style::default().fg(t.mana)),
        Some(Tile::StairsUp) => ('<', Style::default().fg(t.success)),
        Some(Tile::StairsDown) => ('>', Style::default().fg(t.success)),
        Some(Tile::Chest) => ('c', Style::default().fg(t.item)),
        Some(Tile::Trigger) => ('!', Style::default().fg(t.warning)),
        _ => (' ', Style::default()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn npc_spawn_uses_npc_color() {
        let (glyph, style) = render_map_tile(Some(Tile::NpcSpawn));
        assert_eq!(glyph, 'n');
    }
}
