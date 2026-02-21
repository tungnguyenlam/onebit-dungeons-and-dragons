use crate::data::loader::{load_global_assets, load_lore, load_monsters, load_quests, load_region};
use crate::data::types::{
    ArmorDef, ArmorType, DialogTree, ItemBonuses, ItemDef, ItemType, LoreEntry, MonsterAction,
    MonsterDef, NpcDef, QuestDef, QuestKind, QuestStageDef, QuestTransition, SpellDef, TriggerKind,
    WeaponDef,
};
use crate::game::{
    character::{
        progression::{
            class_hit_die, hp_on_level_up, is_asi_level, level_for_xp, spell_slots_for_class_level,
        },
        AbilityScores, Character,
    },
    combat::{
        apply_damage, can_cast, expend_slot, resolve_spell_effect, roll_attack, AttackProfile,
        CombatState, CombatantState, DefenseProfile, EnemyAiRole, HitType, RollMode, SpellEffect,
    },
    dice::DiceExpr,
    items::{armor::armor_class, equipment::EquipmentSlot},
    save::{load_from_path, save_to_path, SaveGame, SAVE_FORMAT_VERSION},
    story::{
        dialog::{choose as dialog_choose, resolve as dialog_resolve, ResolvedNode},
        events::{inspect_lore, EventEngine, EventTrigger, WorldEvent},
        journal::{Category as JournalCategory, Journal},
        quest::QuestLog,
        WorldState,
    },
    world::region::Region,
};
/// Application glue layer.
///
/// `App` owns the full mutable game state (`AppState`) and all game
/// sub-systems. It is renderer-agnostic — it has no direct dependency on
/// ratatui, crossterm, egui, or eframe.
///
/// The active renderer calls `App::handle_event` to drive state transitions
/// and reads `App::state` (and sub-system state) during rendering.
use crate::renderer::{ControlFlow, GameEvent, SoundEffect};
use anyhow::Result;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

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
    Settings,
    GameOver,
}

/// Placeholder — will be expanded in `src/game/combat/`.
#[derive(Debug, Clone)]
pub struct CombatContext {
    pub state: CombatState,
    pub world_state: WorldState,
    pub log: Vec<String>,
}

impl Default for CombatContext {
    fn default() -> Self {
        let state = CombatState::new_with_seed(
            vec![
                CombatantState::new(
                    "player",
                    "Theron",
                    true,
                    24,
                    16,
                    30,
                    2,
                    5,
                    DiceExpr::new(1, 8, 3),
                ),
                CombatantState::new(
                    "goblin_a",
                    "Goblin A",
                    false,
                    10,
                    13,
                    30,
                    2,
                    4,
                    DiceExpr::new(1, 6, 2),
                ),
                CombatantState::new(
                    "goblin_b",
                    "Goblin B",
                    false,
                    10,
                    13,
                    30,
                    2,
                    4,
                    DiceExpr::new(1, 6, 2),
                ),
            ],
            1337,
        );
        let mut state = state;
        if let Some(goblin_a) = state.combatants.get_mut("goblin_a") {
            goblin_a.on_hit_condition =
                Some(crate::game::character::conditions::Condition::Poisoned);
        }
        Self {
            state,
            world_state: WorldState::new(),
            log: vec![
                "Combat started.".into(),
                "Press 'a' to attack.".into(),
                "Press '.' to advance turn.".into(),
                "Press Esc to leave combat.".into(),
            ],
        }
    }
}

