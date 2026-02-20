# Spells List

> Spell TOML files live in `assets/spells/<id>.toml`.
> Full schema in [gameplay/spells.md](../gameplay/spells.md).

---

## Priority: Author These First

These spells cover the most common use-cases and should be implemented before
others (exercises damage, healing, condition, and utility paths):

| ID | Name | Level | School | Type | Status |
|---|---|---|---|---|---|
| `fire-bolt` | Fire Bolt | 0 (cantrip) | Evocation | damage | 🔲 |
| `mage-hand` | Mage Hand | 0 | Conjuration | utility | 🔲 |
| `minor-illusion` | Minor Illusion | 0 | Illusion | utility | 🔲 |
| `cure-wounds` | Cure Wounds | 1 | Evocation | heal | 🔲 |
| `magic-missile` | Magic Missile | 1 | Evocation | damage (auto) | 🔲 |
| `shield` | Shield | 1 | Abjuration | reaction/AC | 🔲 |
| `sleep` | Sleep | 1 | Enchantment | condition | 🔲 |
| `burning-hands` | Burning Hands | 1 | Evocation | damage AoE | 🔲 |
| `hold-person` | Hold Person | 2 | Enchantment | condition | 🔲 |
| `misty-step` | Misty Step | 2 | Conjuration | utility | 🔲 |
| `shatter` | Shatter | 2 | Evocation | damage AoE | 🔲 |
| `fireball` | Fireball | 3 | Evocation | damage AoE | 🔲 |
| `counterspell` | Counterspell | 3 | Abjuration | reaction | 🔲 |
| `lightning-bolt` | Lightning Bolt | 3 | Evocation | damage line | 🔲 |
| `greater-invisibility` | Greater Invisibility | 4 | Illusion | condition | 🔲 |
| `polymorph` | Polymorph | 4 | Transmutation | transform | 🔲 |
| `cone-of-cold` | Cone of Cold | 5 | Evocation | damage cone | 🔲 |
| `hold-monster` | Hold Monster | 5 | Enchantment | condition | 🔲 |

---

## Cantrips by Class

| Cantrip | Wizard | Sorcerer | Warlock | Bard | Cleric | Druid |
|---|---|---|---|---|---|---|
| Fire Bolt | ✓ | ✓ | ✓ | | | |
| Mage Hand | ✓ | ✓ | ✓ | ✓ | | |
| Minor Illusion | ✓ | ✓ | ✓ | ✓ | | |
| Sacred Flame | | | | | ✓ | |
| Shillelagh | | | | | | ✓ |
