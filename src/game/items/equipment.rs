/// Equipment slots — what the character has equipped.
use serde::{Deserialize, Serialize};

pub type ItemId = String;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EquipmentSlots {
    pub main_hand: Option<ItemId>,
    pub off_hand:  Option<ItemId>,
    pub armor:     Option<ItemId>,
    pub helmet:    Option<ItemId>,
    pub boots:     Option<ItemId>,
    pub ring_1:    Option<ItemId>,
    pub ring_2:    Option<ItemId>,
    pub amulet:    Option<ItemId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Armor,
    Helmet,
    Boots,
    Ring1,
    Ring2,
    Amulet,
}

impl EquipmentSlots {
    /// Whether any item is equipped in the main-hand slot.
    pub fn has_weapon(&self) -> bool { self.main_hand.is_some() }
    pub fn has_shield(&self) -> bool { self.off_hand.is_some() }

    pub fn equip(&mut self, slot: EquipmentSlot, item_id: impl Into<String>) -> Option<ItemId> {
        let id = Some(item_id.into());
        match slot {
            EquipmentSlot::MainHand => std::mem::replace(&mut self.main_hand, id),
            EquipmentSlot::OffHand  => std::mem::replace(&mut self.off_hand, id),
            EquipmentSlot::Armor    => std::mem::replace(&mut self.armor, id),
            EquipmentSlot::Helmet   => std::mem::replace(&mut self.helmet, id),
            EquipmentSlot::Boots    => std::mem::replace(&mut self.boots, id),
            EquipmentSlot::Ring1    => std::mem::replace(&mut self.ring_1, id),
            EquipmentSlot::Ring2    => std::mem::replace(&mut self.ring_2, id),
            EquipmentSlot::Amulet   => std::mem::replace(&mut self.amulet, id),
        }
    }

    pub fn unequip(&mut self, slot: EquipmentSlot) -> Option<ItemId> {
        match slot {
            EquipmentSlot::MainHand => self.main_hand.take(),
            EquipmentSlot::OffHand  => self.off_hand.take(),
            EquipmentSlot::Armor    => self.armor.take(),
            EquipmentSlot::Helmet   => self.helmet.take(),
            EquipmentSlot::Boots    => self.boots.take(),
            EquipmentSlot::Ring1    => self.ring_1.take(),
            EquipmentSlot::Ring2    => self.ring_2.take(),
            EquipmentSlot::Amulet   => self.amulet.take(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equip_and_unequip_roundtrip() {
        let mut e = EquipmentSlots::default();
        assert_eq!(e.equip(EquipmentSlot::MainHand, "longsword"), None);
        assert_eq!(e.main_hand.as_deref(), Some("longsword"));
        assert_eq!(e.unequip(EquipmentSlot::MainHand).as_deref(), Some("longsword"));
        assert!(e.main_hand.is_none());
    }
}
