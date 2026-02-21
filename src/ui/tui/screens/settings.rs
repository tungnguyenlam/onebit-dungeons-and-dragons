use crate::{app::App, ui::tui::theme};
use ratatui::{
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::vertical([Constraint::Min(0)])
        .horizontal_margin(area.width / 4)
        .vertical_margin(area.height / 4)
        .split(area);

    let mut lines = vec![
        Line::from("Difficulty & Accessibility").style(theme::emph_style()),
        Line::from(""),
    ];

    let options = vec![
        format!("Enemy HP Multiplier: x{:.1}", app.settings.enemy_hp_multiplier),
        format!("Player Damage Multiplier: x{:.1}", app.settings.player_damage_multiplier),
        format!("Reduced Motion: {}", if app.settings.reduced_motion { "On" } else { "Off" }),
    ];

    for (i, opt) in options.iter().enumerate() {
        let prefix = if i == app.settings_ui.selected { "> " } else { "  " };
        let style = if i == app.settings_ui.selected {
            theme::accent_style()
        } else {
            ratatui::style::Style::default()
        };
        lines.push(Line::from(format!("{}{}", prefix, opt)).style(style));
    }

    lines.push(Line::from(""));
    lines.push(Line::from("Use ↑/↓ to select, ←/→ to adjust values.").style(theme::muted_style()));
    lines.push(Line::from("Press Esc to return.").style(theme::muted_style()));

    frame.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .block(Block::default().title("Settings").borders(Borders::ALL).style(theme::panel_style_focused())),
        chunks[0],
    );
}
