/// Application glue layer.
///
/// `App` owns the full mutable game state (`AppState`) and all game
/// sub-systems. It is renderer-agnostic — it has no direct dependency on
/// ratatui, crossterm, egui, or eframe.
///
/// The active renderer calls `App::handle_event` to drive state transitions
/// and reads `App::state` (and sub-system state) during rendering.
use crate::renderer::{ControlFlow, GameEvent};
use anyhow::Result;

// ---------------------------------------------------------------------------
// AppState
// ---------------------------------------------------------------------------

/// Which screen / mode is currently active. The renderer inspects this to
/// decide which screen module to call.
#[derive(Debug, Clone, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    CharacterCreation,
    WorldMap,
    Combat(CombatContext),
    Dialog(DialogContext),
    Journal,
    Inventory,
    Spellbook,
    GameOver,
}

/// Placeholder — will be expanded in `src/game/combat/`.
#[derive(Debug, Clone, Default)]
pub struct CombatContext;

/// Placeholder — will be expanded in `src/game/story/dialog.rs`.
#[derive(Debug, Clone, Default)]
pub struct DialogContext;

// ---------------------------------------------------------------------------
// App
// ---------------------------------------------------------------------------

/// Central application object. Passed by shared reference to every renderer
/// `render()` call; mutated only inside `handle_event()`.
pub struct App {
    pub state: AppState,
    // TODO: add game sub-system handles here as they are implemented:
    //   pub world:     world::World,
    //   pub character: character::Character,
    //   pub journal:   story::Journal,
    //   ...
}

impl App {
    /// Create a new `App` ready to display the main menu.
    pub fn new() -> Self {
        Self {
            state: AppState::default(),
        }
    }

    /// Drive a state transition.
    pub fn transition(&mut self, next: AppState) {
        self.state = next;
    }

    /// Process one `GameEvent` and possibly update game state.
    ///
    /// Returns `ControlFlow::Exit` when the application should shut down.
    pub fn handle_event(&mut self, event: GameEvent) -> Result<ControlFlow> {
        match event {
            GameEvent::Quit => return Ok(ControlFlow::Exit),

            GameEvent::Tick => {
                // TODO: advance animations, cooldowns, emergent events
            }

            // Route remaining events to the active screen handler.
            other => self.dispatch(other)?,
        }
        Ok(ControlFlow::Continue)
    }

    /// Forward an event to the appropriate sub-system based on `AppState`.
    fn dispatch(&mut self, event: GameEvent) -> Result<()> {
        match &self.state {
            AppState::MainMenu => self.handle_main_menu(event),
            AppState::WorldMap => self.handle_world_map(event),
            AppState::Combat(_) => self.handle_combat(event),
            AppState::Dialog(_) => self.handle_dialog(event),
            AppState::Inventory => self.handle_inventory(event),
            AppState::Journal => self.handle_journal(event),
            AppState::Spellbook => self.handle_spellbook(event),
            AppState::CharacterCreation => self.handle_char_creation(event),
            AppState::GameOver => Ok(()),
        }
    }

    // -----------------------------------------------------------------------
    // Per-screen handlers (stubs — will be filled in per milestone)
    // -----------------------------------------------------------------------

    fn handle_main_menu(&mut self, event: GameEvent) -> Result<()> {
        if event == GameEvent::Confirm {
            self.transition(AppState::CharacterCreation);
        }
        Ok(())
    }

    fn handle_world_map(&mut self, _event: GameEvent) -> Result<()> {
        // TODO: movement, interact, open overlays
        Ok(())
    }

    fn handle_combat(&mut self, _event: GameEvent) -> Result<()> {
        // TODO: attack, wait, use item, cast spell
        Ok(())
    }

    fn handle_dialog(&mut self, _event: GameEvent) -> Result<()> {
        // TODO: Choice(n) advances dialog tree
        Ok(())
    }

    fn handle_inventory(&mut self, event: GameEvent) -> Result<()> {
        if event == GameEvent::Back || event == GameEvent::Cancel {
            self.transition(AppState::WorldMap);
        }
        Ok(())
    }

    fn handle_journal(&mut self, event: GameEvent) -> Result<()> {
        if event == GameEvent::Back || event == GameEvent::Cancel {
            self.transition(AppState::WorldMap);
        }
        Ok(())
    }

    fn handle_spellbook(&mut self, event: GameEvent) -> Result<()> {
        if event == GameEvent::Back || event == GameEvent::Cancel {
            self.transition(AppState::WorldMap);
        }
        Ok(())
    }

    fn handle_char_creation(&mut self, event: GameEvent) -> Result<()> {
        if event == GameEvent::Confirm {
            self.transition(AppState::WorldMap);
        }
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
