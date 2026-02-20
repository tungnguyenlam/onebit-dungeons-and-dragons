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

    pub fn set_equipped(&mut self, item_id: &str, equipped: bool) -> bool {
        if let Some(item) = self.items.iter_mut().find(|i| i.item_id == item_id) {
            item.equipped = equipped;
            return true;
        }
        false
    }

    pub fn is_equipped(&self, item_id: &str) -> bool {
        self.items
            .iter()
            .find(|i| i.item_id == item_id)
            .is_some_and(|i| i.equipped)
    }

    pub fn use_one(&mut self, item_id: &str) -> bool {
        self.remove(item_id, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_query_equipped() {
        let mut inv = Inventory::default();
        inv.add("longsword", 1);
        assert!(!inv.is_equipped("longsword"));
        assert!(inv.set_equipped("longsword", true));
        assert!(inv.is_equipped("longsword"));
    }

    #[test]
    fn use_one_consumes_stack() {
        let mut inv = Inventory::default();
        inv.add("potion", 2);
        assert!(inv.use_one("potion"));
        assert_eq!(inv.count("potion"), 1);
        assert!(inv.use_one("potion"));
        assert_eq!(inv.count("potion"), 0);
    }
}
