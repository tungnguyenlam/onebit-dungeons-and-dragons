use crate::app::App;
use crate::ui::tui::theme::{self, progress_bar, theme};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::vertical([Constraint::Length(7), Constraint::Min(10)]).split(area);

    let t = theme();

    // Player stats with progress bars
    let stats = Paragraph::new(vec![
        Line::from(vec![
            ratatui::text::Span::raw(format!("{} ", theme::icon("health"))),
            ratatui::text::Span::raw(format!("HP ")),
            ratatui::text::Span::styled(
                progress_bar(app.player.current_hp, app.player.max_hp, 8),
                Style::default().fg(theme::health_color(
                    app.player.current_hp,
                    app.player.max_hp,
                )),
            ),
            ratatui::text::Span::raw(format!(" {}/{}", app.player.current_hp, app.player.max_hp)),
        ]),
        Line::from(vec![
            ratatui::text::Span::raw(format!("AC {}", {
                let armor_id = app.player.equipment.armor.as_deref();
                let armor_def = armor_id.and_then(|id| app.item_defs.get(id));
                let armor_tuple =
                    armor_def.and_then(|d| d.armor.as_ref().map(|a| (a.base_ac, &a.armor_type)));
                let shield_equipped = app
                    .player
                    .equipment
                    .off_hand
                    .as_deref()
                    .and_then(|id| app.item_defs.get(id))
                    .map(|d| {
                        d.armor
                            .as_ref()
                            .map(|a| a.armor_type == crate::data::types::ArmorType::Shield)
                            .unwrap_or(false)
                    })
                    .unwrap_or(false);
                crate::game::items::armor::armor_class(
                    armor_tuple,
                    shield_equipped,
                    app.player.scores.dex_mod(),
                )
            })),
            ratatui::text::Span::raw("  "),
            ratatui::text::Span::raw(format!("Level {}", app.player.total_level)),
            ratatui::text::Span::raw("  "),
            ratatui::text::Span::raw("XP "),
            ratatui::text::Span::styled(format!("{}", app.player.xp), Style::default().fg(t.xp)),
        ]),
    ])
    .style(Style::default().fg(t.text_primary))
    .block(
        Block::default()
            .title("Character Stats")
            .borders(Borders::ALL)
            .style(Style::default().fg(t.panel_border)),
    );
    frame.render_widget(stats, chunks[0]);

    // Equipment and inventory
    let mut lines = Vec::new();

    // Equipment slots
    lines.push(Line::from(vec![ratatui::text::Span::styled(
        "Equipment: ",
        Style::default()
            .fg(t.text_emphasis)
            .add_modifier(Modifier::BOLD),
    )]));
    lines.push(Line::from(format!(
        "  Main: {:?}",
        app.player
            .equipment
            .main_hand
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("Empty")
    )));
    lines.push(Line::from(format!(
        "  Off:  {:?}",
        app.player
            .equipment
            .off_hand
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("Empty")
    )));
    lines.push(Line::from(format!(
        "  Armor: {:?}",
        app.player
            .equipment
            .armor
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("Empty")
    )));
    lines.push(Line::from(""));

    // Inventory items with rarity colors
    lines.push(Line::from(vec![ratatui::text::Span::styled(
        "Inventory: ",
        Style::default()
            .fg(t.text_emphasis)
            .add_modifier(Modifier::BOLD),
    )]));

    if app.player.inventory.items.is_empty() {
        lines.push(Line::from("(empty)").style(Style::default().fg(t.text_muted)));
    } else {
        for item in &app.player.inventory.items {
            let marker = if item.equipped { "●" } else { "○" };
            let rarity_color = get_item_rarity_color(&item.item_id, &t);

            lines.push(Line::from(vec![
                ratatui::text::Span::raw(format!("{} ", marker)),
                ratatui::text::Span::styled(
                    format!("{:16}", item.item_id),
                    Style::default().fg(rarity_color),
                ),
                ratatui::text::Span::raw(format!("x{}", item.quantity)),
            ]));
        }
    }

    // Action hints
    lines.push(Line::from(""));
    lines.push(Line::from(vec![ratatui::text::Span::styled(
        "Actions: ",
        Style::default().fg(t.text_emphasis),
    )]));
    lines.push(Line::from("[1-4] Use/equip items  [Esc] Close"));

    let list = Paragraph::new(lines).wrap(Wrap { trim: true }).block(
        Block::default()
            .title(format!("Inventory {}", theme::icon("quest")))
            .borders(Borders::ALL)
            .style(Style::default().fg(t.panel_border)),
    );
    frame.render_widget(list, chunks[1]);
}

fn get_item_rarity_color(item_id: &str, t: &crate::ui::tui::theme::Theme) -> Color {
    // Common items - white/gray
    // Uncommon items - green
    // Rare items - blue
    // Very rare - purple
    // Legendary - orange/gold

    let id_lower = item_id.to_lowercase();

    if id_lower.contains("legendary")
        || id_lower.contains("artifact")
        || id_lower.contains("obsidian")
        || id_lower.contains("null")
    {
        // Gold/Orange for legendary
        Color::Rgb(255, 215, 0)
    } else if id_lower.contains("rare")
        || id_lower.contains("unique")
        || id_lower.contains("scepter")
    {
        // Purple for very rare
        Color::Rgb(155, 89, 182)
    } else if id_lower.contains("uncommon")
        || id_lower.contains("magic")
        || id_lower.contains("ring")
    {
        // Blue for uncommon
        Color::Rgb(52, 152, 219)
    } else if id_lower.contains("fine") || id_lower.contains("masterwork") {
        // Green for uncommon
        Color::Rgb(46, 204, 113)
    } else {
        // Default to primary text color
        t.text_primary
    }
}
