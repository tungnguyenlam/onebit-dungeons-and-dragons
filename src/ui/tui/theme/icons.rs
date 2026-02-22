use ratatui::style::{Color, Modifier, Style};
use std::sync::OnceLock;
use super::types::*;
use super::tier::*;
use super::core::*;
use super::helpers::*;
pub fn icon(key: &str) -> &'static str {
    let tier = terminal_tier();
    match tier {
        TerminalTier::T0 | TerminalTier::T1 => match key {
            "health" => "HP",
            "quest" => "[Q]",
            "magic" => "MP",
            "warning" => "!",
            "player" => "@",
            "enemy" => "E",
            "npc" => "N",
            "item" => "[*]",
            "gold" => "$",
            "xp" => "XP",
            "level" => "Lv",
            "door_open" => "/",
            "door_closed" => "+",
            "chest" => "[C]",
            "stairs_up" => "<",
            "stairs_down" => ">",
            "water" => "~",
            "fire" => "~",
            "trap" => "X",
            "exit" => ">",
            "locked" => "[X]",
            "key" => "-K-",
            "attack" => "[A]",
            "defend" => "[D]",
            "shield" => "[S]",
            "dead" => "XXX",
            "victory" => "***",
            "heart" => "<3",
            "star" => "*",
            "check" => "[+]",
            "cross" => "[x]",
            _ => "*",
        },
        _ => match key {
            "health" => "♥",
            "quest" => "📜",
            "magic" => "✦",
            "warning" => "⚠",
            "player" => "@",
            "enemy" => "👹",
            "npc" => "👤",
            "item" => "💎",
            "gold" => "💰",
            "xp" => "⭐",
            "level" => "⬆",
            "door_open" => "╱",
            "door_closed" => "╼",
            "chest" => "📦",
            "stairs_up" => "⬆",
            "stairs_down" => "⬇",
            "water" => "💧",
            "fire" => "🔥",
            "trap" => "☠",
            "exit" => "🚪",
            "locked" => "🔒",
            "key" => "🔑",
            "attack" => "⚔",
            "defend" => "🛡",
            "shield" => "🛡",
            "dead" => "💀",
            "victory" => "🏆",
            "heart" => "♥",
            "star" => "★",
            "check" => "✓",
            "cross" => "✗",
            _ => "•",
        },
    }
}

pub fn icon_with_fallback(key: &str) -> (String, String) {
    let tier = terminal_tier();
    let fallback = icon(key);

    let fancy = match tier {
        TerminalTier::T0 | TerminalTier::T1 => fallback.to_string(),
        _ => match key {
            "health" => "♥".to_string(),
            "quest" => "📜".to_string(),
            "magic" => "✦".to_string(),
            "warning" => "⚠".to_string(),
            "player" => "@".to_string(),
            "enemy" => "👹".to_string(),
            "npc" => "👤".to_string(),
            "item" => "💎".to_string(),
            "gold" => "💰".to_string(),
            "xp" => "⭐".to_string(),
            "level" => "⬆".to_string(),
            "door_open" => "╱".to_string(),
            "door_closed" => "╼".to_string(),
            "chest" => "📦".to_string(),
            "stairs_up" => "⬆".to_string(),
            "stairs_down" => "⬇".to_string(),
            "water" => "💧".to_string(),
            "fire" => "🔥".to_string(),
            "trap" => "☠".to_string(),
            "exit" => "🚪".to_string(),
            "locked" => "🔒".to_string(),
            "key" => "🔑".to_string(),
            "attack" => "⚔".to_string(),
            "defend" => "🛡".to_string(),
            "shield" => "🛡".to_string(),
            "dead" => "💀".to_string(),
            "victory" => "🏆".to_string(),
            "heart" => "♥".to_string(),
            "star" => "★".to_string(),
            "check" => "✓".to_string(),
            "cross" => "✗".to_string(),
            _ => fallback.to_string(),
        },
    };

    (fallback.to_string(), fancy)
}