use super::core::*;
use super::icons::*;
use super::tier::*;
use super::types::*;
use ratatui::style::{Color, Modifier, Style};
use std::sync::OnceLock;
pub fn reduced_motion() -> bool {
    let value = std::env::var("DND_REDUCED_MOTION")
        .or_else(|_| std::env::var("REDUCED_MOTION"))
        .unwrap_or_default()
        .to_lowercase();
    matches!(value.as_str(), "1" | "true" | "yes" | "on")
}

pub fn health_color(current: i32, maximum: i32) -> Color {
    if maximum == 0 {
        return theme().text_muted;
    }
    let ratio = current as f32 / maximum as f32;
    let t = theme();
    if ratio > 0.6 {
        t.health_full
    } else if ratio > 0.3 {
        t.health_medium
    } else {
        t.health_low
    }
}

pub fn mana_color(_current: i32, maximum: i32) -> Color {
    if maximum == 0 {
        return theme().text_muted;
    }
    theme().mana
}

pub fn xp_color() -> Color {
    theme().xp
}

pub fn progress_bar(current: i32, maximum: i32, width: usize) -> String {
    if maximum == 0 {
        return "=".repeat(width);
    }
    let filled = ((current as f32 / maximum as f32) * width as f32) as usize;
    let filled = filled.min(width);
    let empty = width - filled;
    "=".repeat(filled) + &"-".repeat(empty)
}

pub fn progress_bar_with_value(current: i32, maximum: i32, width: usize) -> String {
    let bar = progress_bar(current, maximum, width);
    format!("{}[{}/{}]", bar, current, maximum)
}

pub fn gradient_color(progress: f32, colors: &[Color]) -> Color {
    if colors.is_empty() {
        return theme().text_primary;
    }
    if colors.len() == 1 {
        return colors[0];
    }

    let progress = progress.clamp(0.0, 1.0);
    let segment = progress * (colors.len() - 1) as f32;
    let index = segment as usize;
    let t = segment - index as f32;

    match (index, colors.get(index), colors.get(index + 1)) {
        (_, Some(c1), Some(c2)) => lerp_color(*c1, *c2, t),
        _ => *colors.last().unwrap_or(&theme().text_primary),
    }
}

fn lerp_color(c1: Color, c2: Color, t: f32) -> Color {
    match (c1, c2) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8;
            let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8;
            let b = (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8;
            Color::Rgb(r, g, b)
        }
        _ => c2,
    }
}

pub fn color_blind_mode() -> Option<&'static str> {
    let mode = std::env::var("DND_COLOR_BLIND")
        .or_else(|_| std::env::var("COLORBLIND_MODE"))
        .unwrap_or_default()
        .to_lowercase();

    match mode.as_str() {
        "protanopia" | "protan" => Some("protanopia"),
        "deuteranopia" | "deutan" => Some("deuteranopia"),
        "tritanopia" | "tritan" => Some("tritanopia"),
        "monochrome" | "mono" => Some("monochrome"),
        _ => None,
    }
}
