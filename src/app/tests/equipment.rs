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