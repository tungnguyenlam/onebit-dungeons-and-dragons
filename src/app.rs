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
    combat::{apply_damage, roll_attack, AttackProfile, CombatState, CombatantState, DefenseProfile, HitType},
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
                // TODO: advance animations, cooldowns, emergent events
            }

            // Route remaining events to the active screen handler.
            other => self.dispatch(other)?,
        }
        Ok(ControlFlow::Continue)
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
        fn push_log(log: &mut Vec<String>, line: String) {
            log.push(line);
            if log.len() > 64 {
                let keep_from = log.len() - 64;
                log.drain(0..keep_from);
            }
        }

        match event {
            GameEvent::Attack => {
                if let AppState::Combat(ctx) = &mut self.state {
                    let Some(attacker_id) = ctx.state.current_combatant_id().map(str::to_string) else {
                        push_log(&mut ctx.log, "No active combatant.".into());
                        return Ok(());
                    };

                    let Some(target_id) = ctx.state.next_enemy_id(&attacker_id).map(str::to_string) else {
                        push_log(&mut ctx.log, "No valid target.".into());
                        return Ok(());
                    };

                    let Some(attacker_view) = ctx.state.combatants.get(&attacker_id) else {
                        push_log(&mut ctx.log, "Attacker not found.".into());
                        return Ok(());
                    };

                    if !attacker_view.can_take_actions() {
                        push_log(
                            &mut ctx.log,
                            format!("{} cannot act (incapacitated).", attacker_view.name),
                        );
                        return Ok(());
                    }

                    if !attacker_view.action_slots.action {
                        push_log(
                            &mut ctx.log,
                            format!("{} has no action remaining this turn.", attacker_view.name),
                        );
                        return Ok(());
                    }

                    let Some(target_view) = ctx.state.combatants.get(&target_id) else {
                        push_log(&mut ctx.log, "Target not found.".into());
                        return Ok(());
                    };

                    let attacker_name = attacker_view.name.clone();
                    let target_name = target_view.name.clone();

                    let outcome = {
                        let attack = AttackProfile {
                            id: &attacker_view.id,
                            attack_bonus: attacker_view.attack_bonus,
                            damage_dice: &attacker_view.damage_dice,
                            conditions: &attacker_view.conditions,
                        };
                        let defense = DefenseProfile {
                            id: &target_view.id,
                            armor_class: target_view.armor_class,
                            conditions: &target_view.conditions,
                        };
                        roll_attack(&attack, &defense, &ctx.world_state)
                    };

                    if let Some(attacker_mut) = ctx.state.combatants.get_mut(&attacker_id) {
                        let _ = attacker_mut.action_slots.use_action();
                    }

                    let mut hp_after = None;
                    if outcome.damage > 0 {
                        if let Some(target_mut) = ctx.state.combatants.get_mut(&target_id) {
                            hp_after = Some(apply_damage(target_mut, outcome.damage));
                        }
                    }

                    let headline = match outcome.hit_type {
                        HitType::Miss => format!(
                            "{attacker_name} attacks {target_name}: miss (d20={} total={}).",
                            outcome.d20, outcome.total
                        ),
                        HitType::Hit => format!(
                            "{attacker_name} hits {target_name} for {} damage (d20={} total={}).",
                            outcome.damage, outcome.d20, outcome.total
                        ),
                        HitType::Critical => format!(
                            "{attacker_name} CRITS {target_name} for {} damage (nat 20).",
                            outcome.damage
                        ),
                    };
                    push_log(&mut ctx.log, headline);

                    if let Some(hp) = hp_after {
                        if hp == 0 {
                            push_log(&mut ctx.log, format!("{target_name} drops to 0 HP."));
                        } else {
                            push_log(&mut ctx.log, format!("{target_name} now has {hp} HP."));
                        }
                    }

                    if ctx.state.is_over() {
                        push_log(&mut ctx.log, "Combat is over.".into());
                    }
                }
            }
            GameEvent::Wait => {
                if let AppState::Combat(ctx) = &mut self.state {
                    let before = ctx
                        .state
                        .current_combatant()
                        .map(|c| c.name.clone())
                        .unwrap_or_else(|| "Unknown".into());
                    let _ = ctx.state.next_turn();
                    let after = ctx
                        .state
                        .current_combatant()
                        .map(|c| c.name.clone())
                        .unwrap_or_else(|| "Unknown".into());
                    ctx.log.push(format!("{before} ends turn. {after} is up."));
                    if ctx.state.is_over() {
                        ctx.log.push("Combat is over.".into());
                    }
                    if ctx.log.len() > 64 {
                        let keep_from = ctx.log.len() - 64;
                        ctx.log.drain(0..keep_from);
                    }
                }
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
            }
            _ => panic!("expected combat state"),
        }
    }
}
