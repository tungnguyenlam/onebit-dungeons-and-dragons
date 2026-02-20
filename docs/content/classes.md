# Classes

> Add each class as `assets/classes/<slug>.toml`.
> Schema defined in [gameplay/character.md](../gameplay/character.md).

---

## Planned Classes

| Slug | Name | Hit Die | Primary Ability | Spellcaster | Status |
|---|---|---|---|---|---|
| `barbarian` | Barbarian | d12 | STR | No | 🔲 |
| `bard` | Bard | d8 | CHA | Yes (CHA) | 🔲 |
| `cleric` | Cleric | d8 | WIS | Yes (WIS) | 🔲 |
| `druid` | Druid | d8 | WIS | Yes (WIS) | 🔲 |
| `fighter` | Fighter | d10 | STR or DEX | No (Eldritch Knight: partial) | 🔲 |
| `monk` | Monk | d8 | DEX + WIS | No | 🔲 |
| `paladin` | Paladin | d10 | STR | Yes (CHA, half-caster) | 🔲 |
| `ranger` | Ranger | d10 | DEX | Yes (WIS, half-caster) | 🔲 |
| `rogue` | Rogue | d8 | DEX | No (Arcane Trickster: partial) | 🔲 |
| `sorcerer` | Sorcerer | d6 | CHA | Yes (CHA) | 🔲 |
| `warlock` | Warlock | d8 | CHA | Yes (CHA, pact slots) | 🔲 |
| `wizard` | Wizard | d6 | INT | Yes (INT) | 🔲 |

---

## Priority for MVP

Author **Fighter**, **Rogue**, and **Wizard** first — they cover the core
archetypes (martial, skill, arcane) and exercise all major subsystems.

---

## Class TOML Example Skeleton

```toml
id                        = "fighter"
name                      = "Fighter"
hit_die                   = 10
primary_ability           = ["strength", "dexterity"]
saving_throw_proficiencies = ["strength", "constitution"]
armor_proficiencies       = ["all", "shields"]
weapon_proficiencies      = ["simple", "martial"]

[[features]]
level  = 1
name   = "Fighting Style"
effect = "choose_fighting_style"   # handled by class feature system

[[features]]
level  = 1
name   = "Second Wind"
effect = "second_wind"

[[features]]
level  = 2
name   = "Action Surge"
effect = "action_surge"

# ... continue through level 20
```
