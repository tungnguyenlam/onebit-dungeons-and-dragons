/// TUI renderer — Ratatui + Crossterm.
///
/// Implements `GameRenderer`. The main loop calls `render` and `poll_event`
/// without knowing anything about crossterm or ratatui types.
///
/// Sub-modules mirror the screen list from `docs/architecture/ui-layer.md`.
// pub mod layout;
pub mod screens;
pub mod theme;
// pub mod widgets;

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

// ---------------------------------------------------------------------------
// TuiRenderer
// ---------------------------------------------------------------------------

pub struct TuiRenderer {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TuiRenderer {
    /// Initialise crossterm raw mode + alternate screen, then build the
    /// ratatui Terminal.
    pub fn new() -> Result<Self> {
        let _ = theme::init_terminal_tier();
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }
}

impl GameRenderer for TuiRenderer {
    fn render(&mut self, app: &App) -> Result<()> {
        if app.sound_enabled && app.pending_beep.replace(false) {
            print!("\x07");
            let _ = io::stdout().flush();
        }
        self.terminal.draw(|frame| {
            match &app.state {
                AppState::MainMenu => screens::main_menu::render(frame, app),
                AppState::CharacterCreation => screens::character_creation::render(frame, app),
                AppState::WorldMap => screens::world_map::render(frame, app),
                AppState::Combat(_) => screens::combat::render(frame, app),
                AppState::Dialog(_) => screens::dialog::render(frame, app),
                AppState::Journal => screens::journal::render(frame, app),
                AppState::Inventory => screens::inventory::render(frame, app),
                AppState::Spellbook => screens::spellbook::render(frame, app),
                AppState::GameOver => screens::game_over::render(frame, app),
            }
        })?;
        Ok(())
    }

    fn poll_event(&mut self) -> Result<GameEvent> {
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                return Ok(map_key(key));
            }
        }
        Ok(GameEvent::Tick)
    }

    fn teardown(mut self: Box<Self>) -> Result<()> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Key mapping
// ---------------------------------------------------------------------------

/// Map a crossterm `KeyEvent` to a renderer-agnostic `GameEvent`.
/// Unmapped keys are silently turned into `GameEvent::Tick` (no-op).
fn map_key(key: KeyEvent) -> GameEvent {
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
        KeyCode::Esc | KeyCode::Backspace => GameEvent::Cancel,

        // In-game shortcuts
        KeyCode::Char('i') => GameEvent::OpenInventory,
        KeyCode::Char('s') => GameEvent::OpenSpellbook,
        KeyCode::Char('n') => GameEvent::OpenJournal,
        KeyCode::Char('m') => GameEvent::OpenMap,
        KeyCode::Char('a') => GameEvent::Attack,
        KeyCode::Char('.') => GameEvent::Wait,
        KeyCode::Char('p') => GameEvent::SaveGame,
        KeyCode::Char('o') => GameEvent::LoadGame,
        KeyCode::Char('b') => GameEvent::ToggleSound,

        // Dialog choices
        KeyCode::Char(c @ '1'..='9') => GameEvent::Choice(c as u8 - b'0'),

        _ => GameEvent::Tick, // unrecognised — treated as no-op
    }
}
