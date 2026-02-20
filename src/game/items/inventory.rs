/// Inventory: the ordered list of items a character carries.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemInstance {
    pub item_id:  String,
    pub quantity: u32,
    pub equipped: bool,
}

impl ItemInstance {
    pub fn new(item_id: impl Into<String>, quantity: u32) -> Self {
        Self { item_id: item_id.into(), quantity, equipped: false }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Inventory {
    pub items: Vec<ItemInstance>,
}

impl Inventory {
    pub fn add(&mut self, item_id: impl Into<String>, quantity: u32) {
        let id = item_id.into();
        if let Some(existing) = self.items.iter_mut().find(|i| i.item_id == id) {
            existing.quantity += quantity;
        } else {
            self.items.push(ItemInstance::new(id, quantity));
        }
    }

    pub fn remove(&mut self, item_id: &str, quantity: u32) -> bool {
        if let Some(pos) = self.items.iter().position(|i| i.item_id == item_id) {
            if self.items[pos].quantity >= quantity {
                self.items[pos].quantity -= quantity;
                if self.items[pos].quantity == 0 {
                    self.items.remove(pos);
                }
                return true;
            }
        }
        false
    }

    pub fn count(&self, item_id: &str) -> u32 {
        self.items.iter()
            .filter(|i| i.item_id == item_id)
            .map(|i| i.quantity)
            .sum()
    }
}
