# Character

## Ability Scores

Six scores: **STR, DEX, CON, INT, WIS, CHA** (range 1–20 for normal creatures,
up to 30 for epic monsters).

Modifier = `floor((score - 10) / 2)` (range −5 to +5).

Stored in `src/game/character/stats.rs`:

```rust
pub struct AbilityScores {
    pub strength:     u8,
    pub dexterity:    u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom:       u8,
    pub charisma:     u8,
}
impl AbilityScores {
    pub fn modifier(&self, score: u8) -> i8 { ((score as i8 - 10) / 2) }
}
```

---

## Proficiency Bonus

Determined by total character level:

| Level | Bonus |
|-------|-------|
| 1–4   | +2    |
| 5–8   | +3    |
| 9–12  | +4    |
| 13–16 | +5    |
| 17–20 | +6    |

---

## HP

`max_hp = hit_die_average_or_roll + CON_modifier` per level.
Level 1 always uses max hit die value.
Current HP tracked separately; temporary HP is an overlay (absorbed first).

---

## Skills

18 skills each tied to an ability. See `src/game/character/skills.rs`.
Skill check = `1d20 + ability_modifier [+ proficiency_bonus if proficient]`.

Expertise: double proficiency bonus on that skill.

---

## Classes

Defined in `assets/classes/<class-slug>.toml`. Each class file declares:
- `hit_die` (d6 / d8 / d10 / d12)
- `primary_ability`
- `saving_throw_proficiencies` (two abilities)
- `armor_proficiencies`, `weapon_proficiencies`
- `features` table keyed by level (name, description, mechanical_effect)
- `subclasses` (chosen at level 3 or class-specified level)
- `spell_slots` table (if spellcaster)

For full class list → [content/classes.md](../content/classes.md)

---

## Races

Defined in `assets/races/<race-slug>.toml`. Each declares:
- Ability score increases
- Speed
- Size
- Racial traits (name, description, mechanical_effect)
- Languages

For full race list → [content/races.md](../content/races.md)

---

## Conditions

Stored as `HashSet<Condition>` on each entity. Each condition has:
- Duration (turns, or "until removed")
- Effect (applied in `src/game/character/conditions.rs`)

Full 5e SRD condition list: Blinded, Charmed, Deafened, Exhaustion (1–6),
Frightened, Grappled, Incapacitated, Invisible, Paralyzed, Petrified,
Poisoned, Prone, Restrained, Stunned, Unconscious.

---

## Leveling & XP

XP thresholds follow standard 5e SRD table. On level-up:
1. Roll (or take average) hit die + CON modifier, add to max HP.
2. May gain new class features (check class TOML `features` table).
3. At levels 4, 8, 12, 16, 19: Ability Score Improvement (+2 to one, or +1 to two).

`src/game/character/progression.rs`

---

## Character Sheet UI

`src/ui/screens/character_sheet.rs` — overlay showing:
- Ability scores + modifiers
- HP / AC / Speed
- Proficiency bonus
- Skill list with proficiency markers
- Class features summary
- Conditions active
