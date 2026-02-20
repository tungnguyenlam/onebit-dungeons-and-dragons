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

impl EquipmentSlots {
    /// Whether any item is equipped in the main-hand slot.
    pub fn has_weapon(&self) -> bool { self.main_hand.is_some() }
    pub fn has_shield(&self) -> bool { self.off_hand.is_some() }
}