/// Placeholder — will be expanded in `src/game/story/dialog.rs`.
#[derive(Debug, Clone)]
pub struct DialogContext {
    pub npc_name: String,
    pub tree: DialogTree,
    pub current_node: String,
    pub resolved: ResolvedNode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JournalUiState {
    pub category: JournalCategory,
    pub selected: usize,
    pub detail_scroll: u16,
}

impl Default for JournalUiState {
    fn default() -> Self {
        Self {
            category: JournalCategory::Quest,
            selected: 0,
            detail_scroll: 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MainMenuUiState {
    pub selected: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct SettingsConfig {
    pub enemy_hp_multiplier: f32,
    pub player_damage_multiplier: f32,
    pub reduced_motion: bool,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        Self {
            enemy_hp_multiplier: 1.0,
            player_damage_multiplier: 1.0,
            reduced_motion: crate::ui::tui::theme::reduced_motion(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SettingsUiState {
    pub selected: usize,
}

#[derive(Debug, Clone)]
pub struct CharacterCreationUiState {
    pub selected: usize,
    pub name: String,
    pub class_options: Vec<String>,
    pub class_index: usize,
    pub race_options: Vec<String>,
    pub race_index: usize,
}

impl Default for CharacterCreationUiState {
    fn default() -> Self {
        Self {
            selected: 0,
            name: "Theron".into(),
            class_options: vec!["fighter".into(), "wizard".into(), "rogue".into()],
            class_index: 0,
            race_options: vec!["human".into(), "elf".into(), "dwarf".into()],
            race_index: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// App
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusedPane {
    #[default]
    Main,
    Side,
}

/// Central application object. Passed by shared reference to every renderer
/// `render()` call; mutated only inside `handle_event()`.
pub struct App {
    pub state: AppState,
    pub player: Character,
    pub item_defs: HashMap<String, ItemDef>,
    pub spell_defs: HashMap<String, SpellDef>,
    pub monster_defs: HashMap<String, MonsterDef>,
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
    pub focused_pane: FocusedPane,
}

impl App {
    /// Create a new `App` ready to display the main menu.
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
        let lore_defs = load_lore("assets").ok().unwrap_or_default();
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
                let spawn = region
                    .room(&room_id)
                    .map(spawn_pos_for_room)
                    .unwrap_or((1, 1));
                (region, loaded.npcs, loaded.dialogs, room_id, spawn)
            } else {
                let (region, npcs, dialogs) = sample_region_bundle();
                let room_id = region
                    .entry()
                    .map(|r| r.id.clone())
                    .unwrap_or_else(|| region.entry_room.clone());
                let spawn = region
                    .room(&room_id)
                    .map(spawn_pos_for_room)
                    .unwrap_or((1, 1));
                (region, npcs, dialogs, room_id, spawn)
            };

        Self {
            state: AppState::default(),
            player,
            item_defs,
            spell_defs,
            monster_defs,
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
            focused_pane: FocusedPane::default(),
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
                self.turn += 1;
                self.handle_tick()?;
            }

            // Route remaining events to the active screen handler.
            other => self.dispatch(other)?,
        }
        Ok(ControlFlow::Continue)
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
            AppState::Journal => self.handle_journal(event),
            AppState::Spellbook => self.handle_spellbook(event),
            AppState::CharacterCreation => self.handle_char_creation(event),
            AppState::Settings => self.handle_settings(event),
            AppState::GameOver => self.handle_game_over(event),
        }
    }

    // -----------------------------------------------------------------------
    // Per-screen handlers (stubs — will be filled in per milestone)
    // -----------------------------------------------------------------------

    fn handle_main_menu(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::MoveUp => {
                self.menu_ui.selected = self.menu_ui.selected.saturating_sub(1);
            }
            GameEvent::MoveDown => {
                self.menu_ui.selected = (self.menu_ui.selected + 1).min(3);
            }
            GameEvent::Confirm => match self.menu_ui.selected {
                0 => self.transition(AppState::CharacterCreation),
                1 => self.transition(AppState::WorldMap),
                2 => {
                    self.load_from_default_path()?;
                    self.transition(AppState::WorldMap);
                }
                3 => self.transition(AppState::GameOver),
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn handle_settings(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Back | GameEvent::Cancel | GameEvent::OpenSettings => {
                if self.player.current_hp > 0 {
                    self.transition(AppState::WorldMap);
                } else {
                    self.transition(AppState::MainMenu);
                }
            }
            GameEvent::MoveUp => {
                self.settings_ui.selected = self.settings_ui.selected.saturating_sub(1);
            }
            GameEvent::MoveDown => {
                let max_opts = 2; // 0, 1, 2
                if self.settings_ui.selected < max_opts {
                    self.settings_ui.selected += 1;
                }
            }
            GameEvent::MoveLeft => match self.settings_ui.selected {
                0 => {
                    self.settings.enemy_hp_multiplier =
                        (self.settings.enemy_hp_multiplier - 0.1).max(0.5)
                }
                1 => {
                    self.settings.player_damage_multiplier =
                        (self.settings.player_damage_multiplier - 0.1).max(0.5)
                }
                2 => self.settings.reduced_motion = !self.settings.reduced_motion,
                _ => {}
            },
            GameEvent::MoveRight | GameEvent::Confirm => match self.settings_ui.selected {
                0 => {
                    self.settings.enemy_hp_multiplier =
                        (self.settings.enemy_hp_multiplier + 0.1).min(2.0)
                }
                1 => {
                    self.settings.player_damage_multiplier =
                        (self.settings.player_damage_multiplier + 0.1).min(2.0)
                }
                2 => self.settings.reduced_motion = !self.settings.reduced_motion,
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn handle_world_map(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Attack => {
                let ctx = self.make_combat_context();
                self.transition(AppState::Combat(ctx));
            }
            GameEvent::OpenInventory => self.transition(AppState::Inventory),
            GameEvent::OpenSpellbook => self.transition(AppState::Spellbook),
            GameEvent::OpenJournal => {
                self.journal.mark_read();
                self.journal_ui.selected = 0;
                self.journal_ui.detail_scroll = 0;
                self.focused_pane = FocusedPane::Main;
                self.transition(AppState::Journal);
            }
            GameEvent::MoveUp => self.try_move_player(0, -1),
            GameEvent::MoveDown => self.try_move_player(0, 1),
            GameEvent::MoveLeft => self.try_move_player(-1, 0),
            GameEvent::MoveRight => self.try_move_player(1, 0),
            GameEvent::Confirm | GameEvent::OpenMap => self.interact_current_tile(),
            _ => {}
        }
        Ok(())
    }

    fn handle_combat(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Attack | GameEvent::Choice(1) => {
                if let AppState::Combat(ctx) = &mut self.state {
                    let Some(attacker_id) = ctx.state.current_combatant_id().map(str::to_string)
                    else {
                        Self::push_log(ctx, "No active combatant.");
                        return Ok(());
                    };

                    if !ctx
                        .state
                        .combatants
                        .get(&attacker_id)
                        .is_some_and(|c| c.is_player)
                    {
                        Self::push_log(ctx, "It's not the player's turn.");
                        return Ok(());
                    }

                    let Some(target_id) = ctx.state.next_enemy_id(&attacker_id).map(str::to_string)
                    else {
                        Self::push_log(ctx, "No valid target.");
                        return Ok(());
                    };
                    let _ = Self::resolve_attack(
                        ctx,
                        &attacker_id,
                        &target_id,
                        self.settings.player_damage_multiplier,
                    );
                }
                self.finish_combat_if_over();
            }
            GameEvent::Choice(2) => {
                if let AppState::Combat(ctx) = &mut self.state {
                    Self::use_potion_in_combat(ctx, &mut self.player);
                }
            }
            GameEvent::Choice(3) => {
                if let AppState::Combat(ctx) = &mut self.state {
                    Self::use_second_wind(ctx);
                }
            }
            GameEvent::Wait => {
                if let AppState::Combat(ctx) = &mut self.state {
                    let before = ctx
                        .state
                        .current_combatant()
                        .map(|c| c.name.clone())
                        .unwrap_or_else(|| "Unknown".into());
                    let after = Self::advance_turn(ctx);
                    Self::push_log(ctx, format!("{before} ends turn. {after} is up."));
                }
                self.run_enemy_turns();
                self.finish_combat_if_over();
            }
            GameEvent::Cancel | GameEvent::Back => self.transition(AppState::WorldMap),
            _ => {}
        }
        Ok(())
    }

    fn handle_dialog(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Cancel | GameEvent::Back => self.transition(AppState::WorldMap),
            GameEvent::Choice(n) => {
                let idx = n.saturating_sub(1) as usize;
                if let AppState::Dialog(ctx) = &mut self.state {
                    let Some(next) =
                        dialog_choose(&ctx.tree, &ctx.current_node, idx, &mut self.world_state)
                    else {
                        self.journal.append(
                            format!("dialog-blocked-{}-{}", ctx.tree.npc_id, self.turn),
                            self.turn,
                            JournalCategory::Dialog,
                            None,
                            format!("Talked with {}", ctx.npc_name),
                            "That option is unavailable right now.",
                        );
                        return Ok(());
                    };
                    if next == "END" {
                        self.transition(AppState::WorldMap);
                        return Ok(());
                    }
                    if let Some(resolved) = dialog_resolve(&ctx.tree, &next, &mut self.world_state)
                    {
                        ctx.current_node = next;
                        ctx.resolved = resolved;
                        self.journal.append(
                            format!("dialog-{}-{}", ctx.tree.npc_id, ctx.current_node),
                            self.turn,
                            JournalCategory::Dialog,
                            None,
                            format!("Talked with {}", ctx.npc_name),
                            ctx.resolved.text.clone(),
                        );
                    } else {
                        self.journal.append(
                            format!("dialog-broken-{}-{}", ctx.tree.npc_id, self.turn),
                            self.turn,
                            JournalCategory::Dialog,
                            None,
                            format!("Talked with {}", ctx.npc_name),
                            "Conversation path is blocked. Try another response or return later.",
                        );
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_inventory(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Back | GameEvent::Cancel => self.transition(AppState::WorldMap),
            GameEvent::Choice(1) => self.toggle_equip(EquipmentSlot::MainHand, "longsword"),
            GameEvent::Choice(2) => self.toggle_equip(EquipmentSlot::Armor, "leather_armor"),
            GameEvent::Choice(3) => self.toggle_equip(EquipmentSlot::OffHand, "shield"),
            GameEvent::Choice(4) => self.use_healing_potion(),
            _ => {}
        }
        Ok(())
    }

    fn handle_journal(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Back | GameEvent::Cancel => {
                if self.focused_pane == FocusedPane::Side {
                    self.focused_pane = FocusedPane::Main;
                } else {
                    self.transition(AppState::WorldMap);
                }
            }
            GameEvent::Confirm => {
                if self.focused_pane == FocusedPane::Main {
                    self.focused_pane = FocusedPane::Side;
                }
            }
            GameEvent::MoveUp => {
                if self.focused_pane == FocusedPane::Main {
                    self.journal_ui.selected = self.journal_ui.selected.saturating_sub(1);
                    self.journal_ui.detail_scroll = 0;
                } else {
                    self.journal_ui.detail_scroll = self.journal_ui.detail_scroll.saturating_sub(1);
                }
            }
            GameEvent::MoveDown => {
                if self.focused_pane == FocusedPane::Main {
                    self.journal_ui.selected = self.journal_ui.selected.saturating_add(1);
                    self.journal_ui.detail_scroll = 0;
                } else {
                    self.journal_ui.detail_scroll = self.journal_ui.detail_scroll.saturating_add(1);
                }
            }
            GameEvent::MoveLeft => {
                if self.focused_pane == FocusedPane::Side {
                    self.focused_pane = FocusedPane::Main;
                } else {
                    self.journal_ui.category = prev_category(self.journal_ui.category);
                    self.journal_ui.selected = 0;
                    self.journal_ui.detail_scroll = 0;
                }
            }
            GameEvent::MoveRight => {
                if self.focused_pane == FocusedPane::Main {
                    self.journal_ui.category = next_category(self.journal_ui.category);
                    self.journal_ui.selected = 0;
                    self.journal_ui.detail_scroll = 0;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_spellbook(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Back | GameEvent::Cancel => self.transition(AppState::WorldMap),
            GameEvent::Choice(n @ 1..=9) => self.cast_known_spell((n - 1) as usize),
            _ => {}
        }
        Ok(())
    }

    fn handle_char_creation(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::MoveUp => {
                self.char_creation_ui.selected = self.char_creation_ui.selected.saturating_sub(1);
            }
            GameEvent::MoveDown => {
                self.char_creation_ui.selected = (self.char_creation_ui.selected + 1).min(3);
            }
            GameEvent::MoveLeft => {
                if self.char_creation_ui.selected == 1 {
                    self.char_creation_ui.class_index =
                        self.char_creation_ui.class_index.saturating_sub(1);
                } else if self.char_creation_ui.selected == 2 {
                    self.char_creation_ui.race_index =
                        self.char_creation_ui.race_index.saturating_sub(1);
                }
            }
            GameEvent::MoveRight => {
                if self.char_creation_ui.selected == 1 {
                    self.char_creation_ui.class_index = (self.char_creation_ui.class_index + 1)
                        .min(self.char_creation_ui.class_options.len().saturating_sub(1));
                } else if self.char_creation_ui.selected == 2 {
                    self.char_creation_ui.race_index = (self.char_creation_ui.race_index + 1)
                        .min(self.char_creation_ui.race_options.len().saturating_sub(1));
                }
            }
            GameEvent::Choice(n @ 1..=9) => {
                if self.char_creation_ui.selected == 0 {
                    self.char_creation_ui.name.push(char::from(b'0' + n));
                }
            }
            GameEvent::Back | GameEvent::Cancel => self.transition(AppState::MainMenu),
            GameEvent::Confirm => {
                if self.char_creation_ui.selected == 3 {
                    self.apply_character_creation();
                    self.transition(AppState::WorldMap);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_game_over(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Confirm => self.transition(AppState::MainMenu),
            GameEvent::LoadGame => self.load_from_default_path()?,
            _ => {}
        }
        Ok(())
    }

    fn current_room(&self) -> Option<&crate::game::world::room::Room> {
        self.region.room(&self.current_room_id)
    }

    fn try_move_player(&mut self, dx: i32, dy: i32) {
        let Some(room) = self.current_room() else {
            return;
        };
        let next_col = self.player_pos.0 as i32 + dx;
        let next_row = self.player_pos.1 as i32 + dy;
        if room.grid.is_passable(next_col, next_row) {
            self.player_pos = (next_col as u32, next_row as u32);
        }
    }

    fn interact_current_tile(&mut self) {
        let trigger = self.current_room().and_then(|room| {
            room.trigger_at(self.player_pos.0, self.player_pos.1)
                .cloned()
        });
        if let Some(trigger) = trigger {
            match trigger.kind {
                TriggerKind::Dialog => {
                    self.start_dialog_with_npc(&trigger.target_id);
                }
                TriggerKind::Encounter => {
                    if trigger.target_id.contains("goblin")
                        && self.world_state.faction_rep("goblin_tribe") >= 2
                    {
                        self.journal.append(
                            format!("encounter-averted-{}-{}", trigger.target_id, self.turn),
                            self.turn,
                            JournalCategory::World,
                            None,
                            "Encounter Averted",
                            "The goblins recognize your standing and let you pass.",
                        );
                        return;
                    }
                    self.pending_encounter_monster = Some(trigger.target_id.clone());
                    self.queue_sound(SoundEffect::LowBeep);
                    let ctx = self.make_combat_context();
                    self.transition(AppState::Combat(ctx));
                }
                TriggerKind::Lore => {
                    if let Some(lore) = self.lore_defs.get(&trigger.target_id) {
                        if inspect_lore(lore, &mut self.world_state, &mut self.journal, self.turn) {
                            self.world_state.set_flag(format!("read_{}", lore.id));
                            if lore.id == "ash_tablet" {
                                self.world_state.set_flag("read_old_tablet");
                            }
                        }
                    }
                }
                TriggerKind::QuestStage => {
                    self.world_state.set_flag(trigger.target_id.clone());
                }
                TriggerKind::Travel => {
                    if self.region.room(&trigger.target_id).is_some() {
                        self.current_room_id = trigger.target_id.clone();
                        if let Some(new_room) = self.current_room() {
                            self.player_pos = spawn_pos_for_room(new_room);
                            self.check_room_hostilities();
                        }
                    }
                }
            }
            return;
        }

        let npc_id = self
            .current_room()
            .and_then(|room| room.npc_at(self.player_pos.0, self.player_pos.1))
            .map(|npc| npc.id.clone());
        if let Some(npc_id) = npc_id {
            self.start_dialog_with_npc(&npc_id);
        }
    }

    fn start_dialog_with_npc(&mut self, npc_id: &str) {
        if let Some(tree) = self.region_dialogs.get(npc_id).cloned() {
            let npc_name = self
                .region_npcs
                .get(npc_id)
                .map(|n| n.name.clone())
                .unwrap_or_else(|| npc_id.to_string());
            if let Some(resolved) = dialog_resolve(&tree, "root", &mut self.world_state) {
                self.transition(AppState::Dialog(DialogContext {
                    npc_name,
                    tree,
                    current_node: "root".into(),
                    resolved,
                }));
            } else {
                self.journal.append(
                    format!("dialog-root-missing-{}-{}", npc_id, self.turn),
                    self.turn,
                    JournalCategory::Dialog,
                    None,
                    "Conversation Unavailable",
                    "This conversation is currently unavailable due to invalid dialog data.",
                );
            }
        }
    }

    fn apply_character_creation(&mut self) {
        let class_id =
            self.char_creation_ui.class_options[self.char_creation_ui.class_index].clone();
        let race_id = self.char_creation_ui.race_options[self.char_creation_ui.race_index].clone();
        self.player.name = self.char_creation_ui.name.clone();
        self.player.class_id = class_id;
        self.player.race_id = race_id;
        self.player.level = 1;
        self.player.xp = 0;
        self.player.spell_slots_max =
            spell_slots_for_class_level(&self.player.class_id, self.player.level);
        self.player.spell_slots = self.player.spell_slots_max;
        self.player.current_hp = self.player.max_hp;
        self.current_room_id = self
            .region
            .entry()
            .map(|r| r.id.clone())
            .unwrap_or_else(|| self.region.entry_room.clone());
        if let Some(room) = self.current_room() {
            self.player_pos = spawn_pos_for_room(room);
        }
    }

    fn equipped_item_ids(&self) -> impl Iterator<Item = &str> {
        [
            self.player.equipment.main_hand.as_deref(),
            self.player.equipment.off_hand.as_deref(),
            self.player.equipment.armor.as_deref(),
            self.player.equipment.helmet.as_deref(),
            self.player.equipment.boots.as_deref(),
            self.player.equipment.ring_1.as_deref(),
            self.player.equipment.ring_2.as_deref(),
            self.player.equipment.amulet.as_deref(),
        ]
        .into_iter()
        .flatten()
    }

    fn equipment_bonus_totals(&self) -> (i32, i32, i32, i32, i32, i32) {
        // attack, damage, ac, spell_attack, spell_damage, max_hp
        self.equipped_item_ids()
            .fold((0, 0, 0, 0, 0, 0), |acc, id| {
                let Some(item) = self.item_defs.get(id) else {
                    return acc;
                };
                (
                    acc.0 + item.bonuses.attack_bonus,
                    acc.1 + item.bonuses.damage_bonus,
                    acc.2 + item.bonuses.armor_class_bonus,
                    acc.3 + item.bonuses.spell_attack_bonus,
                    acc.4 + item.bonuses.spell_damage_bonus,
                    acc.5 + item.bonuses.max_hp_bonus,
                )
            })
    }

    fn grant_player_xp(&mut self, gained_xp: u32) {
        if gained_xp == 0 {
            return;
        }
        let prev_level = self.player.level;
        self.player.xp = self.player.xp.saturating_add(gained_xp);
        let target_level = level_for_xp(self.player.xp);
        if target_level <= prev_level {
            self.journal.append(
                format!("xp-{}", self.turn),
                self.turn,
                JournalCategory::World,
                None,
                "XP Gained",
                format!("Gained {gained_xp} XP (total {}).", self.player.xp),
            );
            return;
        }

        self.queue_sound(SoundEffect::DoubleBeep);
        while self.player.level < target_level {
            let next_level = self.player.level + 1;
            let hit_die = class_hit_die(&self.player.class_id);
            let con_mod = self.player.scores.con_mod() as i32;
            let hp_gain = hp_on_level_up(hit_die, con_mod, next_level, false);
            self.player.level = next_level;
            self.player.max_hp += hp_gain;
            self.player.current_hp += hp_gain;
            self.player.spell_slots_max =
                spell_slots_for_class_level(&self.player.class_id, self.player.level);
            self.player.spell_slots = self.player.spell_slots_max;

            let asi_note = if is_asi_level(self.player.level) {
                " ASI available."
            } else {
                ""
            };
            self.journal.append(
                format!("level-up-{}-{}", self.player.level, self.turn),
                self.turn,
                JournalCategory::World,
                None,
                format!("Level Up: {}", self.player.level),
                format!("+{hp_gain} max HP.{asi_note}"),
            );
        }
        self.journal.append(
            format!("xp-{}", self.turn),
            self.turn,
            JournalCategory::World,
            None,
            "XP Gained",
            format!(
                "Gained {gained_xp} XP (total {}, level {} -> {}).",
                self.player.xp, prev_level, self.player.level
            ),
        );
    }

    fn save_to_default_path(&mut self) -> Result<()> {
        let save = SaveGame {
            format_version: SAVE_FORMAT_VERSION,
            turn: self.turn,
            player: self.player.clone(),
            world_state: self.world_state.clone(),
            journal: self.journal.clone(),
            region_slug: self.region.slug.clone(),
            room_id: self.current_room_id.clone(),
            player_pos: self.player_pos,
        };
        save_to_path("saves/slot1.toml", &save)?;
        self.journal.append(
            format!("save-{}", self.turn),
            self.turn,
            JournalCategory::World,
            None,
            "Game Saved",
            "Saved to saves/slot1.toml",
        );
        self.queue_sound(SoundEffect::Beep);
        Ok(())
    }

    fn load_from_default_path(&mut self) -> Result<()> {
        let save = match load_from_path("saves/slot1.toml") {
            Ok(save) => save,
            Err(_) => {
                self.journal.append(
                    format!("load-failed-{}", self.turn),
                    self.turn,
                    JournalCategory::World,
                    None,
                    "Load Failed",
                    "No save found at saves/slot1.toml",
                );
                return Ok(());
            }
        };
        self.turn = save.turn;
        self.player = save.player;
        self.world_state = save.world_state;
        self.journal = save.journal;

        if let Ok(loaded) = load_region("assets", &save.region_slug) {
            self.region = Region::from_loaded(&loaded);
            self.region_npcs = loaded.npcs;
            self.region_dialogs = loaded.dialogs;
        }
        self.current_room_id = if self.region.room(&save.room_id).is_some() {
            save.room_id
        } else {
            self.region
                .entry()
                .map(|r| r.id.clone())
                .unwrap_or_else(|| self.region.entry_room.clone())
        };
        self.player_pos = save.player_pos;
        self.queue_sound(SoundEffect::Beep);
        Ok(())
    }

    pub fn queue_sound(&self, effect: SoundEffect) {
        if self.sound_enabled {
            self.sound_queue.borrow_mut().push(effect);
        }
    }

    fn push_log(ctx: &mut CombatContext, line: impl Into<String>) {
        ctx.log.push(line.into());
        if ctx.log.len() > 64 {
            let keep_from = ctx.log.len() - 64;
            ctx.log.drain(0..keep_from);
        }
    }

    fn resolve_attack(
        ctx: &mut CombatContext,
        attacker_id: &str,
        target_id: &str,
        pdm: f32,
    ) -> bool {
        let Some(attacker_view) = ctx.state.combatants.get(attacker_id) else {
            Self::push_log(ctx, "Attacker not found.");
            return false;
        };

        if !attacker_view.can_take_actions() {
            let reasons = attacker_view
                .conditions
                .iter()
                .filter(|c| c.is_incapacitating())
                .map(|c| c.name())
                .collect::<Vec<_>>()
                .join(", ");
            Self::push_log(
                ctx,
                format!("{} cannot act ({reasons}).", attacker_view.name),
            );
            return false;
        }

        if !attacker_view.action_slots.action {
            Self::push_log(
                ctx,
                format!("{} has no action remaining this turn.", attacker_view.name),
            );
            return false;
        }

        let Some(target_view) = ctx.state.combatants.get(target_id) else {
            Self::push_log(ctx, "Target not found.");
            return false;
        };

        let attacker_name = attacker_view.name.clone();
        let target_name = target_view.name.clone();

        let outcome = {
            let attack = AttackProfile {
                id: &attacker_view.id,
                attack_bonus: attacker_view.attack_bonus,
                damage_dice: &attacker_view.damage_dice,
                conditions: &attacker_view.conditions,
                on_hit_condition: attacker_view.on_hit_condition.clone(),
            };
            let defense = DefenseProfile {
                id: &target_view.id,
                armor_class: target_view.armor_class,
                conditions: &target_view.conditions,
            };
            roll_attack(&attack, &defense, &ctx.world_state)
        };

        if let Some(attacker_mut) = ctx.state.combatants.get_mut(attacker_id) {
            let _ = attacker_mut.action_slots.use_action();
        }

        let mut final_dmg = outcome.damage;
        if final_dmg > 0 && attacker_id == "player" {
            final_dmg = (final_dmg as f32 * pdm) as u32;
        }

        let mut hp_after = None;
        if let Some(target_mut) = ctx.state.combatants.get_mut(target_id) {
            if final_dmg > 0 {
                hp_after = Some(apply_damage(target_mut, final_dmg));
            }
            if let Some(cond) = &outcome.inflicted_condition {
                target_mut.apply_condition(cond.clone(), Some(2));
            }
        }

        let roll_mode_suffix = match outcome.roll_mode {
            RollMode::Normal => "",
            RollMode::Advantage => " with advantage",
            RollMode::Disadvantage => " with disadvantage",
        };
        let headline = match outcome.hit_type {
            HitType::Miss => format!(
                "{attacker_name} attacks {target_name}{roll_mode_suffix}: miss (d20={} total={}).",
                outcome.d20, outcome.total,
            ),
            HitType::Hit => format!(
                "{attacker_name} hits {target_name}{roll_mode_suffix} for {} damage (d20={} total={}).",
                final_dmg, outcome.d20, outcome.total,
            ),
            HitType::Critical => format!(
                "{attacker_name} CRITS {target_name}{roll_mode_suffix} for {} damage (nat 20).",
                outcome.damage,
            ),
        };
        Self::push_log(ctx, headline);

        if let Some(hp) = hp_after {
            if hp == 0 {
                Self::push_log(ctx, format!("{target_name} drops to 0 HP."));
            } else {
                Self::push_log(ctx, format!("{target_name} now has {hp} HP."));
            }
        }

        if let Some(cond) = &outcome.inflicted_condition {
            Self::push_log(
                ctx,
                format!("{target_name} gains condition: {}.", cond.name()),
            );
        }
        true
    }

    fn run_enemy_turns(&mut self) {
        let player_damaged = {
            let AppState::Combat(ctx) = &mut self.state else {
                return;
            };
            let start_hp = ctx
                .state
                .combatants
                .get("player")
                .map_or(0, |c| c.current_hp);

            loop {
                if ctx.state.is_over() {
                    break;
                }

                let Some(attacker_id) = ctx.state.current_combatant_id().map(str::to_string) else {
                    break;
                };

                let Some(attacker) = ctx.state.combatants.get(&attacker_id) else {
                    break;
                };
                if attacker.is_player {
                    break;
                }

                if !attacker.can_take_actions() {
                    let reasons = attacker
                        .conditions
                        .iter()
                        .filter(|c| c.is_incapacitating())
                        .map(|c| c.name())
                        .collect::<Vec<_>>()
                        .join(", ");
                    let name = attacker.name.clone();
                    Self::push_log(
                        ctx,
                        format!("{name} cannot act ({reasons}) and skips turn."),
                    );
                    let _ = Self::advance_turn(ctx);
                    continue;
                }

                let Some(target_id) = ctx.state.next_enemy_id(&attacker_id).map(str::to_string)
                else {
                    Self::push_log(ctx, "Enemy has no valid target.");
                    break;
                };

                let actor = ctx.state.combatants.get(&attacker_id).cloned();
                let Some(actor) = actor else {
                    break;
                };

                let target_id = match actor.enemy_role {
                    EnemyAiRole::Melee => target_id,
                    EnemyAiRole::Ranged | EnemyAiRole::Spellcaster => {
                        Self::select_enemy_target(ctx, &attacker_id, true).unwrap_or(target_id)
                    }
                };

                match actor.enemy_role {
                    EnemyAiRole::Melee => {
                        let _ = Self::resolve_attack(ctx, &attacker_id, &target_id, 1.0);
                    }
                    EnemyAiRole::Ranged => {
                        if let (Some(bonus), Some(dice)) =
                            (actor.ranged_attack_bonus, actor.ranged_damage_dice)
                        {
                            let _ = Self::resolve_attack_with_stats(
                                ctx,
                                &attacker_id,
                                &target_id,
                                bonus,
                                dice,
                                None,
                                "ranged",
                            );
                        } else {
                            let _ = Self::resolve_attack(ctx, &attacker_id, &target_id, 1.0);
                        }
                    }
                    EnemyAiRole::Spellcaster => {
                        if Self::try_spellcaster_support_action(ctx, &attacker_id) {
                            let _ = Self::advance_turn(ctx);
                            continue;
                        }
                        if let (Some(bonus), Some(dice)) =
                            (actor.spell_attack_bonus, actor.spell_damage_dice)
                        {
                            let _ = Self::resolve_attack_with_stats(
                                ctx,
                                &attacker_id,
                                &target_id,
                                bonus,
                                dice,
                                actor.spell_on_hit_condition.clone(),
                                "spell",
                            );
                        } else {
                            let _ = Self::resolve_attack(ctx, &attacker_id, &target_id, 1.0);
                        }
                    }
                }
                let _ = Self::advance_turn(ctx);
            }

            let end_hp = ctx
                .state
                .combatants
                .get("player")
                .map_or(0, |c| c.current_hp);
            end_hp < start_hp
        };

        if player_damaged {
            self.queue_sound(SoundEffect::LowBeep);
        }
    }

    fn try_spellcaster_support_action(ctx: &mut CombatContext, attacker_id: &str) -> bool {
        let Some(attacker) = ctx.state.combatants.get(attacker_id).cloned() else {
            return false;
        };
        if attacker.enemy_role != EnemyAiRole::Spellcaster || !attacker.action_slots.action {
            return false;
        }

        let ally_to_heal = ctx
            .state
            .combatants
            .values()
            .filter(|c| c.is_alive() && c.is_player == attacker.is_player && c.id != attacker_id)
            .filter(|c| c.current_hp * 2 <= c.max_hp)
            .min_by_key(|c| c.current_hp)
            .map(|c| c.id.clone());
        let Some(ally_id) = ally_to_heal else {
            return false;
        };

        let heal = DiceExpr::new(1, 6, 2).roll().max(1);
        if let Some(ally) = ctx.state.combatants.get_mut(&ally_id) {
            ally.current_hp = (ally.current_hp + heal).min(ally.max_hp);
        }
        if let Some(attacker_mut) = ctx.state.combatants.get_mut(attacker_id) {
            let _ = attacker_mut.action_slots.use_action();
        }
        let caster_name = attacker.name;
        let ally_name = ctx
            .state
            .combatants
            .get(&ally_id)
            .map(|c| c.name.clone())
            .unwrap_or_else(|| ally_id.clone());
        Self::push_log(
            ctx,
            format!("{caster_name} casts a support spell on {ally_name}, restoring {heal} HP."),
        );
        true
    }

    fn resolve_attack_with_stats(
        ctx: &mut CombatContext,
        attacker_id: &str,
        target_id: &str,
        attack_bonus: i32,
        damage_dice: DiceExpr,
        on_hit_condition: Option<crate::game::character::conditions::Condition>,
        attack_label: &str,
    ) -> bool {
        let Some(attacker) = ctx.state.combatants.get(attacker_id) else {
            Self::push_log(ctx, "Attacker not found.");
            return false;
        };
        let Some(target) = ctx.state.combatants.get(target_id) else {
            Self::push_log(ctx, "Target not found.");
            return false;
        };

        let attacker_name = attacker.name.clone();
        let target_name = target.name.clone();
        let outcome = {
            let attack = AttackProfile {
                id: &attacker.id,
                attack_bonus,
                damage_dice: &damage_dice,
                conditions: &attacker.conditions,
                on_hit_condition,
            };
            let defense = DefenseProfile {
                id: &target.id,
                armor_class: target.armor_class,
                conditions: &target.conditions,
            };
            roll_attack(&attack, &defense, &ctx.world_state)
        };

        if let Some(attacker_mut) = ctx.state.combatants.get_mut(attacker_id) {
            let _ = attacker_mut.action_slots.use_action();
        }

        let mut hp_after = None;
        if let Some(target_mut) = ctx.state.combatants.get_mut(target_id) {
            if outcome.damage > 0 {
                hp_after = Some(apply_damage(target_mut, outcome.damage));
            }
            if let Some(cond) = &outcome.inflicted_condition {
                target_mut.apply_condition(cond.clone(), Some(2));
            }
        }

        let verb = match attack_label {
            "spell" => "casts at",
            "ranged" => "shoots",
            _ => "attacks",
        };
        let headline = match outcome.hit_type {
            HitType::Miss => format!(
                "{attacker_name} {verb} {target_name}: miss (d20={} total={}).",
                outcome.d20, outcome.total
            ),
            HitType::Hit => format!(
                "{attacker_name} {verb} {target_name} for {} damage (d20={} total={}).",
                outcome.damage, outcome.d20, outcome.total
            ),
            HitType::Critical => format!(
                "{attacker_name} {verb} {target_name} for {} critical damage (nat 20).",
                outcome.damage
            ),
        };
        Self::push_log(ctx, headline);

        if let Some(hp) = hp_after {
            if hp == 0 {
                Self::push_log(ctx, format!("{target_name} drops to 0 HP."));
            } else {
                Self::push_log(ctx, format!("{target_name} now has {hp} HP."));
            }
        }
        true
    }

    fn select_enemy_target(
        ctx: &CombatContext,
        attacker_id: &str,
        prefer_low_hp: bool,
    ) -> Option<String> {
        let attacker = ctx.state.combatants.get(attacker_id)?;
        let mut candidates: Vec<&CombatantState> = ctx
            .state
            .combatants
            .values()
            .filter(|c| c.is_alive() && c.is_player != attacker.is_player)
            .collect();
        if candidates.is_empty() {
            return None;
        }
        if prefer_low_hp {
            candidates.sort_by_key(|c| c.current_hp);
            return Some(candidates[0].id.clone());
        }
        ctx.state.next_enemy_id(attacker_id).map(str::to_string)
    }

    fn advance_turn(ctx: &mut CombatContext) -> String {
        let leaving_name = ctx
            .state
            .current_combatant()
            .map(|c| c.name.clone())
            .unwrap_or_else(|| "Unknown".into());
        let (_, expired, _) = ctx.state.advance_turn_with_condition_tick();
        for cond in expired {
            Self::push_log(ctx, format!("{leaving_name}'s {} expired.", cond.name()));
        }
        ctx.state
            .current_combatant()
            .map(|c| c.name.clone())
            .unwrap_or_else(|| "Unknown".into())
    }

    fn finish_combat_if_over(&mut self) {
        let Some((is_over, players_alive, player_hp, ws, gained_xp)) = (match &self.state {
            AppState::Combat(ctx) => Some((
                ctx.state.is_over(),
                ctx.state
                    .combatants
                    .values()
                    .any(|c| c.is_player && c.is_alive()),
                ctx.state.combatants.get("player").map(|c| c.current_hp),
                ctx.world_state.clone(),
                ctx.state
                    .combatants
                    .values()
                    .filter(|c| !c.is_player && c.current_hp <= 0)
                    .filter_map(|c| {
                        c.id.split('_')
                            .next()
                            .and_then(|mid| self.monster_defs.get(mid))
                            .map(|m| m.xp)
                    })
                    .sum::<u32>(),
            )),
            _ => None,
        }) else {
            return;
        };

        if !is_over {
            return;
        }
        if let Some(hp) = player_hp {
            self.player.current_hp = hp;
        }
        self.world_state = ws;

        if players_alive {
            self.grant_player_xp(gained_xp);
            self.world_state.set_flag("won_first_combat");
            self.modify_faction_rep("town_guard", 1);
            self.modify_faction_rep("goblin_tribe", -2);
            self.transition(AppState::WorldMap);
        } else {
            self.modify_faction_rep("town_guard", -1);
            self.transition(AppState::GameOver);
        }
    }

    pub fn modify_faction_rep(&mut self, faction: &str, delta: i32) {
        let old_rep = self.world_state.faction_rep(faction);
        let new_rep = self.world_state.modify_faction_rep(faction, delta);
        if delta.abs() >= 5 {
            let label = if delta > 0 { "improved" } else { "declined" };
            self.journal.append(
                format!("faction-rep-{}-{}-{}", faction, self.turn, delta),
                self.turn,
                JournalCategory::World,
                None,
                format!("Reputation {}", label),
                format!(
                    "Your standing with {} has {} ({} -> {}).",
                    faction, label, old_rep, new_rep
                ),
            );
        }
    }

    pub fn check_room_hostilities(&mut self) {
        let Some(room) = self.current_room() else {
            return;
        };
        let hostile_threshold = -10;
        let mut hostile_npc = None;
        for npc_ref in &room.npcs {
            if let Some(npc_def) = self.region_npcs.get(&npc_ref.id) {
                if !npc_def.faction.is_empty() {
                    let rep = self.world_state.faction_rep(&npc_def.faction);
                    if rep <= hostile_threshold {
                        hostile_npc = Some(npc_def.id.clone());
                        break;
                    }
                }
            }
        }

        if let Some(npc_id) = hostile_npc {
            self.pending_encounter_monster = self
                .region_npcs
                .get(&npc_id)
                .map(|n| n.monster_ref.clone())
                .filter(|m| !m.is_empty());
            if self.pending_encounter_monster.is_some() {
                let ctx = self.make_combat_context();
                self.transition(AppState::Combat(ctx));
                self.journal.append(
                    format!("hostile-intercept-{}-{}", npc_id, self.turn),
                    self.turn,
                    JournalCategory::World,
                    None,
                    "Hostile Intercept",
                    "An NPC from a hostile faction has intercepted you!".to_string(),
                );
            }
        }
    }

    fn tick_story_systems(&mut self) {
        if self.world_state.flag("accept_demo_quest")
            && !self.quests.states.contains_key("demo_contract")
        {
            self.quests.accept_quest(
                "demo_contract",
                &mut self.world_state,
                &mut self.journal,
                self.turn,
            );
        }

        if self.world_state.flag("accept_emberpeak_rune_task")
            && !self.quests.states.contains_key("volcanic_curse")
        {
            self.quests.accept_quest(
                "volcanic_curse",
                &mut self.world_state,
                &mut self.journal,
                self.turn,
            );
        }

        let quest_ids: Vec<String> = self.quests.states.keys().cloned().collect();
        for q in quest_ids {
            let was_completed = matches!(
                self.quests.states.get(&q),
                Some(crate::game::story::quest::QuestStatus::Completed { .. })
            );
            let _ = self
                .quests
                .tick_quest(&q, &mut self.world_state, &mut self.journal, self.turn);
            let is_completed = matches!(
                self.quests.states.get(&q),
                Some(crate::game::story::quest::QuestStatus::Completed { .. })
            );
            if !was_completed && is_completed {
                self.queue_sound(SoundEffect::HighBeep);
                self.queue_sound(SoundEffect::LowBeep);
            }
        }
        self.world_events
            .tick(&mut self.world_state, &mut self.journal, self.turn);

        // Ambient trigger
        if self.turn.is_multiple_of(20) && !self.region.ambient.is_empty() {
            self.journal.append(
                format!("ambient-{}-{}", self.region.ambient, self.turn),
                self.turn,
                JournalCategory::World,
                None,
                "Atmosphere",
                format!("{} drifts through the air...", self.region.ambient),
            );
        }

        // M25: Inter-faction relationship event
        if self.world_state.faction_rep("goblin_tribe") < -5
            && self.world_state.faction_rep("town_guard") > 5
            && !self.world_state.flag("town_guard_vouched")
        {
            self.world_state.set_flag("town_guard_vouched");
            self.journal.append(
                format!("faction-event-vouched-{}", self.turn),
                self.turn,
                JournalCategory::World,
                None,
                "Town Guard Vouched",
                "Your hostility towards the goblins has earned you special favor with the Town Guard. New dialog options may be available.",
            );
        }
    }

    fn toggle_equip(&mut self, slot: EquipmentSlot, item_id: &str) {
        let currently = match slot {
            EquipmentSlot::MainHand => self.player.equipment.main_hand.as_deref() == Some(item_id),
            EquipmentSlot::OffHand => self.player.equipment.off_hand.as_deref() == Some(item_id),
            EquipmentSlot::Armor => self.player.equipment.armor.as_deref() == Some(item_id),
            EquipmentSlot::Helmet => self.player.equipment.helmet.as_deref() == Some(item_id),
            EquipmentSlot::Boots => self.player.equipment.boots.as_deref() == Some(item_id),
            EquipmentSlot::Ring1 => self.player.equipment.ring_1.as_deref() == Some(item_id),
            EquipmentSlot::Ring2 => self.player.equipment.ring_2.as_deref() == Some(item_id),
            EquipmentSlot::Amulet => self.player.equipment.amulet.as_deref() == Some(item_id),
        };

        if currently {
            self.player.equipment.unequip(slot);
            self.player.inventory.set_equipped(item_id, false);
            return;
        }
        if self.player.inventory.count(item_id) == 0 {
            return;
        }
        if let Some(prev) = self.player.equipment.equip(slot, item_id.to_string()) {
            self.player.inventory.set_equipped(&prev, false);
        }
        self.player.inventory.set_equipped(item_id, true);
    }

    fn use_healing_potion(&mut self) {
        if !self.player.inventory.use_one("healing_potion") {
            return;
        }
        self.player.heal(8);
        self.journal.append(
            format!("item-heal-{}", self.turn),
            self.turn,
            JournalCategory::World,
            None,
            "Used Healing Potion",
            "You recover 8 HP.",
        );
    }

    fn use_potion_in_combat(ctx: &mut CombatContext, player: &mut Character) {
        let Some(actor_id) = ctx.state.current_combatant_id().map(str::to_string) else {
            Self::push_log(ctx, "No active combatant.");
            return;
        };
        if actor_id != "player" {
            Self::push_log(ctx, "It's not the player's turn.");
            return;
        }
        let Some(actor) = ctx.state.combatants.get_mut(&actor_id) else {
            return;
        };
        if !actor.action_slots.action {
            Self::push_log(ctx, "No action remaining.");
            return;
        }
        if !player.inventory.use_one("healing_potion") {
            Self::push_log(ctx, "No healing potion available.");
            return;
        }
        let _ = actor.action_slots.use_action();
        player.heal(8);
        actor.current_hp = player.current_hp;
        Self::push_log(ctx, "Player drinks a healing potion and recovers 8 HP.");
    }

    fn use_second_wind(ctx: &mut CombatContext) {
        let Some(actor_id) = ctx.state.current_combatant_id().map(str::to_string) else {
            Self::push_log(ctx, "No active combatant.");
            return;
        };
        if actor_id != "player" {
            Self::push_log(ctx, "It's not the player's turn.");
            return;
        }
        let Some(actor) = ctx.state.combatants.get_mut(&actor_id) else {
            return;
        };
        if !actor.action_slots.bonus_action {
            Self::push_log(ctx, "No bonus action remaining.");
            return;
        }
        let heal = DiceExpr::new(1, 10, actor.max_hp.min(20) / 10)
            .roll()
            .max(1);
        let _ = actor.action_slots.use_bonus_action();
        actor.current_hp = (actor.current_hp + heal).min(actor.max_hp);
        Self::push_log(
            ctx,
            format!("Player uses Second Wind and recovers {heal} HP."),
        );
    }

    fn cast_known_spell(&mut self, idx: usize) {
        let Some(spell_id) = self.known_spells.get(idx).cloned() else {
            return;
        };
        let Some(spell) = self.spell_defs.get(&spell_id).cloned() else {
            return;
        };

        let slot_level = if spell.level == 0 {
            None
        } else {
            (spell.level..=9)
                .rev()
                .find(|lvl| self.player.spell_slots[(*lvl - 1) as usize] > 0)
        };
        if !can_cast(&spell, &self.player.spell_slots, slot_level) {
            self.journal.append(
                format!("spell-fail-{}-{}", spell.id, self.turn),
                self.turn,
                JournalCategory::World,
                None,
                format!("Failed to cast {}", spell.name),
                "Not enough spell slots.",
            );
            return;
        }
        if let Some(level) = slot_level {
            let _ = expend_slot(&mut self.player.spell_slots, level);
        }
        let (_, _, _, _, spell_damage_bonus, _) = self.equipment_bonus_totals();

        match resolve_spell_effect(&spell, slot_level, self.player.level, spell_damage_bonus) {
            Some(SpellEffect::Heal { amount }) => {
                self.player.heal(amount);
                self.journal.append(
                    format!("spell-{}-{}", spell.id, self.turn),
                    self.turn,
                    JournalCategory::World,
                    None,
                    format!("Cast {}", spell.name),
                    format!(
                        "You recover {amount} HP{}.",
                        slot_level
                            .map(|lvl| format!(" (slot {lvl})"))
                            .unwrap_or_default()
                    ),
                );
            }
            Some(SpellEffect::Damage {
                amount,
                damage_type,
            }) => {
                self.world_state
                    .set_counter("last_spell_damage", amount as i32);
                self.journal.append(
                    format!("spell-{}-{}", spell.id, self.turn),
                    self.turn,
                    JournalCategory::World,
                    None,
                    format!("Cast {}", spell.name),
                    format!(
                        "Spell deals {amount} {damage_type} damage{}.",
                        slot_level
                            .map(|lvl| format!(" (slot {lvl})"))
                            .unwrap_or_default()
                    ),
                );
            }
            Some(SpellEffect::Condition { condition }) => {
                self.world_state.set_flag(format!(
                    "spell_inflicted_{}",
                    condition.name().to_lowercase()
                ));
                self.journal.append(
                    format!("spell-{}-{}", spell.id, self.turn),
                    self.turn,
                    JournalCategory::World,
                    None,
                    format!("Cast {}", spell.name),
                    format!("Spell may inflict {}.", condition.name()),
                );
            }
            None => {
                self.journal.append(
                    format!("spell-{}-{}", spell.id, self.turn),
                    self.turn,
                    JournalCategory::World,
                    None,
                    format!("Cast {}", spell.name),
                    "No direct gameplay effect resolved.",
                );
            }
        }
    }

    fn make_combat_context(&mut self) -> CombatContext {
        let (attack_bonus_bonus, damage_bonus, ac_bonus, _, _, max_hp_bonus) =
            self.equipment_bonus_totals();
        let armor_item = self
            .player
            .equipment
            .armor
            .as_ref()
            .and_then(|id| self.item_defs.get(id))
            .and_then(|it| it.armor.as_ref().map(|a| (a.base_ac, &a.armor_type)));
        let shield_equipped = self
            .player
            .equipment
            .off_hand
            .as_ref()
            .and_then(|id| self.item_defs.get(id))
            .and_then(|it| it.armor.as_ref())
            .is_some_and(|a| a.armor_type == ArmorType::Shield);
        let ac = armor_class(armor_item, shield_equipped, self.player.scores.dex_mod()) + ac_bonus;

        let main_weapon = self
            .player
            .equipment
            .main_hand
            .as_ref()
            .and_then(|id| self.item_defs.get(id))
            .and_then(|it| it.weapon.as_ref())
            .cloned();
        let weapon_uses_dex = main_weapon
            .as_ref()
            .is_some_and(|w| w.properties.iter().any(|p| p == "finesse" || p == "ranged"));
        let ability_mod = if weapon_uses_dex {
            self.player.scores.dex_mod() as i32
        } else {
            self.player.scores.str_mod() as i32
        };
        let versatile = main_weapon.as_ref().is_some_and(|w| {
            w.properties.iter().any(|p| p == "versatile")
                && self.player.equipment.off_hand.is_none()
                && w.versatile_damage.is_some()
        });
        let mut damage = if versatile {
            main_weapon
                .as_ref()
                .and_then(|w| w.versatile_damage.clone())
                .unwrap_or_else(|| DiceExpr::new(1, 4, 0))
        } else {
            main_weapon
                .as_ref()
                .map(|w| w.damage.clone())
                .unwrap_or_else(|| DiceExpr::new(1, 4, 0))
        };
        damage.modifier += ability_mod + damage_bonus;
        let attack_bonus = ability_mod + self.player.proficiency_bonus() + attack_bonus_bonus;

        let mut combatants = vec![CombatantState::new(
            "player",
            self.player.name.clone(),
            true,
            self.player.max_hp + max_hp_bonus,
            ac,
            self.player.speed,
            self.player.scores.dex_mod() as i32,
            attack_bonus,
            damage,
        )];
        if self.world_state.faction_rep("town_guard") >= 3
            || self.world_state.flag("request_guard_support")
        {
            let mut ally = CombatantState::new(
                "ally_guard",
                "Guard Ally",
                true,
                16,
                15,
                30,
                1,
                4,
                DiceExpr::new(1, 8, 2),
            );
            ally.enemy_role = EnemyAiRole::Melee;
            combatants.push(ally);
            self.world_state.clear_flag("request_guard_support");
        }
        let queued = self.pending_encounter_monster.take();
        combatants.extend(self.build_encounter_monsters(queued.as_deref()));

        let mut state = CombatState::new_with_seed(combatants, 1337);
        if let Some(player) = state.combatants.get_mut("player") {
            player.current_hp = self.player.current_hp;
        }
        CombatContext {
            state,
            world_state: self.world_state.clone(),
            log: vec![
                "Combat started.".into(),
                "Press 'a' to attack.".into(),
                "Press '.' to advance turn.".into(),
                "Press Esc to leave combat.".into(),
            ],
        }
    }

    fn build_encounter_monsters(&self, queued_monster: Option<&str>) -> Vec<CombatantState> {
        let mut out = Vec::new();
        let ids = if let Some(monster) = queued_monster {
            vec![monster]
        } else if self.world_state.faction_rep("goblin_tribe") <= -2 {
            vec!["goblin", "goblin_archer", "goblin_shaman"]
        } else {
            vec!["goblin", "goblin"]
        };

        let mut seq: HashMap<&str, usize> = HashMap::new();
        for mid in ids {
            let Some(def) = self.monster_defs.get(mid) else {
                continue;
            };
            let n = seq.entry(mid).and_modify(|v| *v += 1).or_insert(1);
            let cid = format!("{}_{}", mid, *n);
            out.push(combatant_from_monster(
                &cid,
                def,
                self.settings.enemy_hp_multiplier,
            ));
        }
        out
    }
}

fn next_category(c: JournalCategory) -> JournalCategory {
    match c {
        JournalCategory::Quest => JournalCategory::Lore,
        JournalCategory::Lore => JournalCategory::World,
        JournalCategory::World => JournalCategory::Combat,
        JournalCategory::Combat => JournalCategory::Dialog,
        JournalCategory::Dialog => JournalCategory::System,
        JournalCategory::System => JournalCategory::Quest,
    }
}

fn prev_category(c: JournalCategory) -> JournalCategory {
    match c {
        JournalCategory::Quest => JournalCategory::System,
        JournalCategory::Lore => JournalCategory::Quest,
        JournalCategory::World => JournalCategory::Lore,
        JournalCategory::Combat => JournalCategory::World,
        JournalCategory::Dialog => JournalCategory::Combat,
        JournalCategory::System => JournalCategory::Dialog,
    }
}

fn spawn_pos_for_room(room: &crate::game::world::room::Room) -> (u32, u32) {
    if let Some((col, row, _)) = room
        .grid
        .iter()
        .find(|(_, _, tile)| *tile == crate::game::world::map::Tile::NpcSpawn)
    {
        return (col, row);
    }
    for row in 0..room.height() {
        for col in 0..room.width() {
            if room.grid.is_passable(col as i32, row as i32) {
                return (col, row);
            }
        }
    }
    (1, 1)
}

fn sample_region_bundle() -> (Region, HashMap<String, NpcDef>, HashMap<String, DialogTree>) {
    let loaded = load_region("assets", "valley-of-ash").ok();
    if let Some(loaded) = loaded {
        return (Region::from_loaded(&loaded), loaded.npcs, loaded.dialogs);
    }

    let fallback = crate::data::loader::LoadedRegion {
        manifest: crate::data::types::RegionManifest {
            slug: "fallback".into(),
            name: "Fallback Region".into(),
            description: "Fallback region when assets are unavailable.".into(),
            entry_room: "start".into(),
            ambient: "".into(),
            rooms: vec![crate::data::types::RoomRef {
                id: "start".into(),
                file: "rooms/start.toml".into(),
            }],
            connections: vec![],
        },
        rooms: {
            let mut map = HashMap::new();
            map.insert(
                "start".into(),
                crate::data::types::RoomDef {
                    id: "start".into(),
                    name: "Start".into(),
                    description: "Fallback room".into(),
                    grid: "#####\n#...#\n#.@.#\n#####\n".into(),
                    terminal: false,
                    npcs: vec![],
                    items: vec![],
                    triggers: vec![],
                },
            );
            map
        },
        npcs: HashMap::new(),
        dialogs: HashMap::new(),
    };
    (
        Region::from_loaded(&fallback),
        HashMap::new(),
        HashMap::new(),
    )
}

fn sample_item_defs() -> HashMap<String, ItemDef> {
    let mut map = HashMap::new();
    map.insert(
        "longsword".into(),
        ItemDef {
            id: "longsword".into(),
            name: "Longsword".into(),
            item_type: ItemType::Weapon,
            weight: 3.0,
            value_gp: 15,
            description: "A standard steel longsword.".into(),
            weapon: Some(WeaponDef {
                damage: DiceExpr::new(1, 8, 0),
                damage_type: "slashing".into(),
                properties: vec!["versatile".into()],
                versatile_damage: Some(DiceExpr::new(1, 10, 0)),
                range: None,
            }),
            armor: None,
            bonuses: ItemBonuses::default(),
        },
    );
    map.insert(
        "leather_armor".into(),
        ItemDef {
            id: "leather_armor".into(),
            name: "Leather Armor".into(),
            item_type: ItemType::Armor,
            weight: 10.0,
            value_gp: 10,
            description: "Flexible light armor.".into(),
            weapon: None,
            armor: Some(ArmorDef {
                base_ac: 11,
                armor_type: ArmorType::Light,
                stealth_disadvantage: false,
            }),
            bonuses: ItemBonuses::default(),
        },
    );
    map.insert(
        "shield".into(),
        ItemDef {
            id: "shield".into(),
            name: "Shield".into(),
            item_type: ItemType::Armor,
            weight: 6.0,
            value_gp: 10,
            description: "Wooden shield.".into(),
            weapon: None,
            armor: Some(ArmorDef {
                base_ac: 2,
                armor_type: ArmorType::Shield,
                stealth_disadvantage: false,
            }),
            bonuses: ItemBonuses {
                armor_class_bonus: 0,
                ..ItemBonuses::default()
            },
        },
    );
    map.insert(
        "healing_potion".into(),
        ItemDef {
            id: "healing_potion".into(),
            name: "Healing Potion".into(),
            item_type: ItemType::Consumable,
            weight: 0.5,
            value_gp: 50,
            description: "Restores health.".into(),
            weapon: None,
            armor: None,
            bonuses: ItemBonuses::default(),
        },
    );
    map
}

fn sample_spell_defs() -> HashMap<String, SpellDef> {
    let mut map = HashMap::new();
    map.insert(
        "cure_wounds".into(),
        SpellDef {
            id: "cure_wounds".into(),
            name: "Cure Wounds".into(),
            level: 1,
            school: "evocation".into(),
            casting_time: "action".into(),
            range: "touch".into(),
            components: vec!["V".into(), "S".into()],
            duration: "instantaneous".into(),
            description: "Healing energy restores HP.".into(),
            damage: None,
            damage_type: None,
            save: None,
            heal: Some(DiceExpr::new(1, 8, 2)),
            classes: vec!["cleric".into()],
        },
    );
    map.insert(
        "fire_bolt".into(),
        SpellDef {
            id: "fire_bolt".into(),
            name: "Fire Bolt".into(),
            level: 0,
            school: "evocation".into(),
            casting_time: "action".into(),
            range: "120ft".into(),
            components: vec!["V".into(), "S".into()],
            duration: "instantaneous".into(),
            description: "A mote of fire.".into(),
            damage: Some(DiceExpr::new(1, 10, 0)),
            damage_type: Some("fire".into()),
            save: None,
            heal: None,
            classes: vec!["wizard".into()],
        },
    );
    map.insert(
        "poison_spray".into(),
        SpellDef {
            id: "poison_spray".into(),
            name: "Poison Spray".into(),
            level: 0,
            school: "conjuration".into(),
            casting_time: "action".into(),
            range: "10ft".into(),
            components: vec!["V".into(), "S".into()],
            duration: "instantaneous".into(),
            description: "Noxious gas.".into(),
            damage: None,
            damage_type: None,
            save: Some("constitution".into()),
            heal: None,
            classes: vec!["wizard".into()],
        },
    );
    map
}

fn sample_monster_defs() -> HashMap<String, MonsterDef> {
    let mut map = HashMap::new();
    map.insert(
        "goblin".into(),
        MonsterDef {
            id: "goblin".into(),
            name: "Goblin".into(),
            cr: 0.25,
            size: "small".into(),
            monster_type: "humanoid".into(),
            alignment: "neutral_evil".into(),
            hp: DiceExpr::new(2, 6, 0),
            ac: 13,
            speed: 30,
            str_score: 8,
            dex_score: 14,
            con_score: 10,
            int_score: 10,
            wis_score: 8,
            cha_score: 8,
            xp: 50,
            actions: vec![MonsterAction {
                name: "Scimitar".into(),
                description: "Melee attack".into(),
                attack_bonus: Some(4),
                damage: Some(DiceExpr::new(1, 6, 2)),
                damage_type: Some("slashing".into()),
            }],
            traits: vec![],
        },
    );
    map.insert(
        "goblin_archer".into(),
        MonsterDef {
            id: "goblin_archer".into(),
            name: "Goblin Archer".into(),
            cr: 0.25,
            size: "small".into(),
            monster_type: "humanoid".into(),
            alignment: "neutral_evil".into(),
            hp: DiceExpr::new(2, 6, 0),
            ac: 13,
            speed: 30,
            str_score: 8,
            dex_score: 14,
            con_score: 10,
            int_score: 10,
            wis_score: 8,
            cha_score: 8,
            xp: 50,
            actions: vec![
                MonsterAction {
                    name: "Scimitar".into(),
                    description: "Melee attack".into(),
                    attack_bonus: Some(4),
                    damage: Some(DiceExpr::new(1, 6, 2)),
                    damage_type: Some("slashing".into()),
                },
                MonsterAction {
                    name: "Shortbow".into(),
                    description: "Ranged attack".into(),
                    attack_bonus: Some(4),
                    damage: Some(DiceExpr::new(1, 6, 2)),
                    damage_type: Some("piercing".into()),
                },
            ],
            traits: vec![],
        },
    );
    map.insert(
        "goblin_shaman".into(),
        MonsterDef {
            id: "goblin_shaman".into(),
            name: "Goblin Shaman".into(),
            cr: 0.5,
            size: "small".into(),
            monster_type: "humanoid".into(),
            alignment: "neutral_evil".into(),
            hp: DiceExpr::new(3, 6, 0),
            ac: 12,
            speed: 30,
            str_score: 8,
            dex_score: 12,
            con_score: 10,
            int_score: 12,
            wis_score: 14,
            cha_score: 10,
            xp: 100,
            actions: vec![
                MonsterAction {
                    name: "Dagger".into(),
                    description: "Melee attack".into(),
                    attack_bonus: Some(3),
                    damage: Some(DiceExpr::new(1, 4, 1)),
                    damage_type: Some("piercing".into()),
                },
                MonsterAction {
                    name: "Poison Bolt".into(),
                    description: "Spell attack".into(),
                    attack_bonus: Some(4),
                    damage: Some(DiceExpr::new(1, 8, 1)),
                    damage_type: Some("poison".into()),
                },
            ],
            traits: vec![],
        },
    );
    map
}

fn demo_world_events() -> EventEngine {
    EventEngine {
        triggers: vec![
            EventTrigger {
                condition: "counter:faction_town_guard_rep >= 3".into(),
                event: WorldEvent::AddJournalEntry {
                    id: "faction-town-guard-friendly".into(),
                    category: JournalCategory::World,
                    title: "Town Guard Trust".into(),
                    body: "The town guard now recognizes your service and offers support.".into(),
                },
                once: true,
                fired: false,
            },
            EventTrigger {
                condition: "counter:faction_town_guard_rep >= 2".into(),
                event: WorldEvent::SetFlag {
                    key: "town_guard_trusted".into(),
                },
                once: true,
                fired: false,
            },
            EventTrigger {
                condition: "counter:faction_goblin_tribe_rep <= -4".into(),
                event: WorldEvent::SetFlag {
                    key: "goblin_tribe_hostile".into(),
                },
                once: true,
                fired: false,
            },
            EventTrigger {
                condition: "flag:town_guard_trusted && flag:read_ember_rune".into(),
                event: WorldEvent::AddJournalEntry {
                    id: "chain-emberpeak-briefing".into(),
                    category: JournalCategory::World,
                    title: "Joint War Council".into(),
                    body: "The guard and summit wardens coordinate supply lines through Emberpeak."
                        .into(),
                },
                once: true,
                fired: false,
            },
            EventTrigger {
                condition: "flag:town_guard_trusted && counter:faction_goblin_tribe_rep <= -3"
                    .into(),
                event: WorldEvent::SetFlag {
                    key: "valley_warfront".into(),
                },
                once: true,
                fired: false,
            },
        ],
    }
}

fn combatant_from_monster(
    combat_id: &str,
    monster: &MonsterDef,
    hp_multiplier: f32,
) -> CombatantState {
    let mut melee_bonus = 2;
    let mut melee_damage = DiceExpr::new(1, 4, 0);
    let mut ranged_attack_bonus = None;
    let mut ranged_damage_dice = None;
    let mut spell_attack_bonus = None;
    let mut spell_damage_dice = None;
    let mut role = EnemyAiRole::Melee;

    for action in &monster.actions {
        let name = action.name.to_lowercase();
        let desc = action.description.to_lowercase();
        let is_spell = name.contains("spell")
            || name.contains("bolt")
            || name.contains("ray")
            || desc.contains("spell");
        let is_ranged = name.contains("bow")
            || name.contains("sling")
            || name.contains("shot")
            || desc.contains("ranged");

        let bonus = action.attack_bonus.unwrap_or(2);
        let damage = action
            .damage
            .clone()
            .unwrap_or_else(|| DiceExpr::new(1, 4, 0));

        if is_spell {
            spell_attack_bonus = Some(bonus);
            spell_damage_dice = Some(damage);
            role = EnemyAiRole::Spellcaster;
            continue;
        }
        if is_ranged {
            ranged_attack_bonus = Some(bonus);
            ranged_damage_dice = Some(damage);
            if role != EnemyAiRole::Spellcaster {
                role = EnemyAiRole::Ranged;
            }
            continue;
        }
        melee_bonus = bonus;
        melee_damage = damage;
    }

    let max_hp = (monster.hp.average() as f32 * hp_multiplier).max(1.0) as i32;
    let mut c = CombatantState::new(
        combat_id,
        monster.name.clone(),
        false,
        max_hp,
        monster.ac as i32,
        monster.speed,
        AbilityScores::modifier(monster.dex_score) as i32,
        melee_bonus,
        melee_damage,
    );
    c.enemy_role = role;
    c.ranged_attack_bonus = ranged_attack_bonus;
    c.ranged_damage_dice = ranged_damage_dice;
    c.spell_attack_bonus = spell_attack_bonus;
    c.spell_damage_dice = spell_damage_dice;
    if role == EnemyAiRole::Spellcaster {
        c.spell_on_hit_condition = Some(crate::game::character::conditions::Condition::Poisoned);
    }
    c
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::TriggerKind;
    use crate::game::character::conditions::Condition;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    fn save_lock() -> MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    fn trigger_position(app: &App, room_id: &str, kind: TriggerKind) -> (u32, u32) {
        let room = app.region.room(room_id).expect("room should exist");
        let trigger = room
            .triggers
            .iter()
            .find(|t| t.kind == kind)
            .expect("trigger should exist");
        (trigger.position[0], trigger.position[1])
    }

    #[test]
    fn combat_attack_consumes_action() {
        let mut app = App::new();
        app.transition(AppState::WorldMap);
        app.handle_event(GameEvent::Attack).unwrap(); // enter combat

        let attacker_id = match &app.state {
            AppState::Combat(ctx) => ctx.state.current_combatant_id().unwrap().to_string(),
            _ => panic!("expected combat state"),
        };
        app.handle_event(GameEvent::Attack).unwrap();

        match &app.state {
            AppState::Combat(ctx) => {
                let attacker = ctx.state.combatants.get(&attacker_id).unwrap();
                assert!(!attacker.action_slots.action);
            }
            _ => panic!("expected combat state"),
        }
    }

    #[test]
    fn incapacitated_combatant_cannot_attack() {
        let mut app = App::new();
        app.transition(AppState::WorldMap);
        app.handle_event(GameEvent::Attack).unwrap(); // enter combat

        if let AppState::Combat(ctx) = &mut app.state {
            let id = ctx.state.current_combatant_id().unwrap().to_string();
            ctx.state
                .combatants
                .get_mut(&id)
                .unwrap()
                .conditions
                .insert(Condition::Stunned);
        }

        app.handle_event(GameEvent::Attack).unwrap();

        match &app.state {
            AppState::Combat(ctx) => {
                assert!(ctx.log.iter().any(|line| line.contains("cannot act")));
                assert!(ctx.log.iter().any(|line| line.contains("Stunned")));
            }
            _ => panic!("expected combat state"),
        }
    }

    #[test]
    fn enemy_turn_executes_on_tick_and_returns_to_player() {
        let mut app = App::new();
        app.transition(AppState::WorldMap);
        app.handle_event(GameEvent::Attack).unwrap(); // enter combat

        if let AppState::Combat(ctx) = &mut app.state {
            if let Some(idx) = ctx
                .state
                .turn_queue
                .iter()
                .position(|id| ctx.state.combatants.get(id).is_some_and(|c| !c.is_player))
            {
                ctx.state.active_turn = idx;
            }
        }

        app.handle_event(GameEvent::Tick).unwrap();

        match &app.state {
            AppState::Combat(ctx) => {
                assert!(ctx.state.current_combatant().is_some_and(|c| c.is_player));
                assert!(ctx.log.iter().any(|line| line.contains("Goblin")));
            }
            _ => panic!("expected combat state"),
        }
    }

    #[test]
    fn tick_transitions_to_world_map_on_player_victory() {
        let mut app = App::new();
        app.transition(AppState::WorldMap);
        app.handle_event(GameEvent::Attack).unwrap(); // enter combat

        if let AppState::Combat(ctx) = &mut app.state {
            for c in ctx.state.combatants.values_mut().filter(|c| !c.is_player) {
                c.current_hp = 0;
            }
        }

        app.handle_event(GameEvent::Tick).unwrap();
        assert!(matches!(app.state, AppState::WorldMap));
    }

    #[test]
    fn tick_transitions_to_game_over_on_player_defeat() {
        let mut app = App::new();
        app.transition(AppState::WorldMap);
        app.handle_event(GameEvent::Attack).unwrap(); // enter combat

        if let AppState::Combat(ctx) = &mut app.state {
            for c in ctx.state.combatants.values_mut().filter(|c| c.is_player) {
                c.current_hp = 0;
            }
        }

        app.handle_event(GameEvent::Tick).unwrap();
        assert!(matches!(app.state, AppState::GameOver));
    }

    #[test]
    fn timed_condition_expires_when_turn_ends() {
        let mut app = App::new();
        app.transition(AppState::WorldMap);
        app.handle_event(GameEvent::Attack).unwrap(); // enter combat

        if let AppState::Combat(ctx) = &mut app.state {
            let current_id = ctx.state.current_combatant_id().unwrap().to_string();
            ctx.state
                .combatants
                .get_mut(&current_id)
                .unwrap()
                .apply_condition(Condition::Poisoned, Some(1));
            let _next = App::advance_turn(ctx);
            assert!(!ctx
                .state
                .combatants
                .get(&current_id)
                .unwrap()
                .conditions
                .contains(&Condition::Poisoned));
            assert!(ctx.log.iter().any(|l| l.contains("expired")));
        } else {
            panic!("expected combat state");
        }
    }

    #[test]
    fn inventory_toggle_equips_weapon_for_combat() {
        let mut app = App::new();
        app.transition(AppState::WorldMap);
        app.handle_event(GameEvent::OpenInventory).unwrap();
        app.handle_event(GameEvent::Choice(1)).unwrap(); // equip longsword
        app.transition(AppState::WorldMap);
        app.handle_event(GameEvent::Attack).unwrap(); // enter combat
        match &app.state {
            AppState::Combat(ctx) => {
                let p = ctx.state.combatants.get("player").unwrap();
                assert_eq!(p.damage_dice, DiceExpr::new(1, 10, 3));
            }
            _ => panic!("expected combat"),
        }
    }

    #[test]
    fn casting_cure_wounds_spends_slot_and_heals() {
        let mut app = App::new();
        app.player.current_hp = 10;
        app.transition(AppState::Spellbook);
        let before_slots = app.player.spell_slots[0];
        app.handle_event(GameEvent::Choice(1)).unwrap(); // cure wounds
        assert!(app.player.current_hp > 10);
        assert_eq!(app.player.spell_slots[0], before_slots - 1);
    }

    #[test]
    fn leveling_up_from_xp_updates_hp_and_level() {
        let mut app = App::new();
        app.player.class_id = "fighter".into();
        app.player.level = 1;
        app.player.xp = 0;
        let hp_before = app.player.max_hp;
        app.grant_player_xp(300); // level 2 threshold
        assert_eq!(app.player.level, 2);
        assert!(app.player.max_hp > hp_before);
    }

    #[test]
    fn casting_uses_highest_available_slot() {
        let mut app = App::new();
        app.player.class_id = "wizard".into();
        app.player.level = 3;
        app.player.spell_slots = [1, 1, 0, 0, 0, 0, 0, 0, 0];
        app.player.spell_slots_max = app.player.spell_slots;
        app.player.current_hp = 5;
        app.transition(AppState::Spellbook);
        app.handle_event(GameEvent::Choice(1)).unwrap(); // cure wounds
        assert_eq!(app.player.spell_slots[1], 0);
        assert_eq!(app.player.spell_slots[0], 1);
    }

    #[test]
    fn equipment_bonus_is_applied_to_combat_stats() {
        let mut app = App::new();
        app.item_defs
            .get_mut("longsword")
            .unwrap()
            .bonuses
            .attack_bonus = 2;
        app.transition(AppState::WorldMap);
        app.handle_event(GameEvent::OpenInventory).unwrap();
        app.handle_event(GameEvent::Choice(1)).unwrap(); // equip longsword
        app.transition(AppState::WorldMap);
        app.handle_event(GameEvent::Attack).unwrap();
        match &app.state {
            AppState::Combat(ctx) => {
                let p = ctx.state.combatants.get("player").unwrap();
                assert!(p.attack_bonus >= 7);
            }
            _ => panic!("expected combat"),
        }
    }

    #[test]
    fn combat_choice_uses_potion_action() {
        let mut app = App::new();
        app.player.current_hp = 8;
        app.transition(AppState::WorldMap);
        app.handle_event(GameEvent::Attack).unwrap(); // enter combat
        if let AppState::Combat(ctx) = &mut app.state {
            ctx.state.active_turn = ctx
                .state
                .turn_queue
                .iter()
                .position(|id| id == "player")
                .unwrap_or(0);
        }
        app.handle_event(GameEvent::Choice(2)).unwrap();
        assert!(app.player.current_hp >= 16);
    }

    #[test]
    fn combat_context_uses_monster_templates() {
        let mut app = App::new();
        let ctx = app.make_combat_context();
        let enemies: Vec<&CombatantState> = ctx
            .state
            .combatants
            .values()
            .filter(|c| !c.is_player)
            .collect();
        assert!(!enemies.is_empty());
        assert!(enemies.iter().all(|c| c.id.starts_with("goblin")));
    }

    #[test]
    fn hostile_goblin_rep_adds_shaman_encounter() {
        let mut app = App::new();
        app.world_state.set_faction_rep("goblin_tribe", -3);
        let ctx = app.make_combat_context();
        assert!(ctx
            .state
            .combatants
            .keys()
            .any(|id| id.starts_with("goblin_shaman")));
    }

    #[test]
    fn world_events_fire_from_faction_thresholds() {
        let mut app = App::new();
        app.world_state.set_faction_rep("town_guard", 3);
        app.handle_event(GameEvent::Tick).unwrap();
        assert!(app
            .journal
            .entries
            .iter()
            .any(|e| e.id == "faction-town-guard-friendly"));

        app.world_state.set_faction_rep("goblin_tribe", -4);
        app.handle_event(GameEvent::Tick).unwrap();
        assert!(app.world_state.flag("goblin_tribe_hostile"));
    }

    #[test]
    fn event_chain_sets_followup_flags_and_entries() {
        let mut app = App::new();
        app.world_state.set_faction_rep("town_guard", 2);
        app.world_state.set_flag("read_ember_rune");
        app.world_state.set_faction_rep("goblin_tribe", -3);
        app.handle_event(GameEvent::Tick).unwrap();
        assert!(app.world_state.flag("town_guard_trusted"));
        assert!(app.world_state.flag("valley_warfront"));
        assert!(app
            .journal
            .entries
            .iter()
            .any(|e| e.id == "chain-emberpeak-briefing"));
    }

    #[test]
    fn positive_goblin_rep_averts_goblin_encounter_trigger() {
        let mut app = App::new();
        app.transition(AppState::WorldMap);
        app.current_room_id = "ember_square".into();
        app.player_pos = trigger_position(&app, "ember_square", TriggerKind::Encounter);
        app.world_state.set_faction_rep("goblin_tribe", 2);
        app.handle_event(GameEvent::Confirm).unwrap();
        assert!(matches!(app.state, AppState::WorldMap));
        assert!(app
            .journal
            .entries
            .iter()
            .any(|e| e.id.contains("encounter-averted")));
    }

    #[test]
    fn guard_support_flag_adds_combat_ally() {
        let mut app = App::new();
        app.world_state.set_flag("request_guard_support");
        let ctx = app.make_combat_context();
        assert!(ctx.state.combatants.contains_key("ally_guard"));
    }

    #[test]
    fn world_map_uses_loaded_region_assets() {
        let app = App::new();
        assert_eq!(app.region.slug, "valley-of-ash");
        assert!(app.region.room("ash_gate").is_some());
        assert!(app.region.room("ember_square").is_some());
    }

    #[test]
    fn world_map_trigger_transitions_into_dialog() {
        let mut app = App::new();
        app.transition(AppState::WorldMap);
        let room_id = app.current_room_id.clone();
        app.player_pos = trigger_position(&app, &room_id, TriggerKind::Dialog);
        app.handle_event(GameEvent::Confirm).unwrap();

        match &app.state {
            AppState::Dialog(ctx) => {
                assert_eq!(ctx.tree.npc_id, "captain_kael");
                assert_eq!(ctx.current_node, "root");
            }
            _ => panic!("expected dialog state"),
        }
    }

    #[test]
    fn dialog_invalid_choice_adds_blocked_feedback_entry() {
        let mut app = App::new();
        app.transition(AppState::Dialog(DialogContext {
            npc_name: "Test NPC".into(),
            tree: DialogTree {
                npc_id: "test_npc".into(),
                nodes: vec![crate::data::types::DialogNode {
                    id: "root".into(),
                    text: "Hello".into(),
                    effect: vec![],
                    choices: vec![],
                    skill: None,
                    dc: None,
                    on_pass: None,
                    on_fail: None,
                }],
            },
            current_node: "root".into(),
            resolved: ResolvedNode {
                id: "root".into(),
                text: "Hello".into(),
                choices: vec![],
            },
        }));

        app.handle_event(GameEvent::Choice(1)).unwrap();
        assert!(app
            .journal
            .entries
            .iter()
            .any(|e| e.body.contains("option is unavailable")));
    }

    #[test]
    fn ash_gate_has_travel_trigger_to_ember_square() {
        let app = App::new();
        let room = app.region.room("ash_gate").expect("ash_gate room");
        let travel = room
            .triggers
            .iter()
            .find(|t| t.kind == TriggerKind::Travel)
            .expect("travel trigger must exist");
        assert_eq!(travel.target_id, "ember_square");
        let [col, row] = travel.position;
        assert!(
            room.grid.is_passable(col as i32, row as i32),
            "travel trigger must be on a passable tile"
        );
    }

    #[test]
    fn world_map_travel_from_ash_gate_moves_to_ember_square() {
        let mut app = App::new();
        app.transition(AppState::WorldMap);
        app.current_room_id = "ash_gate".into();
        // Travel trigger position in ash_gate room.
        app.player_pos = (5, 3);
        app.handle_event(GameEvent::Confirm).unwrap();
        assert_eq!(app.current_room_id, "ember_square");
    }

    #[test]
    fn world_map_trigger_transitions_into_combat() {
        let mut app = App::new();
        app.transition(AppState::WorldMap);
        app.current_room_id = "ember_square".into();
        app.player_pos = trigger_position(&app, "ember_square", TriggerKind::Encounter);
        app.handle_event(GameEvent::Confirm).unwrap();

        match &app.state {
            AppState::Combat(ctx) => {
                let enemies: Vec<&CombatantState> = ctx
                    .state
                    .combatants
                    .values()
                    .filter(|c| !c.is_player)
                    .collect();
                assert_eq!(enemies.len(), 1);
                assert!(enemies[0].id.starts_with("goblin_"));
            }
            _ => panic!("expected combat state"),
        }
    }

    #[test]
    fn save_and_load_roundtrip_through_events() {
        let _guard = save_lock();
        let mut app = App::new();
        let _ = std::fs::remove_file("saves/slot1.toml");
        app.player.current_hp = 9;
        app.player_pos = (2, 2);
        app.handle_event(GameEvent::SaveGame).unwrap();
        app.player.current_hp = 1;
        app.player_pos = (1, 1);
        app.handle_event(GameEvent::LoadGame).unwrap();
        assert_eq!(app.player.current_hp, 9);
        assert_eq!(app.player_pos, (2, 2));
        let _ = std::fs::remove_file("saves/slot1.toml");
        let _ = std::fs::remove_dir("saves");
    }

    #[test]
    fn save_and_load_roundtrip_from_active_world_state() {
        let _guard = save_lock();
        let mut app = App::new();
        let _ = std::fs::remove_file("saves/slot1.toml");
        app.transition(AppState::WorldMap);
        app.current_room_id = "ember_square".into();
        app.player_pos = (5, 3);
        app.turn = 77;
        app.player.current_hp = 5;
        app.world_state.set_flag("m8_active_state");
        app.world_state.set_faction_rep("town_guard", 2);
        app.journal.append(
            "m8-active",
            app.turn,
            JournalCategory::World,
            None,
            "M8",
            "Active runtime snapshot",
        );
        app.handle_event(GameEvent::SaveGame).unwrap();

        app.turn = 1;
        app.player.current_hp = 1;
        app.current_room_id = "ash_gate".into();
        app.player_pos = (1, 1);
        app.world_state.clear_flag("m8_active_state");
        app.world_state.set_faction_rep("town_guard", -2);
        app.journal.entries.clear();

        app.handle_event(GameEvent::LoadGame).unwrap();
        assert!(matches!(app.state, AppState::WorldMap));
        assert_eq!(app.turn, 77);
        assert_eq!(app.player.current_hp, 5);
        assert_eq!(app.current_room_id, "ember_square");
        assert_eq!(app.player_pos, (5, 3));
        assert!(app.world_state.flag("m8_active_state"));
        assert_eq!(app.world_state.faction_rep("town_guard"), 2);
        assert!(app.journal.entries.iter().any(|e| e.id == "m8-active"));

        let _ = std::fs::remove_file("saves/slot1.toml");
        let _ = std::fs::remove_file("saves/slot1.toml");
        let _ = std::fs::remove_dir("saves");
    }

    #[test]
    fn hostile_threshold_combat_trigger() {
        let mut app = App::new();
        app.transition(AppState::WorldMap);
        app.current_room_id = "ember_square".into();
        // Travel trigger to ash_gate is at (5,3)
        app.player_pos = (5, 3);
        // Make town_guard hostile
        app.world_state.set_faction_rep("town_guard", -11);
        app.handle_event(GameEvent::Confirm).unwrap();
        // Confirm moves player to ash_gate and checks hostilities.
        // ash_gate has 'captain_kael' who is in 'town_guard' faction.
        assert_eq!(app.current_room_id, "ash_gate");
        assert!(matches!(app.state, AppState::Combat(_)));
        assert!(app
            .journal
            .entries
            .iter()
            .any(|e| e.id.contains("hostile-intercept")));
    }

    #[test]
    fn significant_rep_change_journal_entry() {
        let mut app = App::new();
        app.modify_faction_rep("guild", 5);
        assert!(app
            .journal
            .entries
            .iter()
            .any(|e| e.title == "Reputation improved"));
        app.modify_faction_rep("guild", -10);
        assert!(app
            .journal
            .entries
            .iter()
            .any(|e| e.title == "Reputation declined"));
    }

    #[test]
    fn inter_faction_vouch_event() {
        let mut app = App::new();
        app.world_state.set_faction_rep("goblin_tribe", -6);
        app.world_state.set_faction_rep("town_guard", 6);
        app.handle_event(GameEvent::Tick).unwrap();
        assert!(app.world_state.flag("town_guard_vouched"));
        assert!(app
            .journal
            .entries
            .iter()
            .any(|e| e.title == "Town Guard Vouched"));
    }
}
