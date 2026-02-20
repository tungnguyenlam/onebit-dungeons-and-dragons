use crate::data::loader::load_monsters;
use crate::data::types::{
    ArmorDef, ArmorType, DialogChoice, DialogEffect, DialogNode, DialogTree, ItemDef, ItemType,
    LoreEntry, MonsterAction, MonsterDef, QuestDef, QuestKind, QuestStageDef, QuestTransition,
    SpellDef, WeaponDef,
};
use crate::game::{
    character::{AbilityScores, Character},
    combat::{
        apply_damage, can_cast, expend_slot, resolve_spell_effect, roll_attack, AttackProfile,
        CombatState, CombatantState, DefenseProfile, EnemyAiRole, HitType, RollMode, SpellEffect,
    },
    dice::DiceExpr,
    items::{armor::armor_class, equipment::EquipmentSlot},
    story::{
        dialog::{choose as dialog_choose, resolve as dialog_resolve, ResolvedNode},
        events::{inspect_lore, EventEngine, EventTrigger, WorldEvent},
        journal::{Category as JournalCategory, Journal},
        quest::QuestLog,
        WorldState,
    },
};
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
}

impl Default for JournalUiState {
    fn default() -> Self {
        Self {
            category: JournalCategory::Quest,
            selected: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// App
// ---------------------------------------------------------------------------

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
    pub journal_ui: JournalUiState,
    pub turn: u64,
}

impl App {
    /// Create a new `App` ready to display the main menu.
    pub fn new() -> Self {
        let item_defs = sample_item_defs();
        let spell_defs = sample_spell_defs();
        let monster_defs = load_monsters("assets")
            .ok()
            .filter(|m| !m.is_empty())
            .unwrap_or_else(sample_monster_defs);
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
            quests: QuestLog::with_defs(vec![sample_quest]),
            world_events: demo_world_events(),
            journal_ui: JournalUiState::default(),
            turn: 0,
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

    fn handle_world_map(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Attack => self.transition(AppState::Combat(self.make_combat_context())),
            GameEvent::OpenInventory => self.transition(AppState::Inventory),
            GameEvent::OpenSpellbook => self.transition(AppState::Spellbook),
            GameEvent::OpenJournal => {
                self.journal.mark_read();
                self.journal_ui.selected = 0;
                self.transition(AppState::Journal);
            }
            GameEvent::Confirm => {
                let lore = LoreEntry {
                    id: "old_tablet".into(),
                    title: "Old Tablet".into(),
                    text: "A cracked tablet describes a vault beneath the city.".into(),
                    tags: vec!["history".into()],
                };
                if inspect_lore(&lore, &mut self.world_state, &mut self.journal, self.turn) {
                    self.world_state.set_flag("read_old_tablet");
                }
            }
            GameEvent::OpenMap => {
                if let Some(dialog) = self.make_demo_dialog() {
                    self.transition(AppState::Dialog(dialog));
                }
            }
            _ => {}
        }
        // TODO: movement, interact, open overlays
        Ok(())
    }

    fn handle_combat(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Attack => {
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
                    let _ = Self::resolve_attack(ctx, &attacker_id, &target_id);
                }
                self.finish_combat_if_over();
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
            GameEvent::Back | GameEvent::Cancel => self.transition(AppState::WorldMap),
            GameEvent::MoveUp => {
                self.journal_ui.selected = self.journal_ui.selected.saturating_sub(1);
            }
            GameEvent::MoveDown => {
                self.journal_ui.selected = self.journal_ui.selected.saturating_add(1);
            }
            GameEvent::MoveLeft => {
                self.journal_ui.category = prev_category(self.journal_ui.category);
                self.journal_ui.selected = 0;
            }
            GameEvent::MoveRight => {
                self.journal_ui.category = next_category(self.journal_ui.category);
                self.journal_ui.selected = 0;
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
        if event == GameEvent::Confirm {
            self.transition(AppState::WorldMap);
        }
        Ok(())
    }

    fn push_log(ctx: &mut CombatContext, line: impl Into<String>) {
        ctx.log.push(line.into());
        if ctx.log.len() > 64 {
            let keep_from = ctx.log.len() - 64;
            ctx.log.drain(0..keep_from);
        }
    }

    fn resolve_attack(ctx: &mut CombatContext, attacker_id: &str, target_id: &str) -> bool {
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

        let mut hp_after = None;
        if let Some(target_mut) = ctx.state.combatants.get_mut(target_id) {
            if outcome.damage > 0 {
                hp_after = Some(apply_damage(target_mut, outcome.damage));
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
                outcome.damage, outcome.d20, outcome.total,
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
        let AppState::Combat(ctx) = &mut self.state else {
            return;
        };

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

            let Some(target_id) = ctx.state.next_enemy_id(&attacker_id).map(str::to_string) else {
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
                    let _ = Self::resolve_attack(ctx, &attacker_id, &target_id);
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
                        let _ = Self::resolve_attack(ctx, &attacker_id, &target_id);
                    }
                }
                EnemyAiRole::Spellcaster => {
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
                        let _ = Self::resolve_attack(ctx, &attacker_id, &target_id);
                    }
                }
            }
            let _ = Self::advance_turn(ctx);
        }
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
        let Some((is_over, players_alive, player_hp, ws)) = (match &self.state {
            AppState::Combat(ctx) => Some((
                ctx.state.is_over(),
                ctx.state
                    .combatants
                    .values()
                    .any(|c| c.is_player && c.is_alive()),
                ctx.state.combatants.get("player").map(|c| c.current_hp),
                ctx.world_state.clone(),
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
            self.world_state.set_flag("won_first_combat");
            self.world_state.delta_faction_rep("town_guard", 1);
            self.world_state.delta_faction_rep("goblin_tribe", -2);
            self.transition(AppState::WorldMap);
        } else {
            self.world_state.delta_faction_rep("town_guard", -1);
            self.transition(AppState::GameOver);
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

        let quest_ids: Vec<String> = self.quests.states.keys().cloned().collect();
        for q in quest_ids {
            let _ = self
                .quests
                .tick_quest(&q, &mut self.world_state, &mut self.journal, self.turn);
        }
        self.world_events
            .tick(&mut self.world_state, &mut self.journal, self.turn);
    }

    fn make_demo_dialog(&mut self) -> Option<DialogContext> {
        let tree = DialogTree {
            npc_id: "captain_kael".into(),
            nodes: vec![
                DialogNode {
                    id: "root".into(),
                    text: "The city is in danger. Will you help us?".into(),
                    effect: vec![],
                    choices: vec![
                        DialogChoice {
                            text: "I accept the contract.".into(),
                            condition: "".into(),
                            effect: vec![DialogEffect::SetFlag {
                                set_flag: "accept_demo_quest".into(),
                            }],
                            next: "accepted".into(),
                        },
                        DialogChoice {
                            text: "Report faction standings.".into(),
                            condition: "counter:faction_town_guard_rep >= 1".into(),
                            effect: vec![],
                            next: "faction_status".into(),
                        },
                        DialogChoice {
                            text: "Not now.".into(),
                            condition: "".into(),
                            effect: vec![],
                            next: "END".into(),
                        },
                    ],
                    skill: None,
                    dc: None,
                    on_pass: None,
                    on_fail: None,
                },
                DialogNode {
                    id: "faction_status".into(),
                    text: "The guard appreciates your help. Keep this up and doors will open."
                        .into(),
                    effect: vec![DialogEffect::DeltaCounter {
                        delta_counter: crate::data::types::CounterDelta {
                            key: "faction_town_guard_rep".into(),
                            delta: 1,
                        },
                    }],
                    choices: vec![DialogChoice {
                        text: "Back to business.".into(),
                        condition: "".into(),
                        effect: vec![],
                        next: "END".into(),
                    }],
                    skill: None,
                    dc: None,
                    on_pass: None,
                    on_fail: None,
                },
                DialogNode {
                    id: "accepted".into(),
                    text: "Good. Start by reading the old tablet in the square.".into(),
                    effect: vec![],
                    choices: vec![DialogChoice {
                        text: "Understood.".into(),
                        condition: "".into(),
                        effect: vec![],
                        next: "END".into(),
                    }],
                    skill: None,
                    dc: None,
                    on_pass: None,
                    on_fail: None,
                },
            ],
        };

        let resolved = dialog_resolve(&tree, "root", &mut self.world_state)?;
        Some(DialogContext {
            npc_name: "Captain Kael".into(),
            tree,
            current_node: "root".into(),
            resolved,
        })
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
            Some(spell.level)
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

        match resolve_spell_effect(&spell) {
            Some(SpellEffect::Heal { amount }) => {
                self.player.heal(amount);
                self.journal.append(
                    format!("spell-{}-{}", spell.id, self.turn),
                    self.turn,
                    JournalCategory::World,
                    None,
                    format!("Cast {}", spell.name),
                    format!("You recover {amount} HP."),
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
                    format!("Spell deals {amount} {damage_type} damage."),
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

    fn make_combat_context(&self) -> CombatContext {
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
        let ac = armor_class(armor_item, shield_equipped, self.player.scores.dex_mod());

        let main_weapon = self
            .player
            .equipment
            .main_hand
            .as_ref()
            .and_then(|id| self.item_defs.get(id))
            .and_then(|it| it.weapon.as_ref())
            .cloned();
        let damage = main_weapon
            .as_ref()
            .map(|w| w.damage.clone())
            .unwrap_or_else(|| DiceExpr::new(1, 4, 0));
        let attack_bonus = self.player.scores.str_mod() as i32 + self.player.proficiency_bonus();

        let mut combatants = vec![CombatantState::new(
            "player",
            self.player.name.clone(),
            true,
            self.player.max_hp,
            ac,
            self.player.speed,
            self.player.scores.dex_mod() as i32,
            attack_bonus,
            damage,
        )];
        combatants.extend(self.build_encounter_monsters());

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

    fn build_encounter_monsters(&self) -> Vec<CombatantState> {
        let mut out = Vec::new();
        let ids = if self.world_state.faction_rep("goblin_tribe") <= -2 {
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
            out.push(combatant_from_monster(&cid, def));
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
        JournalCategory::Dialog => JournalCategory::Quest,
    }
}

fn prev_category(c: JournalCategory) -> JournalCategory {
    match c {
        JournalCategory::Quest => JournalCategory::Dialog,
        JournalCategory::Lore => JournalCategory::Quest,
        JournalCategory::World => JournalCategory::Lore,
        JournalCategory::Combat => JournalCategory::World,
        JournalCategory::Dialog => JournalCategory::Combat,
    }
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
                condition: "counter:faction_goblin_tribe_rep <= -4".into(),
                event: WorldEvent::SetFlag {
                    key: "goblin_tribe_hostile".into(),
                },
                once: true,
                fired: false,
            },
        ],
    }
}

fn combatant_from_monster(combat_id: &str, monster: &MonsterDef) -> CombatantState {
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

    let max_hp = monster.hp.average().max(1);
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
    use crate::game::character::conditions::Condition;

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
                assert_eq!(p.damage_dice, DiceExpr::new(1, 8, 0));
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
    fn combat_context_uses_monster_templates() {
        let app = App::new();
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
}
