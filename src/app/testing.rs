use crate::app::{App, AppState};
use crate::renderer::{ControlFlow, GameEvent, GameRenderer};
use anyhow::Result;

pub struct HeadlessRenderer {
    frame_count: u64,
    event_log: Vec<String>,
    state_snapshots: Vec<AppSnapshot>,
    captured_frames: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AppSnapshot {
    pub frame: u64,
    pub state: String,
    pub player_hp: i32,
    pub player_max_hp: i32,
    pub player_level: u8,
    pub player_xp: u32,
    pub current_room: String,
    pub current_region: String,
    pub quest_count: usize,
    pub journal_entries: usize,
    pub inventory_count: usize,
}

impl HeadlessRenderer {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            event_log: Vec::new(),
            state_snapshots: Vec::new(),
            captured_frames: Vec::new(),
        }
    }

    pub fn capture_snapshot(&mut self, app: &App) {
        let player = &app.player;
        self.frame_count += 1;

        let state_name = match &app.state {
            AppState::MainMenu => "MainMenu",
            AppState::CharacterCreation => "CharacterCreation",
            AppState::WorldMap => "WorldMap",
            AppState::Combat(_) => "Combat",
            AppState::Dialog(_) => "Dialog",
            AppState::Journal => "Journal",
            AppState::Inventory => "Inventory",
            AppState::Spellbook => "Spellbook",
            AppState::Settings => "Settings",
            AppState::GameOver => "GameOver",
        }
        .to_string();

        self.state_snapshots.push(AppSnapshot {
            frame: self.frame_count,
            state: state_name,
            player_hp: player.hp,
            player_max_hp: player.max_hp(),
            player_level: player.level,
            player_xp: player.xp,
            current_room: app.current_room_id.clone(),
            current_region: app.region.manifest.slug.clone(),
            quest_count: app.quests.states.len(),
            journal_entries: app.journal.entries.len(),
            inventory_count: player.inventory.len(),
        });
    }

    pub fn log_event(&mut self, event: &str) {
        self.event_log
            .push(format!("[{}] {}", self.frame_count, event));
    }

    pub fn get_snapshots(&self) -> &[AppSnapshot] {
        &self.state_snapshots
    }

    pub fn get_last_snapshot(&self) -> Option<&AppSnapshot> {
        self.state_snapshots.last()
    }

    pub fn get_event_log(&self) -> &[String] {
        &self.event_log
    }

    pub fn reset(&mut self) {
        self.frame_count = 0;
        self.event_log.clear();
        self.state_snapshots.clear();
        self.captured_frames.clear();
    }
}

impl Default for HeadlessRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl GameRenderer for HeadlessRenderer {
    fn render(&mut self, app: &App) -> Result<()> {
        self.capture_snapshot(app);
        Ok(())
    }

    fn poll_event(&mut self) -> Result<GameEvent> {
        Ok(GameEvent::Tick)
    }

    fn teardown(self: Box<Self>) -> Result<()> {
        Ok(())
    }
}

pub struct TestingEngine {
    pub renderer: HeadlessRenderer,
    pub actions: Vec<GameEvent>,
    pub current_action: usize,
    pub auto_advance: bool,
}

impl TestingEngine {
    pub fn new() -> Self {
        Self {
            renderer: HeadlessRenderer::new(),
            actions: Vec::new(),
            current_action: 0,
            auto_advance: true,
        }
    }

    pub fn add_action(&mut self, action: GameEvent) {
        self.actions.push(action);
    }

    pub fn add_move(&mut self, direction: MoveDirection) {
        let action = match direction {
            MoveDirection::Up => GameEvent::MoveUp,
            MoveDirection::Down => GameEvent::MoveDown,
            MoveDirection::Left => GameEvent::MoveLeft,
            MoveDirection::Right => GameEvent::MoveRight,
        };
        self.actions.push(action);
    }

    pub fn add_wait(&mut self) {
        self.actions.push(GameEvent::Wait);
    }

    pub fn add_attack(&mut self) {
        self.actions.push(GameEvent::Attack);
    }

    pub fn add_confirm(&mut self) {
        self.actions.push(GameEvent::Confirm);
    }

    pub fn add_cancel(&mut self) {
        self.actions.push(GameEvent::Cancel);
    }

    pub fn add_open_inventory(&mut self) {
        self.actions.push(GameEvent::OpenInventory);
    }

    pub fn add_open_journal(&mut self) {
        self.actions.push(GameEvent::OpenJournal);
    }

    pub fn next_action(&mut self) -> Option<GameEvent> {
        if self.current_action < self.actions.len() {
            let action = self.actions[self.current_action].clone();
            self.current_action += 1;
            Some(action)
        } else if self.auto_advance {
            self.current_action = 0;
            self.actions.first().cloned()
        } else {
            None
        }
    }

    pub fn has_more_actions(&self) -> bool {
        self.current_action < self.actions.len() || self.auto_advance
    }

    pub fn reset(&mut self) {
        self.current_action = 0;
        self.renderer.reset();
    }
}

impl Default for TestingEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

pub fn run_scenario<F>(mut app: App, engine: &mut TestingEngine, mut handler: F) -> App
where
    F: FnMut(&App, &HeadlessRenderer),
{
    loop {
        engine.renderer.capture_snapshot(&app);
        handler(&app, &engine.renderer);

        if let Some(action) = engine.next_action() {
            match app.handle_event(action).unwrap_or(ControlFlow::Exit) {
                ControlFlow::Exit => break,
                ControlFlow::Continue => {}
            }
        } else {
            break;
        }
    }
    app
}
