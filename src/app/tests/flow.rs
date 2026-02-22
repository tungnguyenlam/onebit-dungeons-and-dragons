use crate::App;
use crate::app::AppState;
use super::*;
use crate::data::types::TriggerKind;
use crate::data::types::{ItemBonuses, ItemDef, ItemType};
use crate::game::character::conditions::Condition;
use crate::game::combat::{roll_attack, roll_attack_with_seed, AttackProfile, DefenseProfile};
use crate::game::dice::DiceExpr;
use crate::game::items::equipment::EquipmentSlot;
use crate::renderer::GameEvent;
use std::sync::{Mutex, MutexGuard, OnceLock};
use super::utils::*;
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

    app.pass_turn().unwrap();
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

    app.pass_turn().unwrap();
    assert!(matches!(app.state, AppState::GameOver));
}

#[test]
fn functional_smoke_test_main_menu_to_world() {
    let mut app = App::new();
    assert!(matches!(app.state, AppState::MainMenu));

    // Confirm "New Game"
    app.handle_event(GameEvent::Confirm).unwrap();
    assert!(matches!(app.state, AppState::CharacterCreation));

    // Navigate to "Start Adventure"
    app.handle_event(GameEvent::MoveDown).unwrap();
    app.handle_event(GameEvent::MoveDown).unwrap();
    app.handle_event(GameEvent::MoveDown).unwrap();
    assert_eq!(app.char_creation_ui.selected, 3);

    // Confirm adventure start
    app.handle_event(GameEvent::Confirm).unwrap();
    assert!(matches!(app.state, AppState::WorldMap));
    assert!(app.current_room().is_some());
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
    app.player.classes = vec![crate::data::types::ClassLevel { class_id: "fighter".into(), level: 1 }];
    app.player.update_total_level();
    app.player.xp = 0;
    let hp_before = app.player.max_hp;
    app.grant_player_xp(300); // level 2 threshold
    assert_eq!(app.player.total_level, 2);
    assert!(app.player.max_hp > hp_before);
}