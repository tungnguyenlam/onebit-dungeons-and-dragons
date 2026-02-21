use crate::app::App;
use crate::ui::tui::theme::{self, theme, TerminalTier};
use crate::ui::tui::vfx::{VfxEngine, VfxTier};
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

static ANIMATION_FRAME: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(10),
        Constraint::Length(4),
    ])
    .split(area);

    let t = theme();
    let tier = theme::terminal_tier();

    // Animated title - only on T2/T3 terminals with reduced_motion disabled
    let frame_count = ANIMATION_FRAME.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let use_animation =
        !theme::reduced_motion() && matches!(tier, TerminalTier::T2 | TerminalTier::T3);
    let title_text = if use_animation {
        // Animated version - cycle through slight variations
        let frames = [
            "OneBit Dungeons & Dragons",
            "OneBit Dungeons & Dragons",
            "OneBit Dungeons & Dragons",
        ];
        let idx = (frame_count / 30) as usize % frames.len();
        frames[idx]
    } else {
        "OneBit Dungeons & Dragons"
    };

    let title = Paragraph::new(title_text)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(t.accent_primary)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .title("Main Menu")
                .borders(Borders::ALL)
                .style(Style::default().fg(t.panel_border)),
        );
    frame.render_widget(title, chunks[0]);

    // Menu options with selection highlight
    let items = ["New Game", "Continue", "Load Save", "Quit"];
    let mut lines = Vec::new();
    for (idx, item) in items.iter().enumerate() {
        let is_selected = idx == app.menu_ui.selected;
        let marker = if is_selected { "►" } else { " " };

        let style = if is_selected {
            Style::default()
                .fg(t.accent_primary)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(t.text_primary)
        };

        lines.push(Line::from(vec![
            ratatui::text::Span::raw(format!("{} ", marker)),
            ratatui::text::Span::styled(*item, style),
        ]));
    }

    frame.render_widget(
        Paragraph::new(lines).block(
            Block::default()
                .title("Options")
                .borders(Borders::ALL)
                .style(Style::default().fg(t.panel_border)),
        ),
        chunks[1],
    );

    // Footer with controls and system info
    let footer_text = vec![
        Line::from(vec![
            ratatui::text::Span::raw(format!("{} ", theme::icon("warning"))),
            ratatui::text::Span::raw("↑/↓ select  "),
            ratatui::text::Span::raw("Enter confirm  "),
            ratatui::text::Span::raw("q quit"),
        ]),
        Line::from(vec![
            ratatui::text::Span::raw("b toggle sound  "),
            ratatui::text::Span::raw("p save  "),
            ratatui::text::Span::raw("o load  "),
            ratatui::text::Span::styled(
                format!("Tier: {:?}", tier),
                Style::default().fg(t.text_muted),
            ),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(footer_text)
            .style(Style::default().fg(t.text_muted))
            .block(
                Block::default()
                    .title("Controls")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(t.panel_border)),
            ),
        chunks[2],
    );
}
