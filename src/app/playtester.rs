use crate::app::{App, AppState};
use crate::renderer::GameEvent;
use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct PlaytestReport {
    pub rooms_visited: Vec<String>,
    pub items_found: Vec<String>,
    pub quests_started: Vec<String>,
    pub quests_completed: Vec<String>,
    pub combats_encountered: usize,
    pub npcs_interacted: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub exploration_percent: f32,
}

pub struct PlaytestAgent {
    visited_rooms: HashSet<String>,
    visited_regions: HashSet<String>,
    known_exits: HashMap<String, Vec<String>>,
    action_queue: VecDeque<GameEvent>,
    max_steps: usize,
    current_step: usize,
    report: PlaytestReport,
    state_history: Vec<String>,
}

impl PlaytestAgent {
    pub fn new(max_steps: usize) -> Self {
        Self {
            visited_rooms: HashSet::new(),
            visited_regions: HashSet::new(),
            known_exits: HashMap::new(),
            action_queue: VecDeque::new(),
            max_steps,
            current_step: 0,
            report: PlaytestReport {
                rooms_visited: Vec::new(),
                items_found: Vec::new(),
                quests_started: Vec::new(),
                quests_completed: Vec::new(),
                combats_encountered: 0,
                npcs_interacted: 0,
                errors: Vec::new(),
                warnings: Vec::new(),
                exploration_percent: 0.0,
            },
            state_history: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.visited_rooms.clear();
        self.visited_regions.clear();
        self.known_exits.clear();
        self.action_queue.clear();
        self.current_step = 0;
        self.report = PlaytestReport {
            rooms_visited: Vec::new(),
            items_found: Vec::new(),
            quests_started: Vec::new(),
            quests_completed: Vec::new(),
            combats_encountered: 0,
            npcs_interacted: 0,
            errors: Vec::new(),
            warnings: Vec::new(),
            exploration_percent: 0.0,
        };
        self.state_history.clear();
    }

    pub fn record_room(&mut self, room_id: &str, region_slug: &str) {
        let key = format!("{}:{}", region_slug, room_id);
        if self.visited_rooms.insert(key.clone()) {
            self.report.rooms_visited.push(room_id.to_string());
        }
        self.visited_regions.insert(region_slug.to_string());
    }

    pub fn record_combat(&mut self) {
        self.report.combats_encountered += 1;
    }

    pub fn record_npc_interaction(&mut self) {
        self.report.npcs_interacted += 1;
    }

    pub fn record_item(&mut self, item_id: &str) {
        if !self.report.items_found.contains(&item_id.to_string()) {
            self.report.items_found.push(item_id.to_string());
        }
    }

    pub fn record_quest_start(&mut self, quest_id: &str) {
        if !self.report.quests_started.contains(&quest_id.to_string()) {
            self.report.quests_started.push(quest_id.to_string());
        }
    }

    pub fn record_quest_complete(&mut self, quest_id: &str) {
        if !self.report.quests_completed.contains(&quest_id.to_string()) {
            self.report.quests_completed.push(quest_id.to_string());
        }
    }

    pub fn add_error(&mut self, error: &str) {
        self.report.errors.push(error.to_string());
    }

    pub fn add_warning(&mut self, warning: &str) {
        self.report.warnings.push(warning.to_string());
    }

    pub fn record_state(&mut self, state: &str) {
        self.state_history.push(state.to_string());
    }

    pub fn get_next_action(&mut self, app: &App) -> Option<GameEvent> {
        if self.current_step >= self.max_steps {
            self.add_warning(&format!("Max steps ({}) reached", self.max_steps));
            return None;
        }

        self.current_step += 1;

        match &app.state {
            AppState::WorldMap => self.plan_worldmap_action(app),
            AppState::Combat => self.plan_combat_action(app),
            AppState::Dialog => self.plan_dialog_action(app),
            AppState::MainMenu => Some(GameEvent::Confirm),
            AppState::CharacterCreation => Some(GameEvent::Confirm),
            AppState::Inventory => Some(GameEvent::Cancel),
            AppState::Spellbook => Some(GameEvent::Cancel),
            AppState::Journal => Some(GameEvent::Cancel),
            AppState::Settings => Some(GameEvent::Cancel),
            AppState::GameOver => None,
        }
    }

    fn plan_worldmap_action(&mut self, app: &App) -> Option<GameEvent> {
        let room_id = &app.current_room_id;
        let region_slug = &app.region.manifest.slug;

        self.record_room(room_id, region_slug);

        let directions = [
            GameEvent::MoveUp,
            GameEvent::MoveDown,
            GameEvent::MoveLeft,
            GameEvent::MoveRight,
        ];

        for dir in directions {
            if self.action_queue.pop_front().is_some() {
                return Some(dir);
            }
        }

        Some(directions[rand::random::<usize>() % 4])
    }

    fn plan_combat_action(&mut self, app: &App) -> Option<GameEvent> {
        self.record_combat();

        let actions = [GameEvent::Attack, GameEvent::Wait];

        Some(actions[rand::random::<usize>() % 2])
    }

    fn plan_dialog_action(&mut self, app: &App) -> Option<GameEvent> {
        self.record_npc_interaction();

        if let AppState::Dialog(ctx) = &app.state {
            if ctx.resolved.choices.is_empty() {
                return Some(GameEvent::Cancel);
            }
            return Some(GameEvent::Choice(1));
        }

        Some(GameEvent::Cancel)
    }

    pub fn finalize_report(&mut self, total_rooms: usize) {
        if total_rooms > 0 {
            self.report.exploration_percent =
                (self.visited_rooms.len() as f32 / total_rooms as f32) * 100.0;
        }
    }

    pub fn get_report(&self) -> &PlaytestReport {
        &self.report
    }

    pub fn run_playtest(mut self, mut app: App) -> (App, PlaytestReport) {
        loop {
            let state_name = format!("{:?}", app.state);
            self.record_state(&state_name);

            if let Some(action) = self.get_next_action(&app) {
                match app
                    .handle_event(action)
                    .unwrap_or(crate::renderer::ControlFlow::Exit)
                {
                    crate::renderer::ControlFlow::Exit => break,
                    crate::renderer::ControlFlow::Continue => {}
                }
            } else {
                break;
            }
        }

        self.finalize_report(100);
        (app, self.report.clone())
    }
}

impl PlaytestReport {
    pub fn summary(&self) -> String {
        format!(
            "Playtest Report:\n\
            Rooms Visited: {} ({:.1}%)\n\
            Items Found: {}\n\
            Quests Started: {}\n\
            Quests Completed: {}\n\
            Combats: {}\n\
            NPC Interactions: {}\n\
            Errors: {}\n\
            Warnings: {}",
            self.rooms_visited.len(),
            self.exploration_percent,
            self.items_found.len(),
            self.quests_started.len(),
            self.quests_completed.len(),
            self.combats_encountered,
            self.npcs_interacted,
            self.errors.len(),
            self.warnings.len()
        )
    }

    pub fn detailed(&self) -> String {
        let mut details = self.summary();
        details.push_str("\n\nRooms:");
        for room in &self.rooms_visited {
            details.push_str(&format!("\n  - {}", room));
        }
        details.push_str("\n\nItems:");
        for item in &self.items_found {
            details.push_str(&format!("\n  - {}", item));
        }
        details.push_str("\n\nQuests Started:");
        for quest in &self.quests_started {
            details.push_str(&format!("\n  - {}", quest));
        }
        details.push_str("\n\nQuests Completed:");
        for quest in &self.quests_completed {
            details.push_str(&format!("\n  - {}", quest));
        }
        if !self.errors.is_empty() {
            details.push_str("\n\nErrors:");
            for error in &self.errors {
                details.push_str(&format!("\n  - {}", error));
            }
        }
        if !self.warnings.is_empty() {
            details.push_str("\n\nWarnings:");
            for warning in &self.warnings {
                details.push_str(&format!("\n  - {}", warning));
            }
        }
        details
    }
}
