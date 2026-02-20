/// Renderer abstraction — the boundary between game logic and any front-end.
///
/// `src/game/` and `src/app.rs` are completely unaware of which renderer is
/// active. The renderer is selected at launch via `--mode tui|gui` and
/// injected into the main loop as a `Box<dyn GameRenderer>`.
use crate::app::App;
use anyhow::Result;

// ---------------------------------------------------------------------------
// GameEvent
// ---------------------------------------------------------------------------

/// A renderer-agnostic input event. Both TuiRenderer and GuiRenderer map
/// their platform-specific inputs (crossterm KeyEvent / egui Key) down to
/// these variants before returning them from `poll_event`.
///
/// `App::handle_event` only ever sees `GameEvent`, never raw key codes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameEvent {
    // --- navigation ---
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Confirm,
    Cancel,
    Back,

    // --- in-game actions ---
    OpenInventory,
    OpenSpellbook,
    OpenJournal,
    OpenMap,
    Attack,
    Wait,

    // --- dialog choice (key 1–9) ---
    Choice(u8),

    // --- system ---
    /// Fired every ~250 ms when there is no keyboard input. Used to advance
    /// animations, cooldowns, and timed world events.
    Tick,
    /// Tells the main loop to exit cleanly.
    Quit,
}

// ---------------------------------------------------------------------------
// GameRenderer trait
// ---------------------------------------------------------------------------

/// Implemented by `TuiRenderer` (`feature = "tui"`) and
/// `GuiRenderer` (`feature = "gui"`).
///
/// The main loop is:
/// ```
/// loop {
///     renderer.render(&app)?;
///     match renderer.poll_event()? {
///         GameEvent::Quit => break,
///         event => app.handle_event(event)?,
///     }
/// }
/// renderer.teardown()?;
/// ```
pub trait GameRenderer {
    /// Draw the current application state to the screen or window.
    /// Receives a *shared* reference to `App` — must not mutate game state.
    fn render(&mut self, app: &App) -> Result<()>;

    /// Block until the next input event or until the 250 ms tick timeout
    /// fires. Returns `GameEvent::Tick` on timeout.
    fn poll_event(&mut self) -> Result<GameEvent>;

    /// Restore the environment (disable raw mode, drop the window, etc.)
    /// and release all resources. Called exactly once after the loop exits.
    fn teardown(self: Box<Self>) -> Result<()>;
}

// ---------------------------------------------------------------------------
// ControlFlow
// ---------------------------------------------------------------------------

/// Returned by `App::handle_event` to signal whether the loop should exit.
#[derive(Debug, PartialEq, Eq)]
pub enum ControlFlow {
    Continue,
    Exit,
}
