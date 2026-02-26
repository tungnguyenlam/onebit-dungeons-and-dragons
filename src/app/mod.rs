use crate::data::loader::{load_global_assets, load_lore, load_monsters, load_quests, load_region};
use crate::data::types::{
    DialogTree, FeatDef, ItemDef, LoreEntry, MonsterDef, NpcDef, QuestDef, QuestKind,
    QuestStageDef, QuestTransition, RecipeDef, SpellDef,
};
use crate::game::{
    character::{progression::level_for_xp, AbilityScores, Character},
    combat::CombatantState,
    items::equipment::EquipmentSlot,
    save::{load_from_path, save_to_path, SaveGame, SAVE_FORMAT_VERSION},
    story::{
        events::{inspect_lore, EventEngine, EventTrigger, WorldEvent},
        journal::{Category as JournalCategory, Journal},
        quest::QuestLog,
        WorldState,
    },
    world::region::Region,
};
use crate::renderer::{ControlFlow, GameEvent, SoundEffect};
use anyhow::Result;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

pub mod combat;
pub mod handlers;
pub mod samples;
pub mod state;

// Submodules for App extensions
pub mod actions;
pub mod debug;
pub mod equipment;
pub mod navigation;
pub mod progression;
pub mod systems;

use samples::*;
pub use state::*;

/// Central application object.
pub struct App {
    pub state: AppState,
    pub player: Character,
    pub item_defs: HashMap<String, ItemDef>,
    pub spell_defs: HashMap<String, SpellDef>,
    pub monster_defs: HashMap<String, MonsterDef>,
    pub feat_defs: HashMap<String, FeatDef>,
    pub recipe_defs: HashMap<String, RecipeDef>,
    pub known_spells: Vec<String>,
    pub world_state: WorldState,
    pub journal: Journal,
    pub quests: QuestLog,
    pub world_events: EventEngine,
    pub lore_defs: HashMap<String, LoreEntry>,
    pub region: Region,
    pub region_npcs: HashMap<String, NpcDef>,
    pub region_dialogs: HashMap<String, DialogTree>,
    pub current_room_id: String,
    pub player_pos: (u32, u32),
    pub pending_encounter_monster: Option<String>,
    pub sound_enabled: bool,
    pub sound_queue: RefCell<Vec<SoundEffect>>,
    pub menu_ui: MainMenuUiState,
    pub char_creation_ui: CharacterCreationUiState,
    pub journal_ui: JournalUiState,
    pub settings_ui: SettingsUiState,
    pub settings: SettingsConfig,
    pub turn: u64,
    pub ending_scroll: u16,
    pub ng_plus_unlocked: bool,
    pub ng_plus_inherited_level: u8,
    pub ng_plus_inherited_xp: u32,
    pub focused_pane: FocusedPane,
    pub feedback_message: Option<(String, std::time::Instant)>,
    pub show_help: bool,
}

