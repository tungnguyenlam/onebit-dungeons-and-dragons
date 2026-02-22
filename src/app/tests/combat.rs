use super::*;
use crate::data::types::TriggerKind;
use crate::data::types::{ItemBonuses, ItemDef, ItemType};
use crate::game::character::conditions::Condition;
use crate::game::combat::{roll_attack, roll_attack_with_seed, AttackProfile, DefenseProfile};
use crate::game::dice::DiceExpr;
use crate::game::items::equipment::EquipmentSlot;
use crate::renderer::GameEvent;
use std::sync::{Mutex, MutexGuard, OnceLock};
use crate::App;
use crate::game::combat::CombatantState;
use crate::app::AppState;
use super::utils::*;
#[test]
fn combat_attack_consumes_action() {
    let mut app = App::new();
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

    let attacker_id = match &app.state {
        AppState::Combat(ctx) => ctx.state.current_combatant_id().unwrap().to_string(),
        _ => panic!("expected combat state"),
    };
    app.handle_event(GameEvent::Attack).unwrap();

    match &app.state {
        AppState::Combat(ctx) => {
            assert!(ctx.log.iter().any(|line| line.contains("hits") || line.contains("misses") || line.contains("Critical")));
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
        ctx.state.active_turn = ctx
            .state
            .turn_queue
            .iter()
            .position(|id| id == "player")
            .unwrap_or(0);
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
            assert!(ctx.log.iter().any(|line: &String| line.contains("cannot act")));
            assert!(ctx.log.iter().any(|line: &String| line.contains("Stunned")));
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

    app.pass_turn().unwrap();

    match &app.state {
        AppState::Combat(ctx) => {
            assert!(ctx.state.current_combatant().is_some_and(|c| c.is_player));
            assert!(ctx.log.iter().any(|line: &String| line.contains("Goblin")));
        }
        _ => panic!("expected combat state"),
    }
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
        assert!(ctx.log.iter().any(|l: &String| l.contains("is no longer Poisoned.")));
    } else {
        panic!("expected combat state");
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
    assert!(app.player.current_hp >= 12); // 8 + (min 2d4+2 = 4) = 12
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
}