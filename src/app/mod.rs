use crate::data::loader::{load_global_assets, load_lore, load_monsters, load_quests, load_region};
use crate::data::types::{
    ArmorDef, ArmorType, DialogTree, ItemBonuses, ItemDef, ItemType, LoreEntry, MonsterAction,
    MonsterDef, NpcDef, QuestDef, QuestKind, QuestStageDef, QuestTransition, SpellDef, WeaponDef,
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

use samples::*;
pub use state::*;

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

    fn handle_travel(&mut self, target_id: &str) {
        let epic_progress = self.world_state.counter("epic_quest_progress");
        let macguffin_acquired = self.world_state.flag("macguffin_acquired");
        let threat_level = if self.world_state.flag("macguffin_acquired") {
            3
        } else if epic_progress >= 2 {
            2
        } else if epic_progress >= 1 {
            1
        } else {
            0
        };

        if threat_level > 0 {
            use rand::Rng;
            let mut rng = rand::rng();
            let ambush_chance = match threat_level {
                3 => 4,
                2 => 5,
                1 => 6,
                _ => 0,
            };

            if rng.random_range(1..=ambush_chance) == 1 {
                self.queue_sound(SoundEffect::Beep);
                let ambush_monster = match threat_level {
                    3 => "ghostly_knight",
                    2 => "orc_warchief",
                    _ => "forest_goblin",
                };
                self.pending_encounter_monster = Some(ambush_monster.into());
                let ctx = self.make_combat_context();
                self.transition(AppState::Combat(ctx));

                if threat_level >= 2 && !self.world_state.flag("antagonist_noticed") {
                    self.world_state.set_flag("antagonist_noticed");
                    self.journal.append(
                        format!("antagonist-notice-{}", self.turn),
                        self.turn,
                        JournalCategory::World,
                        None,
                        "The Antagonist Notices You",
                        "Dark scouts have reported your movements. Expect increased hostility.",
                    );
                }
                return;
            }
        }

        let from_room_id = self.current_room_id.clone();

        if self.region.room(target_id).is_some() {
            self.current_room_id = target_id.to_string();
            if let Some(new_room) = self.current_room() {
                self.player_pos = self.find_entry_pos(new_room, &from_room_id);
                self.check_room_hostilities();
            }
        } else if let Some(conn) = self
            .region
            .connections
            .iter()
            .find(|c| {
                c.from_room == self.current_room_id
                    && (c.to_region == target_id || c.to_room == target_id)
            })
            .cloned()
        {
            let mut target_region = conn.to_region.clone();
            let ruined_map = [("ironhold-mines", "ruined-ironhold-mines")];

            for (normal, ruined) in ruined_map {
                if macguffin_acquired && target_region == normal {
                    target_region = ruined.into();
                    break;
                }
            }

            if let Ok(loaded) = crate::data::loader::load_region("assets", &target_region) {
                self.region = Region::from_loaded(&loaded);
                self.region_npcs = loaded.npcs;
                self.region_dialogs = loaded.dialogs;
                self.current_room_id = conn.to_room.clone();
                if !self.region.rooms.contains_key(&self.current_room_id) {
                    self.current_room_id = loaded.manifest.entry_room;
                }
                if let Some(new_room) = self.current_room() {
                    self.player_pos = self.find_entry_pos(new_room, &from_room_id);
                    self.check_room_hostilities();
                }
            }
            self.queue_sound(SoundEffect::Beep);
        }
    }

    fn find_entry_pos(
        &self,
        room: &crate::game::world::room::Room,
        from_room_id: &str,
    ) -> (u32, u32) {
        // Try to find a travel trigger in the new room that leads back to the old room
        if let Some(back_trigger) = room.triggers.iter().find(|t| {
            matches!(t.kind, crate::data::types::TriggerKind::Travel) && t.target_id == from_room_id
        }) {
            // Spawn next to the back trigger instead of on top of it, if possible
            let tx = back_trigger.position[0];
            let ty = back_trigger.position[1];

            // Just spawn on it for now to ensure it works, but usually you'd offset it
            return (tx, ty);
        }

        // Fallback to default spawn
        find_spawn_pos_for_room(room)
    }

    fn get_npc_at_player_position(&self) -> Option<&NpcDef> {
        let (col, row) = self.player_pos;
        if let Some(room) = self.current_room() {
            if let Some(room_npc) = room
                .npcs
                .iter()
                .find(|n| n.position[0] == col && n.position[1] == row)
            {
                return self.region_npcs.get(&room_npc.id);
            }
        }
        None
    }

    pub fn interact_current_tile(&mut self) {
        let room_id = self.current_room_id.clone();
        let (col, row) = self.player_pos;

        // Check triggers at current position - get fresh borrow
        if let Some(trigger) = self
            .region
            .room(&room_id)
            .and_then(|r| r.trigger_at(col, row).cloned())
        {
            self.execute_trigger(&trigger);
            return;
        }

        // Check for NPCs at current position
        if let Some(room) = self.region.room(&room_id) {
            if let Some(npc_id) = room
                .npcs
                .iter()
                .find(|n| n.position[0] == col && n.position[1] == row)
                .map(|n| n.id.clone())
            {
                self.start_dialog_with_npc(&npc_id);
                return;
            }
        }

        // Check adjacent for doors/chests/travel
        let mut interactable_found = false;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = col as i32 + dx;
                let ny = row as i32 + dy;
                if nx >= 0 && ny >= 0 {
                    let nx = nx as u32;
                    let ny = ny as u32;

                    if let Some(trigger) = self
                        .region
                        .room(&room_id)
                        .and_then(|r| r.trigger_at(nx, ny).cloned())
                    {
                        if matches!(trigger.kind, crate::data::types::TriggerKind::Travel) {
                            self.execute_trigger(&trigger);
                            interactable_found = true;
                            break;
                        }
                    }
                }
            }
            if interactable_found {
                break;
            }
        }

        if !interactable_found {
            if self.is_near_door() {
                self.set_feedback("You are near a door. Step on it or face it to interact.");
            } else if self.is_near_chest() {
                self.set_feedback("You are near a chest. Step on it to open.");
            } else {
                self.set_feedback("Nothing here to interact with.");
            }
        }
    }

    fn execute_trigger(&mut self, trigger: &crate::data::types::TriggerDef) {
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

                let macguffins = [
                    "has_obsidian_eye",
                    "has_obsidian_heart",
                    "has_sylvan_glitch_key",
                    "has_null_scepter",
                ];

                if macguffins.iter().any(|m| *m == trigger.target_id) {
                    if !self.world_state.flag("macguffin_acquired") {
                        self.world_state.set_flag("macguffin_acquired");
                        let macguffin_count = macguffins
                            .iter()
                            .filter(|m| self.world_state.flag(m))
                            .count();
                        self.world_state
                            .delta_counter("epic_quest_progress", macguffin_count as i32);

                        self.journal.append(
                            format!("macguffin-acquired-{}", self.turn),
                            self.turn,
                            JournalCategory::World,
                            None,
                            "The Antagonist Stirs",
                            "Dark forces have detected your acquisition. The enemy grows stronger against you.",
                        );
                    }
                }
            }
            crate::data::types::TriggerKind::Travel => {
                self.handle_travel(&trigger.target_id);
            }
        }
    }

    fn is_near_door(&self) -> bool {
        let (cx, cy) = self.player_pos;
        let room = self.current_room();
        if let Some(room) = room {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = cx as i32 + dx;
                    let ny = cy as i32 + dy;
                    if nx >= 0 && ny >= 0 {
                        if let Some(tile) = room.grid.get(nx as u32, ny as u32) {
                            if matches!(
                                tile,
                                crate::game::world::map::Tile::DoorOpen
                                    | crate::game::world::map::Tile::DoorClosed
                            ) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn is_near_chest(&self) -> bool {
        let (cx, cy) = self.player_pos;
        let room = self.current_room();
        if let Some(room) = room {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = cx as i32 + dx;
                    let ny = cy as i32 + dy;
                    if nx >= 0 && ny >= 0 {
                        if let Some(tile) = room.grid.get(nx as u32, ny as u32) {
                            if matches!(tile, crate::game::world::map::Tile::Chest) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn is_blocked(&self) -> bool {
        let room = self.current_room();
        if let Some(room) = room {
            let (col, row) = self.player_pos;
            return !room.grid.is_passable(col as i32, row as i32);
        }
        false
    }

    pub fn start_dialog_with_npc(&mut self, npc_id: &str) {
        let Some(npc) = self.region_npcs.get(npc_id) else {
            return;
        };
        let tree = if !npc.dialog_ref.is_empty() {
            self.region_dialogs.get(&npc.dialog_ref).cloned()
        } else {
            self.region_dialogs.get(npc_id).cloned()
        };

        let Some(tree) = tree else { return };

        if let Some(resolved) =
            crate::game::story::dialog::resolve(&tree, "START", &mut self.world_state)
        {
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
        self.player.class_id =
            self.char_creation_ui.class_options[self.char_creation_ui.class_index].clone();
        self.player.race_id =
            self.char_creation_ui.race_options[self.char_creation_ui.race_index].clone();
    }

    pub fn equipped_item_ids(&self) -> impl Iterator<Item = &str> {
        self.player.equipment.iter().map(|(_, id)| id.as_str())
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

    pub fn equipment_immunities(&self) -> HashSet<String> {
        let mut set = HashSet::new();
        for id in self.equipped_item_ids() {
            if let Some(item) = self.item_defs.get(id) {
                for i in &item.bonuses.immunities {
                    set.insert(i.clone());
                }
            }
        }
        set
    }

    pub fn equipment_condition_immunities(&self) -> HashSet<String> {
        let mut set = HashSet::new();
        for id in self.equipped_item_ids() {
            if let Some(item) = self.item_defs.get(id) {
                for i in &item.bonuses.condition_immunities {
                    set.insert(i.clone());
                }
            }
        }
        set
    }

    pub fn grant_player_xp(&mut self, gained_xp: u32) {
        self.player.xp += gained_xp;
        let old_level = self.player.level;
        let new_level = level_for_xp(self.player.xp);

        if new_level > old_level {
            let levels_gained = new_level - old_level;
            self.player.level = new_level;
            self.player.max_hp += 8 * levels_gained as i32;
            self.player.current_hp = self.player.max_hp;
            self.player.skill_points += (levels_gained * 2) as u32;

            self.set_feedback(&format!(
                "Leveled up to {}! +{} HP, +{} skill points",
                new_level,
                8 * levels_gained as i32,
                levels_gained * 2
            ));
        }

        let gold_found = gained_xp / 10;
        if gold_found > 0 {
            self.player.gold += gold_found;
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
        self.world_events
            .tick(&mut self.world_state, &mut self.journal, self.turn);
        self.quests
            .tick(&mut self.world_state, &mut self.journal, self.turn);
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
