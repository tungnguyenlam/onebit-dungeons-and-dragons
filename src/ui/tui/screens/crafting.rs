use crate::app::App;
use crate::game::items::crafting::CraftingSystem;
use crate::ui::tui::theme::{self, theme};
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

    let crafting = CraftingSystem::new(app.recipe_defs.clone());
    let available = crafting.get_available_recipes(&app.player.inventory);
    let all_recipes = crafting.get_all_recipes();

    let header = Paragraph::new(vec![
        Line::from(vec![ratatui::text::Span::styled(
            "Crafting & Alchemy ",
            Style::default()
                .fg(t.text_emphasis)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(format!(
            "Available recipes: {}/{}",
            available.len(),
            all_recipes.len()
        )),
    ])
    .style(Style::default().fg(t.text_primary))
    .block(
        Block::default()
            .title(format!(" {} ", theme::icon("spell")))
            .borders(Borders::ALL)
            .style(Style::default().fg(t.panel_border)),
    );
    frame.render_widget(header, chunks[0]);

    let mut lines = Vec::new();

    lines.push(Line::from(vec![ratatui::text::Span::styled(
        "Your Ingredients: ",
        Style::default()
            .fg(t.text_emphasis)
            .add_modifier(Modifier::BOLD),
    )]));

    let ingredient_count = app
        .player
        .inventory
        .items
        .iter()
        .filter(|item| {
            app.item_defs
                .get(&item.item_id)
                .map(|def| def.is_ingredient)
                .unwrap_or(false)
        })
        .count();

    if ingredient_count == 0 {
        lines.push(Line::from("(No ingredients)").style(Style::default().fg(t.text_muted)));
    } else {
        for item in &app.player.inventory.items {
            if app
                .item_defs
                .get(&item.item_id)
                .map(|def| def.is_ingredient)
                .unwrap_or(false)
            {
                let name = app
                    .item_defs
                    .get(&item.item_id)
                    .map(|def| def.name.as_str())
                    .unwrap_or(&item.item_id);
                lines.push(Line::from(format!("  {} x{}", name, item.quantity)));
            }
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![ratatui::text::Span::styled(
        "Recipes: ",
        Style::default()
            .fg(t.text_emphasis)
            .add_modifier(Modifier::BOLD),
    )]));

    for recipe in all_recipes {
        let can_craft = available.iter().any(|r| r.id == recipe.id);
        let status = if can_craft { "[Craft]" } else { "[Missing]" };
        let status_color = if can_craft { t.success } else { t.text_muted };

        lines.push(Line::from(vec![
            ratatui::text::Span::styled(format!("{} ", status), Style::default().fg(status_color)),
            ratatui::text::Span::styled(
                format!("{}: ", recipe.name),
                Style::default().fg(t.text_primary),
            ),
        ]));

        for ing in &recipe.ingredients {
            let name = app
                .item_defs
                .get(&ing.item_id)
                .map(|def| def.name.as_str())
                .unwrap_or(&ing.item_id);
            let have = app.player.inventory.count(&ing.item_id);
            let ing_color = if have >= ing.quantity {
                t.text_primary
            } else {
                t.danger
            };
            lines.push(Line::from(format!(
                "    - {} x{} (have: {})",
                name, ing.quantity, have
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![ratatui::text::Span::styled(
        "Actions: ",
        Style::default().fg(t.text_emphasis),
    )]));
    lines.push(Line::from("[1-5] Craft selected recipe  [Esc] Close"));

    let list = Paragraph::new(lines).wrap(Wrap { trim: true }).block(
        Block::default()
            .title(" Crafting ")
            .borders(Borders::ALL)
            .style(Style::default().fg(t.panel_border)),
    );
    frame.render_widget(list, chunks[1]);
}