impl App {
    pub fn new() -> Self {
        let global_assets = load_global_assets("assets").ok();
        let item_defs = global_assets
            .as_ref()
            .map(|ga| ga.items.clone())
            .filter(|m| !m.is_empty())
            .unwrap_or_else(sample_item_defs);
        let spell_defs = global_assets
            .as_ref()
            .map(|ga| ga.spells.clone())
            .filter(|m| !m.is_empty())
            .unwrap_or_else(sample_spell_defs);
        let monster_defs = load_monsters("assets")
            .ok()
            .filter(|m| !m.is_empty())
            .unwrap_or_else(sample_monster_defs);
        let lore_defs = global_assets
            .as_ref()
            .map(|ga| ga.lore.clone())
            .unwrap_or_default();
        let feat_defs = global_assets
            .as_ref()
            .map(|ga| ga.feats.clone())
            .unwrap_or_default();
        let recipe_defs = global_assets
            .as_ref()
            .map(|ga| ga.recipes.clone())
            .unwrap_or_default();
        let (ng_plus_unlocked, ng_plus_inherited_level, ng_plus_inherited_xp) =
            load_from_path("save.toml")
                .ok()
                .map(|save| {
                    (
                        save.world_state.flag("game_completed"),
                        save.player.total_level,
                        save.player.xp,
                    )
                })
                .unwrap_or((false, 1, 0));

        let mut player = Character::new(
            "Theron".into(),
            "fighter".into(),
            "human".into(),
            AbilityScores {
                strength: 16,
                dexterity: 14,
                constitution: 14,
                intelligence: 10,
                wisdom: 12,
                charisma: 8,
            },
        );
        player.max_hp = 24;
        player.current_hp = 24;
        player.inventory.add("longsword", 1);
        player.inventory.add("leather_armor", 1);
        player.inventory.add("shield", 1);
        player.inventory.add("healing_potion", 3);
        player.spell_slots_max[0] = 2;
        player.spell_slots[0] = 2;

        let sample_quest = QuestDef {
            id: "demo_contract".into(),
            name: "Captain's Contract".into(),
            kind: QuestKind::Main,
            stages: vec![
                QuestStageDef {
                    id: "start".into(),
                    label: "Speak with the captain".into(),
                    condition: "".into(),
                    on_enter: vec![],
                    next: vec![QuestTransition {
                        condition: "flag:read_old_tablet".into(),
                        stage: "investigate".into(),
                    }],
                    journal_entry: "You accepted the captain's contract.".into(),
                },
                QuestStageDef {
                    id: "investigate".into(),
                    label: "Investigate the old tablet".into(),
                    condition: "flag:read_old_tablet".into(),
                    on_enter: vec![],
                    next: vec![QuestTransition {
                        condition: "flag:won_first_combat".into(),
                        stage: "DONE".into(),
                    }],
                    journal_entry: "The old tablet mentions a hidden vault.".into(),
                },
            ],
        };
        let quest_defs = load_quests("assets")
            .ok()
            .filter(|q| !q.is_empty())
            .map(|map| map.into_values().collect::<Vec<_>>())
            .unwrap_or_else(|| vec![sample_quest]);

        let (region, region_npcs, region_dialogs, current_room_id, player_pos) =
            if let Ok(loaded) = load_region("assets", "valley-of-ash") {
                let region = Region::from_loaded(&loaded);
                let room_id = region
                    .entry()
                    .map(|r| r.id.clone())
                    .unwrap_or_else(|| region.entry_room.clone());
                let spawn = find_spawn_pos_for_room(region.room(&room_id).unwrap());
                (region, loaded.npcs, loaded.dialogs, room_id, spawn)
            } else {
                let (region, npcs, dialogs) = sample_region_bundle();
                let room_id = region
                    .entry()
                    .map(|r| r.id.clone())
                    .unwrap_or_else(|| region.entry_room.clone());
                let spawn = find_spawn_pos_for_room(region.room(&room_id).unwrap());
                (region, npcs, dialogs, room_id, spawn)
            };

        Self {
            state: AppState::default(),
            player,
            item_defs,
            spell_defs,
            monster_defs,
            feat_defs,
            recipe_defs,
            known_spells: vec![
                "cure_wounds".into(),
                "fire_bolt".into(),
                "poison_spray".into(),
            ],
            world_state: WorldState::new(),
            journal: Journal::default(),
            quests: QuestLog::with_defs(quest_defs),
            world_events: demo_world_events(),
            lore_defs,
            region,
            region_npcs,
            region_dialogs,
            current_room_id,
            player_pos,
            pending_encounter_monster: None,
            sound_enabled: false,
            sound_queue: RefCell::new(Vec::new()),
            menu_ui: MainMenuUiState::default(),
            char_creation_ui: CharacterCreationUiState::default(),
            journal_ui: JournalUiState::default(),
            settings_ui: SettingsUiState::default(),
            settings: SettingsConfig::default(),
            turn: 0,
            ending_scroll: 0,
            ng_plus_unlocked,
            ng_plus_inherited_level,
            ng_plus_inherited_xp,
            focused_pane: FocusedPane::default(),
            feedback_message: None,
            show_help: false,
        }
    }

    pub fn set_feedback(&mut self, message: &str) {
        self.feedback_message = Some((message.to_string(), std::time::Instant::now()));
    }

    pub fn get_feedback(&self) -> Option<String> {
        if let Some((msg, time)) = &self.feedback_message {
            if time.elapsed().as_secs() < 3 {
                return Some(msg.clone());
            }
        }
        None
    }

    pub fn transition(&mut self, next: AppState) {
        self.state = next;
    }

    pub fn handle_event(&mut self, event: GameEvent) -> Result<ControlFlow> {
        match event {
            GameEvent::Quit => return Ok(ControlFlow::Exit),
            GameEvent::SaveGame => {
                self.save_to_default_path()?;
                return Ok(ControlFlow::Continue);
            }
            GameEvent::LoadGame => {
                self.load_from_default_path()?;
                return Ok(ControlFlow::Continue);
            }
            GameEvent::ToggleSound => {
                self.sound_enabled = !self.sound_enabled;
                if self.sound_enabled {
                    self.queue_sound(SoundEffect::Beep);
                }
                return Ok(ControlFlow::Continue);
            }
            GameEvent::OpenSettings => {
                self.transition(AppState::Settings);
                return Ok(ControlFlow::Continue);
            }

            GameEvent::Tick => {
                // Ticks are for real-time VFX updates only, they do not pass a turn.
            }
            other => self.dispatch(other)?,
        }
        Ok(ControlFlow::Continue)
    }

    pub fn pass_turn(&mut self) -> Result<()> {
        self.turn += 1;
        self.handle_tick()
    }

    fn handle_tick(&mut self) -> Result<()> {
        self.run_enemy_turns();
        self.finish_combat_if_over();
        self.tick_story_systems();
        Ok(())
    }

    fn dispatch(&mut self, event: GameEvent) -> Result<()> {
        match &self.state {
            AppState::MainMenu => self.handle_main_menu(event),
            AppState::WorldMap => self.handle_world_map(event),
            AppState::Combat(_) => self.handle_combat(event),
            AppState::Dialog(_) => self.handle_dialog(event),
            AppState::Inventory => self.handle_inventory(event),
            AppState::Crafting => self.handle_crafting(event),
            AppState::Bestiary => self.handle_bestiary(event),
            AppState::LoreLibrary => self.handle_lore_library(event),
            AppState::Ending => self.handle_ending(event),
            AppState::Journal => self.handle_journal(event),
            AppState::Spellbook => self.handle_spellbook(event),
            AppState::CharacterCreation => self.handle_char_creation(event),
            AppState::Settings => self.handle_settings(event),
            AppState::GameOver => self.handle_game_over(event),
        }
    }

    pub fn current_room(&self) -> Option<&crate::game::world::room::Room> {
        self.region.room(&self.current_room_id)
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod tests;
