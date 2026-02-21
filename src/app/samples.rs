use crate::data::loader::{load_monsters, load_region};
use crate::data::types::{
    ArmorDef, ArmorType, DialogTree, ItemBonuses, ItemDef, ItemType, MonsterAction,
    MonsterDef, NpcDef, WeaponDef, SpellDef,
}; // Careful: check types here
use crate::game::{
    character::{AbilityScores, conditions::Condition},
    combat::CombatantState,
    dice::DiceExpr,
    combat::EnemyAiRole,
    world::region::Region,
    story::journal::Category as JournalCategory,
    story::events::{EventEngine, EventTrigger, WorldEvent},
};
use std::collections::HashMap;

pub fn sample_region_bundle() -> (Region, HashMap<String, NpcDef>, HashMap<String, DialogTree>) {
    let loaded = load_region("assets", "valley-of-ash").ok();
    if let Some(loaded) = loaded {
        return (Region::from_loaded(&loaded), loaded.npcs, loaded.dialogs);
    }

    let fallback = crate::data::loader::LoadedRegion {
        manifest: crate::data::types::RegionManifest {
            slug: "fallback".into(),
            name: "Fallback Region".into(),
            description: "Fallback region when assets are unavailable.".into(),
            entry_room: "start".into(),
            ambient: "".into(),
            rooms: vec![crate::data::types::RoomRef {
                id: "start".into(),
                file: "rooms/start.toml".into(),
            }],
            connections: vec![],
        },
        rooms: {
            let mut map = HashMap::new();
            map.insert(
                "start".into(),
                crate::data::types::RoomDef {
                    id: "start".into(),
                    name: "Start".into(),
                    description: "Fallback room".into(),
                    grid: "#####\n#...#\n#.@.#\n#####\n".into(),
                    terminal: false,
                    npcs: vec![],
                    items: vec![],
                    triggers: vec![],
                },
            );
            map
        },
        npcs: HashMap::new(),
        dialogs: HashMap::new(),
    };
    (
        Region::from_loaded(&fallback),
        HashMap::new(),
        HashMap::new(),
    )
}

pub fn sample_item_defs() -> HashMap<String, ItemDef> {
    let mut map = HashMap::new();
    map.insert(
        "longsword".into(),
        ItemDef {
            id: "longsword".into(),
            name: "Longsword".into(),
            item_type: ItemType::Weapon,
            weight: 3.0,
            value_gp: 15,
            description: "A standard steel longsword.".into(),
            weapon: Some(WeaponDef {
                damage: DiceExpr::new(1, 8, 0),
                damage_type: "slashing".into(),
                properties: vec!["versatile".into()],
                versatile_damage: Some(DiceExpr::new(1, 10, 0)),
                range: None,
            }),
            armor: None,
            bonuses: ItemBonuses::default(),
        },
    );
    map.insert(
        "leather_armor".into(),
        ItemDef {
            id: "leather_armor".into(),
            name: "Leather Armor".into(),
            item_type: ItemType::Armor,
            weight: 10.0,
            value_gp: 10,
            description: "Flexible light armor.".into(),
            weapon: None,
            armor: Some(ArmorDef {
                base_ac: 11,
                armor_type: ArmorType::Light,
                stealth_disadvantage: false,
            }),
            bonuses: ItemBonuses::default(),
        },
    );
    map.insert(
        "shield".into(),
        ItemDef {
            id: "shield".into(),
            name: "Shield".into(),
            item_type: ItemType::Armor,
            weight: 6.0,
            value_gp: 10,
            description: "Wooden shield.".into(),
            weapon: None,
            armor: Some(ArmorDef {
                base_ac: 2,
                armor_type: ArmorType::Shield,
                stealth_disadvantage: false,
            }),
            bonuses: ItemBonuses {
                armor_class_bonus: 0,
                ..ItemBonuses::default()
            },
        },
    );
    map.insert(
        "healing_potion".into(),
        ItemDef {
            id: "healing_potion".into(),
            name: "Healing Potion".into(),
            item_type: ItemType::Consumable,
            weight: 0.5,
            value_gp: 50,
            description: "Restores health.".into(),
            weapon: None,
            armor: None,
            bonuses: ItemBonuses::default(),
        },
    );
    map
}

pub fn sample_spell_defs() -> HashMap<String, SpellDef> {
    let mut map = HashMap::new();
    map.insert(
        "cure_wounds".into(),
        SpellDef {
            id: "cure_wounds".into(),
            name: "Cure Wounds".into(),
            level: 1,
            school: "evocation".into(),
            casting_time: "action".into(),
            range: "touch".into(),
            components: vec!["V".into(), "S".into()],
            duration: "instantaneous".into(),
            description: "Healing energy restores HP.".into(),
            damage: None,
            damage_type: None,
            save: None,
            heal: Some(DiceExpr::new(1, 8, 2)),
            classes: vec!["cleric".into()],
        },
    );
    map.insert(
        "fire_bolt".into(),
        SpellDef {
            id: "fire_bolt".into(),
            name: "Fire Bolt".into(),
            level: 0,
            school: "evocation".into(),
            casting_time: "action".into(),
            range: "120ft".into(),
            components: vec!["V".into(), "S".into()],
            duration: "instantaneous".into(),
            description: "A mote of fire.".into(),
            damage: Some(DiceExpr::new(1, 10, 0)),
            damage_type: Some("fire".into()),
            save: None,
            heal: None,
            classes: vec!["wizard".into()],
        },
    );
    map.insert(
        "poison_spray".into(),
        SpellDef {
            id: "poison_spray".into(),
            name: "Poison Spray".into(),
            level: 0,
            school: "conjuration".into(),
            casting_time: "action".into(),
            range: "10ft".into(),
            components: vec!["V".into(), "S".into()],
            duration: "instantaneous".into(),
            description: "Noxious gas.".into(),
            damage: None,
            damage_type: None,
            save: Some("constitution".into()),
            heal: None,
            classes: vec!["wizard".into()],
        },
    );
    map
}

