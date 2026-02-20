/// Spell slot and effect resolution.
use crate::{data::types::SpellDef, game::{character::conditions::Condition, dice::DiceExpr}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellEffect {
    Damage { amount: u32, damage_type: String },
    Heal { amount: u32 },
    Condition { condition: Condition },
}

/// Can the caster cast `spell` at `slot_level` (1-based) using remaining slots.
pub fn can_cast(spell: &SpellDef, slots_remaining: &[u8; 9], slot_level: Option<u8>) -> bool {
    if spell.level == 0 {
        return true;
    }
    let level = slot_level.unwrap_or(spell.level).max(spell.level);
    let idx = (level - 1) as usize;
    slots_remaining.get(idx).copied().unwrap_or(0) > 0
}

/// Spend one spell slot. Returns true on success.
pub fn expend_slot(slots_remaining: &mut [u8; 9], slot_level: u8) -> bool {
    if slot_level == 0 {
        return true;
    }
    let idx = (slot_level - 1) as usize;
    let Some(slot) = slots_remaining.get_mut(idx) else {
        return false;
    };
    if *slot == 0 {
        return false;
    }
    *slot -= 1;
    true
}

/// Resolve simple spell effects from current schema.
pub fn resolve_effect(spell: &SpellDef) -> Option<SpellEffect> {
    if let Some(heal) = &spell.heal {
        return Some(SpellEffect::Heal {
            amount: heal.roll().max(0) as u32,
        });
    }
    if let Some(damage) = &spell.damage {
        return Some(SpellEffect::Damage {
            amount: damage.roll().max(0) as u32,
            damage_type: spell.damage_type.clone().unwrap_or_else(|| "force".into()),
        });
    }
    infer_condition(&spell.id).map(|condition| SpellEffect::Condition { condition })
}

fn infer_condition(id: &str) -> Option<Condition> {
    if id.contains("poison") {
        Some(Condition::Poisoned)
    } else if id.contains("stun") {
        Some(Condition::Stunned)
    } else if id.contains("blind") {
        Some(Condition::Blinded)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spell() -> SpellDef {
        SpellDef {
            id: "cure_wounds".into(),
            name: "Cure Wounds".into(),
            level: 1,
            school: "evocation".into(),
            casting_time: "action".into(),
            range: "touch".into(),
            components: vec!["V".into(), "S".into()],
            duration: "instant".into(),
            description: "heal".into(),
            damage: None,
            damage_type: None,
            save: None,
            heal: Some(DiceExpr::new(1, 8, 2)),
            classes: vec!["cleric".into()],
        }
    }

    #[test]
    fn slot_checks_and_spend() {
        let s = spell();
        let mut slots = [0; 9];
        slots[0] = 1;
        assert!(can_cast(&s, &slots, None));
        assert!(expend_slot(&mut slots, 1));
        assert!(!can_cast(&s, &slots, None));
    }

    #[test]
    fn resolve_heal_effect() {
        let s = spell();
        let effect = resolve_effect(&s).unwrap();
        match effect {
            SpellEffect::Heal { amount } => assert!(amount >= 3),
            _ => panic!("expected heal"),
        }
    }
}
