use crate::data::loader::{load_monsters, load_region};
use crate::data::types::{
    ArmorDef, ArmorType, DialogTree, ItemBonuses, ItemDef, ItemType, MonsterAction, MonsterDef,
    NpcDef, SpellDef, WeaponDef,
}; // Careful: check types here
use crate::game::{
    character::{conditions::Condition, AbilityScores},
    combat::CombatantState,
    combat::EnemyAiRole,
    dice::DiceExpr,
    story::events::{EventEngine, EventTrigger, WorldEvent},
    story::journal::Category as JournalCategory,
    world::region::Region,
};
use std::collections::HashMap;

pub fn combatant_from_monster(
    combat_id: &str,
    monster: &MonsterDef,
    hp_multiplier: f32,
) -> CombatantState {
    let mut melee_bonus = 2;
    let mut melee_damage = DiceExpr::new(1, 4, 0);
    let mut ranged_attack_bonus = None;
    let mut ranged_damage_dice = None;
    let mut spell_attack_bonus = None;
    let mut spell_damage_dice = None;
    let mut ranged_damage_type = None;
    let mut spell_damage_type = None;
    let mut role = EnemyAiRole::Melee;
    let mut c_damage_type = "bludgeoning".to_string();

    let mut melee_on_hit_condition = None;
    let mut ranged_on_hit_condition = None;
    let mut spell_on_hit_condition = None;

    for action in &monster.actions {
        let name = action.name.to_lowercase();
        let desc = action.description.to_lowercase();
        let is_spell = name.contains("spell")
            || name.contains("bolt")
            || name.contains("ray")
            || desc.contains("spell");
        let is_ranged = name.contains("bow")
            || name.contains("sling")
            || name.contains("shot")
            || name.contains("dart")
            || name.contains("web")
            || desc.contains("ranged");

        let bonus = action.attack_bonus.unwrap_or(2);
        let damage = action
            .damage
            .clone()
            .unwrap_or_else(|| DiceExpr::new(1, 4, 0));
        let type_str = action
            .damage_type
            .clone()
            .unwrap_or_else(|| "bludgeoning".to_string());

        let cond = action.on_hit_condition.as_deref().and_then(|s| match s {
            "blinded" => Some(Condition::Blinded),
            "charmed" => Some(Condition::Charmed),
            "frightened" => Some(Condition::Frightened),
            "grappled" => Some(Condition::Grappled),
            "incapacitated" => Some(Condition::Incapacitated),
            "invisible" => Some(Condition::Invisible),
            "paralyzed" => Some(Condition::Paralyzed),
            "petrified" => Some(Condition::Petrified),
            "poisoned" => Some(Condition::Poisoned),
            "prone" => Some(Condition::Prone),
            "restrained" => Some(Condition::Restrained),
            "stunned" => Some(Condition::Stunned),
            "unconscious" => Some(Condition::Unconscious),
            _ => None,
        });

        if is_spell {
            spell_attack_bonus = Some(bonus);
            spell_damage_dice = Some(damage);
            spell_damage_type = Some(type_str);
            spell_on_hit_condition = cond;
            role = EnemyAiRole::Spellcaster;
            continue;
        }
        if is_ranged {
            ranged_attack_bonus = Some(bonus);
            ranged_damage_dice = Some(damage);
            ranged_damage_type = Some(type_str);
            ranged_on_hit_condition = cond;
            if role != EnemyAiRole::Spellcaster {
                role = EnemyAiRole::Ranged;
            }
            continue;
        }
        melee_bonus = bonus;
        melee_damage = damage;
        melee_on_hit_condition = cond;
        c_damage_type = type_str;
    }

    let max_hp = (monster.hp.average() as f32 * hp_multiplier).max(1.0) as i32;
    let mut c = CombatantState::new(
        combat_id,
        monster.name.clone(),
        false,
        max_hp,
        monster.ac as i32,
        monster.speed,
        AbilityScores::modifier(monster.dex_score) as i32,
        melee_bonus,
        melee_damage,
    );
    c.enemy_role = role;
    c.ranged_attack_bonus = ranged_attack_bonus;
    c.ranged_damage_dice = ranged_damage_dice;
    c.spell_attack_bonus = spell_attack_bonus;
    c.spell_damage_dice = spell_damage_dice;
    c.spell_on_hit_condition = spell_on_hit_condition;
    c.ranged_on_hit_condition = ranged_on_hit_condition;
    c.on_hit_condition = melee_on_hit_condition;
    c.damage_type = c_damage_type;
    c.ranged_damage_type = ranged_damage_type;
    c.spell_damage_type = spell_damage_type;
    c.resistances = monster.resistances.iter().cloned().collect();
    c.vulnerabilities = monster.vulnerabilities.iter().cloned().collect();
    c.immunities = monster.immunities.iter().cloned().collect();
    c.condition_immunities = monster.condition_immunities.iter().cloned().collect();
    c
}
