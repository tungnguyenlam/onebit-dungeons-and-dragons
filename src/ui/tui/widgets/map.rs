use crate::app::navigation::world_map_util::RegionOverview;
use crate::ui::tui::theme::theme;
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

pub fn room_list_lines(overview: &RegionOverview) -> Vec<Line<'static>> {
    let mut lines = vec![Line::from(format!(
        "{} ({})",
        overview.region_name, overview.region_slug
    ))];
    lines.push(Line::from(format!("Current: {}", overview.current_room)));
    if !overview.current_landmark.is_empty() {
        lines.push(Line::from(format!("Landmark: {}", overview.current_landmark)));
    }
    lines.push(Line::from(""));
    lines.push(Line::from("Rooms:"));
    for room in &overview.room_ids {
        let marker = if room == &overview.current_room { ">" } else { " " };
        lines.push(Line::from(format!("{marker} {room}")));
    }
    lines
}

pub fn connected_room_lines(overview: &RegionOverview) -> Vec<Line<'static>> {
    if overview.connected_rooms.is_empty() {
        return vec![Line::from("Paths: none")];
    }
    let mut lines = vec![Line::from("Paths:")];
    for room_id in &overview.connected_rooms {
        lines.push(Line::from(format!("- {room_id}")));
    }
    lines
}

pub fn exit_lines(overview: &RegionOverview) -> Vec<Line<'static>> {
    if overview.exits.is_empty() {
        return vec![Line::from("No regional exits from this room.")];
    }

    let t = theme();
    let mut lines = vec![Line::from("Exits:")];
    for exit in &overview.exits {
        let status = if exit.available { "open" } else { "locked" };
        let style = if exit.available {
            Style::default().fg(t.success)
        } else {
            Style::default().fg(t.warning)
        };
        lines.push(Line::from(vec![
            Span::raw(format!("- {} -> {}:{} ", exit.label, exit.to_region, exit.to_room)),
            Span::styled(format!("[{status}]"), style.add_modifier(Modifier::BOLD)),
        ]));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::navigation::world_map_util::ExitView;

    #[test]
    fn lines_include_current_room_marker() {
        let overview = RegionOverview {
            region_name: "R".into(),
            region_slug: "r".into(),
            current_room: "b".into(),
            room_ids: vec!["a".into(), "b".into()],
            current_landmark: "Beacon".into(),
            connected_rooms: vec!["a".into()],
            exits: vec![],
        };
        let lines = room_list_lines(&overview)
            .into_iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>();
        assert!(lines.iter().any(|l| l.contains("> b")));
        assert!(lines.iter().any(|l| l.contains("Landmark: Beacon")));
    }

    #[test]
    fn exit_lines_show_status() {
        let overview = RegionOverview {
            region_name: "R".into(),
            region_slug: "r".into(),
            current_room: "b".into(),
            room_ids: vec!["a".into(), "b".into()],
            current_landmark: String::new(),
            connected_rooms: vec![],
            exits: vec![ExitView {
                to_region: "x".into(),
                to_room: "y".into(),
                label: "Path".into(),
                available: false,
            }],
        };
        let lines = exit_lines(&overview)
            .into_iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>();
        assert!(lines.iter().any(|l| l.contains("[locked]")));
    }
}
