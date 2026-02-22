use crate::App;
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