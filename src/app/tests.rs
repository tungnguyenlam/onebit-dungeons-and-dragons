use super::*;
use crate::data::types::TriggerKind;
use crate::data::types::{ItemBonuses, ItemDef, ItemType};
use crate::game::character::conditions::Condition;
use crate::game::combat::{roll_attack, roll_attack_with_seed, AttackProfile, DefenseProfile};
use crate::game::dice::DiceExpr;
use crate::game::items::equipment::EquipmentSlot;
use crate::renderer::GameEvent;
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
            // In App::new longsword is 1d8, versatile 1d10.
            // In equipment_bonus_totals it uses w.damage.
            assert_eq!(p.damage_dice, DiceExpr::new(1, 8, 3));
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

#[test]
fn equipment_resistance_halves_elemental_damage() {
    let mut app = App::new();
    app.item_defs.insert(
        "fire_ring".into(),
        ItemDef {
            id: "fire_ring".into(),
            name: "Fire Ring".into(),
            item_type: ItemType::Armor,
            weight: 0.1,
            value_gp: 100,
            description: "Protects from fire.".into(),
            weapon: None,
            armor: None,
            bonuses: ItemBonuses {
                resistances: vec!["fire".into()],
                ..ItemBonuses::default()
            },
        },
    );
    app.player.inventory.add("fire_ring", 1);
    app.player
        .equipment
        .toggle(EquipmentSlot::Ring1, "fire_ring".into());

    app.transition(AppState::WorldMap);
    app.handle_event(GameEvent::Attack).unwrap(); // enter combat

    if let AppState::Combat(ctx) = &mut app.state {
        let p = ctx.state.combatants.get("player").unwrap();
        assert!(
            p.resistances.contains("fire"),
            "Player should have fire resistance"
        );

        let atk_profile = AttackProfile {
            id: "dragon",
            attack_bonus: 100,                      // always hit
            damage_dice: &DiceExpr::new(1, 10, 10), // 11-20
            damage_type: "fire",
            conditions: &std::collections::HashSet::new(),
            on_hit_condition: None,
        };

        let def_profile = DefenseProfile {
            id: "player",
            armor_class: 10,
            conditions: &p.conditions,
            resistances: &p.resistances,
            vulnerabilities: &p.vulnerabilities,
            immunities: &p.immunities,
        };

        let out = roll_attack_with_seed(&atk_profile, &def_profile, 42);
        assert!(
            out.damage >= 5 && out.damage <= 10,
            "Damage {} should be halved (orig 11-20)",
            out.damage
        );
        assert_eq!(out.damage_type, "fire");
    } else {
        panic!("Expected combat state");
    }
}
