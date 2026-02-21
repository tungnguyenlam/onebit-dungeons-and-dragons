/// GUI renderer — egui + eframe.
///
/// eframe drives its own platform event loop, so the architecture differs
/// slightly from TUI. `GuiRenderer` implements `eframe::App` and is handed
/// directly to `eframe::run_native`. Inside `update()` it both draws the
/// UI *and* collects input, translating egui keys to `GameEvent` and
/// forwarding them to `App::handle_event`.
///
/// The `GameRenderer` trait is **also** implemented for cases where a
/// headless test harness needs to drive the renderer through the standard
/// interface. In production, `run()` is used instead of the trait.
// pub mod screens;
// pub mod widgets;
use crate::app::{App, AppState};
use crate::renderer::{ControlFlow, GameEvent, GameRenderer};
use anyhow::Result;
use eframe::egui;

// ---------------------------------------------------------------------------
// Top-level entry point called from main.rs
// ---------------------------------------------------------------------------

/// Start the eframe event loop. Blocks until the window is closed.
pub fn run(app: App) -> Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("OneBit Dungeons & Dragons")
            .with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "OneBit D&D",
        native_options,
        Box::new(|_cc| Ok(Box::new(GuiApp::new(app)))),
    )
    .map_err(|e| anyhow::anyhow!("eframe error: {e}"))
}

// ---------------------------------------------------------------------------
// GuiApp — implements eframe::App
// ---------------------------------------------------------------------------

struct GuiApp {
    app: App,
}

impl GuiApp {
    fn new(app: App) -> Self {
        Self { app }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Collect input events and forward to game logic.
        ctx.input(|input| {
            for event in &input.events {
                if let egui::Event::Key {
                    key, pressed: true, ..
                } = event
                {
                    if let Some(game_event) = map_key(*key) {
                        if let Ok(cf) = self.app.handle_event(game_event) {
                            if cf == ControlFlow::Exit {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }
                    }
                }
            }
        });

        // Draw the current screen.
        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO: dispatch to the correct screen module based on app.state.
            // For now render a placeholder until screens are implemented.
            let title = match &self.app.state {
                AppState::MainMenu => "Main Menu",
                AppState::CharacterCreation => "Character Creation",
                AppState::WorldMap => "World Map",
                AppState::Combat(_) => "Combat",
                AppState::Dialog(_) => "Dialog",
                AppState::Journal => "Journal",
                AppState::Inventory => "Inventory",
                AppState::Spellbook => "Spellbook",
                AppState::GameOver => "Game Over",
            };
            ui.vertical_centered(|ui| {
                ui.heading(format!("[GUI] {title}"));
                ui.label("Press Q to quit.");
            });
        });
    }
}

// ---------------------------------------------------------------------------
// Key mapping
// ---------------------------------------------------------------------------

/// Map an egui `Key` to a renderer-agnostic `GameEvent`.
/// Returns `None` for unmapped keys (they are silently ignored).
fn map_key(key: egui::Key) -> Option<GameEvent> {
    Some(match key {
        // Quit
        egui::Key::Q => GameEvent::Quit,
        egui::Key::Escape => GameEvent::Cancel,

        // Navigation
        egui::Key::ArrowUp | egui::Key::K => GameEvent::MoveUp,
        egui::Key::ArrowDown | egui::Key::J => GameEvent::MoveDown,
        egui::Key::ArrowLeft | egui::Key::H => GameEvent::MoveLeft,
        egui::Key::ArrowRight | egui::Key::L => GameEvent::MoveRight,
        egui::Key::Enter => GameEvent::Confirm,
        egui::Key::Backspace => GameEvent::Back,

        // In-game shortcuts
        egui::Key::I => GameEvent::OpenInventory,
        egui::Key::S => GameEvent::OpenSpellbook,
        egui::Key::N => GameEvent::OpenJournal,
        egui::Key::M => GameEvent::OpenMap,
        egui::Key::A => GameEvent::Attack,
        egui::Key::Period => GameEvent::Wait,
        egui::Key::P => GameEvent::SaveGame,
        egui::Key::O => GameEvent::LoadGame,
        egui::Key::B => GameEvent::ToggleSound,

        // Dialog choices
        egui::Key::Num1 => GameEvent::Choice(1),
        egui::Key::Num2 => GameEvent::Choice(2),
        egui::Key::Num3 => GameEvent::Choice(3),
        egui::Key::Num4 => GameEvent::Choice(4),
        egui::Key::Num5 => GameEvent::Choice(5),
        egui::Key::Num6 => GameEvent::Choice(6),
        egui::Key::Num7 => GameEvent::Choice(7),
        egui::Key::Num8 => GameEvent::Choice(8),
        egui::Key::Num9 => GameEvent::Choice(9),

        _ => return None,
    })
}

// ---------------------------------------------------------------------------
// GameRenderer impl (for testing / headless usage)
// ---------------------------------------------------------------------------

/// A zero-display renderer that records events for headless tests.
pub struct GuiRenderer {
    pending_events: std::collections::VecDeque<GameEvent>,
}

impl GuiRenderer {
    pub fn new() -> Self {
        Self {
            pending_events: std::collections::VecDeque::new(),
        }
    }

    /// Inject a synthetic event (for tests).
    pub fn push_event(&mut self, event: GameEvent) {
        self.pending_events.push_back(event);
    }
}

impl GameRenderer for GuiRenderer {
    fn render(&mut self, _app: &App) -> Result<()> {
        // No-op in headless mode; real rendering happens inside eframe::App.
        Ok(())
    }

    fn poll_event(&mut self) -> Result<GameEvent> {
        Ok(self.pending_events.pop_front().unwrap_or(GameEvent::Tick))
    }

    fn teardown(self: Box<Self>) -> Result<()> {
        Ok(())
    }
}
