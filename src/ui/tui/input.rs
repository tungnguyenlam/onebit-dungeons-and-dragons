use crate::app::{App, AppState};
use crate::renderer::{GameEvent, GameRenderer};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::{self, Stdout, Write},
    time::Duration,
};
/// Map a crossterm `KeyEvent` to a renderer-agnostic `GameEvent`.
/// Unmapped keys are silently turned into `GameEvent::Tick` (no-op).
pub fn map_key(key: KeyEvent) -> GameEvent {
    match key.code {
        // Quit
        KeyCode::Char('q') | KeyCode::Char('Q') => GameEvent::Quit,
        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => GameEvent::Quit,

        // Navigation
        KeyCode::Up | KeyCode::Char('k') => GameEvent::MoveUp,
        KeyCode::Down | KeyCode::Char('j') => GameEvent::MoveDown,
        KeyCode::Left | KeyCode::Char('h') => GameEvent::MoveLeft,
        KeyCode::Right | KeyCode::Char('l') => GameEvent::MoveRight,
        KeyCode::Enter => GameEvent::Confirm,
        KeyCode::Esc => GameEvent::Cancel,
        KeyCode::Backspace => GameEvent::Back,

        // In-game shortcuts
        KeyCode::Char('i') => GameEvent::OpenInventory,
        KeyCode::Char('c') => GameEvent::OpenCrafting,
        KeyCode::Char('v') => GameEvent::OpenBestiary,
        KeyCode::Char('y') => GameEvent::OpenLoreLibrary,
        KeyCode::Char('s') => GameEvent::OpenSpellbook,
        KeyCode::Char('n') => GameEvent::OpenJournal,
        KeyCode::Char('m') => GameEvent::OpenMap,
        KeyCode::Char('a') => GameEvent::Attack,
        KeyCode::Char('f') => GameEvent::Choice(4),
        KeyCode::Char('.') => GameEvent::Wait,
        KeyCode::Char('p') => GameEvent::SaveGame,
        KeyCode::Char('o') => GameEvent::LoadGame,
        KeyCode::Char('b') => GameEvent::ToggleSound,
        KeyCode::Char(',') => GameEvent::OpenSettings,
        KeyCode::Char('?') => GameEvent::OpenHelp,

        // Dialog choices
        KeyCode::Char(c @ '1'..='9') => GameEvent::Choice(c as u8 - b'0'),
        KeyCode::Char(c) if c.is_ascii_alphabetic() && c.is_ascii_uppercase() => {
            GameEvent::TextInput(c)
        }
        KeyCode::Char(' ') => GameEvent::TextInput(' '),

        _ => GameEvent::Tick, // unrecognised — treated as no-op
    }
}
