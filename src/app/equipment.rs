use crate::app::App;
use crate::game::items::equipment::EquipmentSlot;
use std::collections::HashSet;

impl App {
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

    pub fn toggle_equip(&mut self, slot: EquipmentSlot, item_id: &str) {
        if self.player.inventory.count(item_id) > 0 {
            self.player.equipment.toggle(slot, item_id.to_string());
        }
    }
}
