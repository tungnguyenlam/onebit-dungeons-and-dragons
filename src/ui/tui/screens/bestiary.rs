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

    let mut lines = vec![Line::from("Discovered monsters and known stat blocks:")];
    let mut discovered = app.world_state.discovered_monsters();
    discovered.sort();

    if discovered.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from("No entries yet. Defeat monsters to populate the bestiary."));
    } else {
        for monster_id in discovered {
            if let Some(monster) = app.monster_defs.get(&monster_id) {
                let kills = app.world_state.monster_kill_count(&monster_id);
                lines.push(Line::from(""));
                lines.push(Line::from(format!("- {} ({})", monster.name, monster.id)));
                lines.push(Line::from(format!(
                    "  HP {} | AC {} | XP {} | Kills {}",
                    monster.hp, monster.ac, monster.xp, kills
                )));
                if monster.resistances.is_empty() {
                    lines.push(Line::from("  Resistances: none"));
                } else {
                    lines.push(Line::from(format!(
                        "  Resistances: {}",
                        monster.resistances.join(", ")
                    )));
                }
                lines.push(Line::from(format!(
                    "  {} | {}",
                    monster.size, monster.monster_type
                )));
            }
        }
    }

    frame.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .block(Block::default().title("Bestiary").borders(Borders::ALL)),
        chunks[0],
    );
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("Esc/backspace: return to Journal"),
            Line::from("Tip: press 'y' from world/journal to open Lore Library"),
        ])
        .block(Block::default().title("Controls").borders(Borders::ALL)),
        chunks[1],
    );
}
