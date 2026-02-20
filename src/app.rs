/// Application glue layer.
///
/// `App` owns the full mutable game state (`AppState`) and all game
/// sub-systems. It is renderer-agnostic — it has no direct dependency on
/// ratatui, crossterm, egui, or eframe.
///
/// The active renderer calls `App::handle_event` to drive state transitions
/// and reads `App::state` (and sub-system state) during rendering.
use crate::renderer::{ControlFlow, GameEvent};
use crate::game::{
    combat::{
        apply_damage,
        roll_attack,
        AttackProfile,
        CombatState,
        CombatantState,
        DefenseProfile,
        HitType,
        RollMode,
    },
    dice::DiceExpr,
    story::WorldState,
};
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
#[derive(Debug, Clone)]
pub struct CombatContext {
    pub state:       CombatState,
    pub world_state: WorldState,
    pub log:         Vec<String>,
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
            goblin_a.on_hit_condition = Some(crate::game::character::conditions::Condition::Poisoned);
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
        if event == GameEvent::Attack {
            self.transition(AppState::Combat(CombatContext::default()));
        }
        // TODO: movement, interact, open overlays
        Ok(())
    }

    fn handle_combat(&mut self, event: GameEvent) -> Result<()> {
        match event {
            GameEvent::Attack => {
                if let AppState::Combat(ctx) = &mut self.state {
                    let Some(attacker_id) = ctx.state.current_combatant_id().map(str::to_string) else {
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

                    let Some(target_id) = ctx.state.next_enemy_id(&attacker_id).map(str::to_string) else {
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
            Self::push_log(ctx, format!("{target_name} gains condition: {}.", cond.name()));
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
                Self::push_log(ctx, format!("{name} cannot act ({reasons}) and skips turn."));
                let _ = Self::advance_turn(ctx);
                continue;
            }

            let Some(target_id) = ctx.state.next_enemy_id(&attacker_id).map(str::to_string) else {
                Self::push_log(ctx, "Enemy has no valid target.");
                break;
            };

            let _ = Self::resolve_attack(ctx, &attacker_id, &target_id);
            let _ = Self::advance_turn(ctx);
        }
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
        let Some((is_over, players_alive)) = (match &self.state {
            AppState::Combat(ctx) => Some((
                ctx.state.is_over(),
                ctx.state.combatants.values().any(|c| c.is_player && c.is_alive()),
            )),
            _ => None,
        }) else {
            return;
        };

        if !is_over {
            return;
        }

        if players_alive {
            self.transition(AppState::WorldMap);
        } else {
            self.transition(AppState::GameOver);
        }
    }
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
                assert!(ctx
                    .state
                    .current_combatant()
                    .is_some_and(|c| c.is_player));
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
}