pub fn sample_monster_defs() -> HashMap<String, MonsterDef> {
    let mut map = HashMap::new();
    map.insert(
        "goblin".into(),
        MonsterDef {
            id: "goblin".into(),
            name: "Goblin".into(),
            cr: 0.25,
            size: "small".into(),
            monster_type: "humanoid".into(),
            alignment: "neutral_evil".into(),
            hp: DiceExpr::new(2, 6, 0),
            ac: 13,
            speed: 30,
            str_score: 8,
            dex_score: 14,
            con_score: 10,
            int_score: 10,
            wis_score: 8,
            cha_score: 8,
            xp: 50,
            actions: vec![MonsterAction {
                name: "Club".into(),
                description: "Melee attack".into(),
                attack_bonus: Some(4),
                damage: Some(DiceExpr::new(1, 6, 2)),
                damage_type: Some("bludgeoning".into()),
                on_hit_condition: None,
            }],
            traits: vec![],
            resistances: vec![],
            vulnerabilities: vec![],
        },
    );
    map.insert(
        "goblin_archer".into(),
        MonsterDef {
            id: "goblin_archer".into(),
            name: "Goblin Archer".into(),
            cr: 0.25,
            size: "small".into(),
            monster_type: "humanoid".into(),
            alignment: "neutral_evil".into(),
            hp: DiceExpr::new(2, 6, 0),
            ac: 13,
            speed: 30,
            str_score: 8,
            dex_score: 14,
            con_score: 10,
            int_score: 10,
            wis_score: 8,
            cha_score: 8,
            xp: 50,
            actions: vec![
                MonsterAction {
                    name: "Scimitar".into(),
                    description: "Melee attack".into(),
                    attack_bonus: Some(4),
                    damage: Some(DiceExpr::new(1, 6, 2)),
                    damage_type: Some("slashing".into()),
                    on_hit_condition: None,
                },
                MonsterAction {
                    name: "Shortbow".into(),
                    description: "Ranged attack".into(),
                    attack_bonus: Some(4),
                    damage: Some(DiceExpr::new(1, 6, 2)),
                    damage_type: Some("piercing".into()),
                    on_hit_condition: None,
                },
            ],
            traits: vec![],
            resistances: vec![],
            vulnerabilities: vec![],
        },
    );
    map.insert(
        "goblin_shaman".into(),
        MonsterDef {
            id: "goblin_shaman".into(),
            name: "Goblin Shaman".into(),
            cr: 0.5,
            size: "small".into(),
            monster_type: "humanoid".into(),
            alignment: "neutral_evil".into(),
            hp: DiceExpr::new(3, 6, 0),
            ac: 12,
            speed: 30,
            str_score: 8,
            dex_score: 12,
            con_score: 10,
            int_score: 12,
            wis_score: 14,
            cha_score: 10,
            xp: 100,
            actions: vec![
                MonsterAction {
                    name: "Dagger".into(),
                    description: "Melee attack".into(),
                    attack_bonus: Some(3),
                    damage: Some(DiceExpr::new(1, 4, 1)),
                    damage_type: Some("piercing".into()),
                    on_hit_condition: None,
                },
                MonsterAction {
                    name: "Poison Bolt".into(),
                    description: "Spell attack".into(),
                    attack_bonus: Some(4),
                    damage: Some(DiceExpr::new(1, 8, 1)),
                    damage_type: Some("poison".into()),
                    on_hit_condition: Some("poisoned".into()),
                },
            ],
            traits: vec![],
            resistances: vec![],
            vulnerabilities: vec![],
        },
    );
    map
}

pub fn demo_world_events() -> EventEngine {
    EventEngine {
        triggers: vec![
            EventTrigger {
                condition: "counter:faction_town_guard_rep >= 3".into(),
                event: WorldEvent::AddJournalEntry {
                    id: "faction-town-guard-friendly".into(),
                    category: JournalCategory::World,
                    title: "Town Guard Trust".into(),
                    body: "The town guard now recognizes your service and offers support.".into(),
                },
                once: true,
                fired: false,
            },
            EventTrigger {
                condition: "counter:faction_town_guard_rep >= 2".into(),
                event: WorldEvent::SetFlag {
                    key: "town_guard_trusted".into(),
                },
                once: true,
                fired: false,
            },
            EventTrigger {
                condition: "counter:faction_goblin_tribe_rep <= -4".into(),
                event: WorldEvent::SetFlag {
                    key: "goblin_tribe_hostile".into(),
                },
                once: true,
                fired: false,
            },
            EventTrigger {
                condition: "flag:town_guard_trusted && flag:read_ember_rune".into(),
                event: WorldEvent::AddJournalEntry {
                    id: "chain-emberpeak-briefing".into(),
                    category: JournalCategory::World,
                    title: "Joint War Council".into(),
                    body: "The guard and summit wardens coordinate supply lines through Emberpeak."
                        .into(),
                },
                once: true,
                fired: false,
            },
            EventTrigger {
                condition: "flag:town_guard_trusted && counter:faction_goblin_tribe_rep <= -3"
                    .into(),
                event: WorldEvent::SetFlag {
                    key: "valley_warfront".into(),
                },
                once: true,
                fired: false,
            },
        ],
    }
}

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
    c
}
