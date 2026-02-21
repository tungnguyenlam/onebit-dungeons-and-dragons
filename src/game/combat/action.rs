/// Per-turn action economy tracking.
///
/// Mirrors 5e's action + bonus action + reaction plus movement budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionSlots {
    pub action: bool,
    pub bonus_action: bool,
    pub reaction: bool,
    pub movement_remaining: u32,
}

impl ActionSlots {
    /// Fresh turn state with all slots available.
    pub fn new(speed: u32) -> Self {
        Self {
            action: true,
            bonus_action: true,
            reaction: true,
            movement_remaining: speed,
        }
    }

    /// Reset to a new turn's full budget.
    pub fn reset_turn(&mut self, speed: u32) {
        *self = Self::new(speed);
    }

    pub fn use_action(&mut self) -> bool {
        if !self.action {
            return false;
        }
        self.action = false;
        true
    }

    pub fn use_bonus_action(&mut self) -> bool {
        if !self.bonus_action {
            return false;
        }
        self.bonus_action = false;
        true
    }

    pub fn use_reaction(&mut self) -> bool {
        if !self.reaction {
            return false;
        }
        self.reaction = false;
        true
    }

    /// Spend movement budget in tiles/feet units used by the caller.
    pub fn spend_movement(&mut self, amount: u32) -> bool {
        if amount > self.movement_remaining {
            return false;
        }
        self.movement_remaining -= amount;
        true
    }
}

impl Default for ActionSlots {
    fn default() -> Self {
        Self::new(30)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consumes_action_slots_once() {
        let mut slots = ActionSlots::new(30);
        assert!(slots.use_action());
        assert!(!slots.use_action());
        assert!(slots.use_bonus_action());
        assert!(!slots.use_bonus_action());
        assert!(slots.use_reaction());
        assert!(!slots.use_reaction());
    }

    #[test]
    fn movement_budget_spends_and_caps() {
        let mut slots = ActionSlots::new(30);
        assert!(slots.spend_movement(10));
        assert_eq!(slots.movement_remaining, 20);
        assert!(!slots.spend_movement(21));
        assert_eq!(slots.movement_remaining, 20);
    }

    #[test]
    fn reset_turn_restores_all_slots() {
        let mut slots = ActionSlots::new(30);
        slots.use_action();
        slots.use_bonus_action();
        slots.use_reaction();
        slots.spend_movement(20);
        slots.reset_turn(25);
        assert_eq!(slots, ActionSlots::new(25));
    }
}
