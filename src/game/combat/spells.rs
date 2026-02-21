/// Spell slot and effect resolution.
use crate::{
    data::types::SpellDef,
    game::{character::conditions::Condition, dice::DiceExpr},
};

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

/// Resolve spell effects with simple scaling rules:
/// - cantrips scale with level (5/11/17)
/// - leveled spells gain +1 damage/heal die per upcast level
pub fn resolve_effect(
    spell: &SpellDef,
    cast_level: Option<u8>,
    caster_level: u8,
    flat_bonus: i32,
) -> Option<SpellEffect> {
    let resolved_level = cast_level.unwrap_or(spell.level).max(spell.level);
    if let Some(heal) = &spell.heal {
        let mut expr = heal.clone();
        if resolved_level > spell.level && spell.level > 0 {
            expr.count += (resolved_level - spell.level) as u32;
        }
        expr.modifier += flat_bonus;
        return Some(SpellEffect::Heal {
            amount: expr.roll().max(0) as u32,
        });
    }
    if let Some(damage) = &spell.damage {
        let mut expr = damage.clone();
        if spell.level == 0 {
            let scale = cantrip_scale(caster_level);
            expr.count *= scale;
        } else if resolved_level > spell.level {
            expr.count += (resolved_level - spell.level) as u32;
        }
        expr.modifier += flat_bonus;
        return Some(SpellEffect::Damage {
            amount: expr.roll().max(0) as u32,
            damage_type: spell.damage_type.clone().unwrap_or_else(|| "force".into()),
        });
    }
    infer_condition(&spell.id).map(|condition| SpellEffect::Condition { condition })
}

fn cantrip_scale(level: u8) -> u32 {
    match level {
        1..=4 => 1,
        5..=10 => 2,
        11..=16 => 3,
        _ => 4,
    }
}

fn infer_condition(id: &str) -> Option<Condition> {
    if id.contains("poison") {
        Some(Condition::Poisoned)
    } else if id.contains("stun") {
        Some(Condition::Stunned)
    } else if id.contains("blind") {
        Some(Condition::Blinded)
    } else if id.contains("haste") {
        Some(Condition::Hasted)
    } else if id.contains("invisib") {
        Some(Condition::Invisible)
    } else if id.contains("banish") {
        Some(Condition::Incapacitated)
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
        let effect = resolve_effect(&s, Some(1), 1, 0).unwrap();
        match effect {
            SpellEffect::Heal { amount } => assert!(amount >= 3),
            _ => panic!("expected heal"),
        }
    }

    #[test]
    fn cantrip_damage_scales_by_level() {
        let spell = SpellDef {
            id: "fire_bolt".into(),
            name: "Fire Bolt".into(),
            level: 0,
            school: "evocation".into(),
            casting_time: "action".into(),
            range: "120ft".into(),
            components: vec!["V".into(), "S".into()],
            duration: "instant".into(),
            description: "burn".into(),
            damage: Some(DiceExpr::new(1, 10, 0)),
            damage_type: Some("fire".into()),
            save: None,
            heal: None,
            classes: vec!["wizard".into()],
        };
        let low = resolve_effect(&spell, None, 1, 0).unwrap();
        let high = resolve_effect(&spell, None, 11, 0).unwrap();
        let (
            SpellEffect::Damage {
                amount: low_amt, ..
            },
            SpellEffect::Damage {
                amount: high_amt, ..
            },
        ) = (low, high)
        else {
            panic!("expected damage");
        };
        assert!(high_amt >= 3);
        assert!(high_amt >= low_amt.min(1));
    }
}
