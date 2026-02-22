use crate::ui::tui::theme::{health_color, mana_color, progress_bar, theme, xp_color};
use ratatui::{buffer::Buffer, layout::Rect, style::Style};

pub struct ProgressBar {
    pub width: u16,
    pub filled_style: Style,
    pub empty_style: Style,
    pub show_text: bool,
}

impl ProgressBar {
    pub fn new(width: u16) -> Self {
        Self {
            width,
            filled_style: Style::default(),
            empty_style: Style::default(),
            show_text: true,
        }
    }

    pub fn with_filled_style(mut self, style: Style) -> Self {
        self.filled_style = style;
        self
    }

    pub fn with_empty_style(mut self, style: Style) -> Self {
        self.empty_style = style;
        self
    }

    pub fn show_text(mut self, show: bool) -> Self {
        self.show_text = show;
        self
    }

    pub fn render(&self, buf: &mut Buffer, area: Rect, current: i32, maximum: i32) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let width = area.width.min(self.width);
        let filled = if maximum > 0 {
            ((current as f32 / maximum as f32) * width as f32) as u16
        } else {
            0
        };

        for x in 0..width {
            let char = if (x as u16) < filled { '█' } else { '░' };
            let style = if (x as u16) < filled {
                self.filled_style
            } else {
                self.empty_style
            };
            buf.get_mut(area.x + x, area.y)
                .set_char(char)
                .set_style(style);
        }
    }
}

pub fn render_health_bar(buf: &mut Buffer, area: Rect, current: i32, maximum: i32, label: &str) {
    if area.width == 0 {
        return;
    }

    let _t = theme();
    let width = area.width.saturating_sub(2);
    let bar = progress_bar(current, maximum, width as usize);
    let color = health_color(current, maximum);

    let label_text = format!("{}: ", label);
    for (i, c) in label_text.chars().enumerate() {
        let pos = area.x + i as u16;
        if pos < area.x + area.width {
            buf.get_mut(pos, area.y).set_char(c);
        }
    }

    for (i, c) in bar.chars().enumerate() {
        let x = area.x + label_text.len() as u16 + i as u16;
        if x < area.x + area.width {
            buf.get_mut(x, area.y)
                .set_char(c)
                .set_style(Style::default().fg(color));
        }
    }
}

pub fn render_mana_bar(buf: &mut Buffer, area: Rect, current: i32, maximum: i32, label: &str) {
    if area.width == 0 {
        return;
    }

    let _t = theme();
    let width = area.width.saturating_sub(2);
    let bar = progress_bar(current, maximum, width as usize);
    let color = mana_color(current, maximum);

    let label_text = format!("{}: ", label);
    for (i, c) in label_text.chars().enumerate() {
        let pos = area.x + i as u16;
        if pos < area.x + area.width {
            buf.get_mut(pos, area.y).set_char(c);
        }
    }

    for (i, c) in bar.chars().enumerate() {
        let x = area.x + label_text.len() as u16 + i as u16;
        if x < area.x + area.width {
            buf.get_mut(x, area.y)
                .set_char(c)
                .set_style(Style::default().fg(color));
        }
    }
}

pub fn render_xp_bar(buf: &mut Buffer, area: Rect, current: u32, maximum: u32, level: u8) {
    if area.width == 0 {
        return;
    }

    let _t = theme();
    let width = area.width.saturating_sub(2);
    let bar = progress_bar(current as i32, maximum as i32, width as usize);
    let color = xp_color();

    let label_text = format!("Lv{} XP: ", level);
    for (i, c) in label_text.chars().enumerate() {
        let pos = area.x + i as u16;
        if pos < area.x + area.width {
            buf.get_mut(pos, area.y).set_char(c);
        }
    }

    for (i, c) in bar.chars().enumerate() {
        let x = area.x + label_text.len() as u16 + i as u16;
        if x < area.x + area.width {
            buf.get_mut(x, area.y)
                .set_char(c)
                .set_style(Style::default().fg(color));
        }
    }
}
