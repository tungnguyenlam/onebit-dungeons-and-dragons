use crate::app::App;
use crate::app::navigation::world_map_util::build_region_overview;
use crate::game::world::map::Tile;
use crate::game::world::{fov, weather::WeatherType};
use crate::ui::tui::theme::{self, progress_bar, theme};
use crate::ui::tui::widgets::{connected_room_lines, exit_lines, room_list_lines};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let t = theme();
    let feedback = app.get_feedback();
    let weather_effect = match WeatherType::from_region_tag(&app.region.weather) {
        WeatherType::Rain => "Weather: Rain (-2 ranged accuracy, fire attacks disadvantaged)",
        WeatherType::Fog => "Weather: Fog (reduced FOV, ranged attacks disadvantaged)",
        WeatherType::Ash => "Weather: Ash (periodic coughing/poisoned pressure)",
        WeatherType::Snow => "Weather: Snow (slippery footing)",
        WeatherType::Clear => "Weather: Clear",
    };
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
            Line::from(weather_effect),
            Line::from("Move: arrows/hjkl  Interact: Enter  ?: help"),
            Line::from("a combat  i inventory  c crafting  s spellbook"),
            Line::from("n journal  v bestiary  y lore"),
            Line::from("p save  o load  b toggle sound  q quit"),
        ]
    } else {
        vec![
            Line::from(weather_effect),
            Line::from("Move: arrows/hjkl  Interact: Enter  ?: help"),
            Line::from("a combat  i inventory  c crafting  s spellbook"),
            Line::from("n journal  v bestiary  y lore"),
            Line::from("p save  o load  b toggle sound  q quit"),
        ]
    };
    let footer_height = (footer_lines.len() + 2) as u16;
    let chunks = Layout::vertical([
        Constraint::Length(5),
        Constraint::Length(4),
        Constraint::Min(7),
        Constraint::Length(footer_height),
    ])
    .split(area);

    // Header with region info
    let room_name = app
        .current_room()
        .map(|r| r.name.clone())
        .unwrap_or_else(|| app.current_room_id.clone());
    let room_landmark = app
        .current_room()
        .map(|r| r.landmark.clone())
        .unwrap_or_default();
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
        Line::from(format!(
            "Room: {} ({}) | Turn: {}",
            room_name, app.current_room_id, app.turn
        ))
        .style(Style::default().fg(t.text_muted)),
        Line::from(if room_landmark.is_empty() {
            "Landmark: (none)".to_string()
        } else {
            format!("Landmark: {room_landmark}")
        })
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
            Span::raw(format!("Lv{} ", app.player.total_level)),
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
        let weather = WeatherType::from_region_tag(&app.region.weather);
        let visible = if matches!(weather, WeatherType::Fog) {
            Some(fov::compute(
                (app.player_pos.0 as i32, app.player_pos.1 as i32),
                weather.fov_radius(),
                &room.grid,
            ))
        } else {
            None
        };
        let mut rows = Vec::new();
        for r in 0..room.height() {
            let mut line = Vec::new();
            for c in 0..room.width() {
                let pos = (c, r);
                if let Some(visible) = &visible {
                    let is_visible = visible.contains(&(c as i32, r as i32));
                    if !is_visible {
                        line.push(Span::raw(" "));
                        continue;
                    }
                }
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

    let map_chunks = Layout::horizontal([Constraint::Min(28), Constraint::Length(36)]).split(chunks[2]);

    frame.render_widget(
        Paragraph::new(map_text).block(
            Block::default()
                .title("Local Map")
                .borders(Borders::ALL)
                .style(Style::default().fg(t.panel_border)),
        ),
        map_chunks[0],
    );

    let overview = build_region_overview(&app.region, &app.current_room_id, &app.world_state);
    let right_lines = room_list_lines(&overview)
        .into_iter()
        .chain(std::iter::once(Line::from("")))
        .chain(connected_room_lines(&overview))
        .chain(std::iter::once(Line::from("")))
        .chain(exit_lines(&overview))
        .collect::<Vec<_>>();
    frame.render_widget(
        Paragraph::new(right_lines)
            .block(
                Block::default()
                    .title("World Map Widget")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(t.panel_border)),
            )
            .wrap(ratatui::widgets::Wrap { trim: true }),
        map_chunks[1],
    );

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

fn render_trigger_tile(
    trigger: &crate::data::types::TriggerDef,
    t: &crate::ui::tui::theme::Theme,
) -> (char, Style) {
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
