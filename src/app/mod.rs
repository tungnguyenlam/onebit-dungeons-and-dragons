use crate::data::loader::{load_global_assets, load_lore, load_monsters, load_quests, load_region};
use crate::data::types::{
    ArmorDef, ArmorType, DialogTree, ItemBonuses, ItemDef, ItemType, LoreEntry, MonsterDef, NpcDef,
    QuestDef, QuestKind, QuestStageDef, QuestTransition, SpellDef, MonsterAction, WeaponDef,
};
use crate::game::{
    character::{progression::level_for_xp, AbilityScores, Character},
    combat::CombatantState,
    items::equipment::EquipmentSlot,
    save::{load_from_path, save_to_path, SaveGame, SAVE_FORMAT_VERSION},
    story::{
        events::{inspect_lore, EventEngine, EventTrigger, WorldEvent},
        journal::{Journal, Category as JournalCategory},
        quest::QuestLog,
        WorldState,
    },
    world::region::Region,
};
use crate::renderer::{ControlFlow, GameEvent, SoundEffect};
use anyhow::Result;
use std::cell::{RefCell};
use std::collections::{HashMap, HashSet};

pub mod state;
pub mod samples;
pub mod combat;
pub mod handlers;

pub use state::*;
use samples::*;

/// Central application object.
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
                let spawn = find_spawn_pos_for_room(&region.room(&room_id).unwrap());
                (region, loaded.npcs, loaded.dialogs, room_id, spawn)
            } else {
                let (region, npcs, dialogs) = sample_region_bundle();
                let room_id = region
                    .entry()
                    .map(|r| r.id.clone())
                    .unwrap_or_else(|| region.entry_room.clone());
                let spawn = find_spawn_pos_for_room(&region.room(&room_id).unwrap());
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
                self.turn += 1;
                self.handle_tick()?;
            }
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

    pub fn current_room(&self) -> Option<&crate::game::world::room::Room> {
        self.region.room(&self.current_room_id)
    }

    pub fn try_move_player(&mut self, dx: i32, dy: i32) {
        let Some(room) = self.current_room() else {
            return;
        };
        let next_col = self.player_pos.0 as i32 + dx;
        let next_row = self.player_pos.1 as i32 + dy;
        if room.grid.is_passable(next_col, next_row) {
            self.player_pos = (next_col as u32, next_row as u32);
        }
    }

    pub fn interact_current_tile(&mut self) {
        let Some(room) = self.current_room() else { return };
        let (col, row) = (self.player_pos.0 as i32, self.player_pos.1 as i32);
        
        if let Some(trigger) = room.trigger_at(col as u32, row as u32).cloned() {
            match trigger.kind {
                crate::data::types::TriggerKind::Dialog => {
                    self.start_dialog_with_npc(&trigger.target_id);
                }
                crate::data::types::TriggerKind::Encounter => {
                    self.pending_encounter_monster = Some(trigger.target_id.clone());
                    let ctx = self.make_combat_context();
                    self.transition(AppState::Combat(ctx));
                }
                crate::data::types::TriggerKind::Lore => {
                    if let Some(entry) = self.lore_defs.get(&trigger.target_id) {
                        crate::game::story::events::inspect_lore(
                            entry,
                            &mut self.world_state,
                            &mut self.journal,
                            self.turn,
                        );
                    }
                }
                crate::data::types::TriggerKind::QuestStage => {
                    self.world_state.set_flag(trigger.target_id.clone());
                }
                crate::data::types::TriggerKind::Travel => {
                    if self.region.room(&trigger.target_id).is_some() {
                        self.current_room_id = trigger.target_id.clone();
                        if let Some(new_room) = self.current_room() {
                            self.player_pos = find_spawn_pos_for_room(new_room);
                            self.check_room_hostilities();
                        }
                    } else if let Some(_conn) = self.region.connections.iter()
                        .find(|c| c.from_room == self.current_room_id && (c.to_region == trigger.target_id || c.to_room == trigger.target_id))
                    {
                         self.queue_sound(SoundEffect::Beep);
                    }
                }
            }
        }
    }

    pub fn start_dialog_with_npc(&mut self, npc_id: &str) {
        let Some(npc) = self.region_npcs.get(npc_id) else { return };
        let tree = if !npc.dialog_ref.is_empty() {
             self.region_dialogs.get(&npc.dialog_ref).cloned()
        } else {
             self.region_dialogs.get(npc_id).cloned()
        };

        let Some(tree) = tree else { return };

        if let Some(resolved) = crate::game::story::dialog::resolve(&tree, "START", &mut self.world_state) {
            self.transition(AppState::Dialog(DialogContext {
                npc_name: npc.name.clone(),
                tree,
                current_node: "START".into(),
                resolved,
            }));
        }
    }

    pub fn apply_character_creation(&mut self) {
        self.player.name = self.char_creation_ui.name.clone();
        self.player.class_id = self.char_creation_ui.class_options[self.char_creation_ui.class_index].clone();
        self.player.race_id = self.char_creation_ui.race_options[self.char_creation_ui.race_index].clone();
    }

    pub fn equipped_item_ids(&self) -> impl Iterator<Item = &str> {
        self.player.equipment.iter().filter_map(|(_, id)| Some(id.as_str()))
    }

    pub fn equipment_bonus_totals(&self) -> (i32, crate::game::dice::DiceExpr, i32, i32, i32, i32) {
        let mut attack_bonus = 0;
        let mut damage_dice = crate::game::dice::DiceExpr::new(1, 4, 0);
        let mut ac_bonus = 0;
        
        for id in self.equipped_item_ids() {
            if let Some(item) = self.item_defs.get(id) {
                attack_bonus += item.bonuses.attack_bonus;
                ac_bonus += item.bonuses.armor_class_bonus;
                if let Some(w) = &item.weapon {
                    damage_dice = w.damage.clone();
                }
            }
        }
        (attack_bonus, damage_dice, ac_bonus, 0, 0, 0)
    }

    pub fn equipment_resistances(&self) -> HashSet<String> {
        let mut set = HashSet::new();
        for id in self.equipped_item_ids() {
            if let Some(item) = self.item_defs.get(id) {
                for r in &item.bonuses.resistances {
                    set.insert(r.clone());
                }
            }
        }
        set
    }

    pub fn grant_player_xp(&mut self, gained_xp: u32) {
        self.player.xp += gained_xp;
        let new_level = level_for_xp(self.player.xp);
        if new_level > self.player.level {
            self.player.level = new_level;
            self.player.max_hp += 8; // simplified
            self.player.current_hp = self.player.max_hp;
        }
    }

    pub fn save_to_default_path(&mut self) -> Result<()> {
        let save = SaveGame {
            format_version: SAVE_FORMAT_VERSION,
            player: self.player.clone(),
            world_state: self.world_state.clone(),
            journal: self.journal.clone(),
            turn: self.turn,
            region_slug: self.region.slug.clone(),
            room_id: self.current_room_id.clone(),
            player_pos: self.player_pos,
        };
        save_to_path("save.toml", &save)
    }

    pub fn load_from_default_path(&mut self) -> Result<()> {
        let save = load_from_path("save.toml")?;
        self.player = save.player;
        self.world_state = save.world_state;
        self.journal = save.journal;
        self.turn = save.turn;
        self.current_room_id = save.room_id;
        self.player_pos = save.player_pos;
        Ok(())
    }

    pub fn queue_sound(&self, effect: SoundEffect) {
        if self.sound_enabled {
            self.sound_queue.borrow_mut().push(effect);
        }
    }

    pub fn modify_faction_rep(&mut self, faction: &str, delta: i32) {
        let key = format!("faction_{}_rep", faction);
        let cur = self.world_state.counter(&key);
        self.world_state.set_counter(&key, cur + delta);
    }

    pub fn check_room_hostilities(&mut self) {
         // Logic to check if room is hostile
    }

    pub fn tick_story_systems(&mut self) {
        self.world_events.tick(&mut self.world_state, &mut self.journal, self.turn);
        self.quests.tick(&mut self.world_state, &mut self.journal, self.turn);
    }

    pub fn toggle_equip(&mut self, slot: EquipmentSlot, item_id: &str) {
        if self.player.inventory.count(item_id) > 0 {
             self.player.equipment.toggle(slot, item_id.to_string());
        }
    }

    pub fn use_healing_potion(&mut self) {
        if self.player.inventory.count("healing_potion") > 0 {
            self.player.inventory.remove("healing_potion", 1);
            self.player.current_hp = (self.player.current_hp + 10).min(self.player.max_hp);
        }
    }

    pub fn cast_known_spell(&mut self, idx: usize) {
        let Some(spell_id) = self.known_spells.get(idx).cloned() else {
            return;
        };
        let Some(spell_def) = self.spell_defs.get(&spell_id) else {
            return;
        };

        let slot_idx = (spell_def.level.saturating_sub(1)) as usize;
        if slot_idx >= 9 || self.player.spell_slots[slot_idx] == 0 {
            return;
        }

        if let Some(heal_dice) = &spell_def.heal {
            let amount = heal_dice.roll();
            self.player.heal(amount as u32);
            self.player.spell_slots[slot_idx] -= 1;
        }
    }
}

fn find_spawn_pos_for_room(room: &crate::game::world::room::Room) -> (u32, u32) {
    if let Some((col, row, _)) = room
        .grid
        .iter()
        .find(|(_, _, tile)| *tile == crate::game::world::map::Tile::NpcSpawn)
    {
        return (col, row);
    }
    (1, 1)
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod tests;
