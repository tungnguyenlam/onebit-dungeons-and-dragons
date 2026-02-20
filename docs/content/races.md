# Races

> Add each race as `assets/races/<slug>.toml`.
> Schema defined in [gameplay/character.md](../gameplay/character.md).

---

## Planned Races

| Slug | Name | ASI | Speed | Status |
|---|---|---|---|---|
| `human` | Human | +1 all | 30 ft | 🔲 |
| `elf-high` | High Elf | +2 DEX, +1 INT | 30 ft | 🔲 |
| `elf-wood` | Wood Elf | +2 DEX, +1 WIS | 35 ft | 🔲 |
| `dwarf-hill` | Hill Dwarf | +2 CON, +1 WIS | 25 ft | 🔲 |
| `dwarf-mountain` | Mountain Dwarf | +2 CON, +2 STR | 25 ft | 🔲 |
| `halfling-lightfoot` | Lightfoot Halfling | +2 DEX, +1 CHA | 25 ft | 🔲 |
| `gnome-forest` | Forest Gnome | +2 INT, +1 DEX | 25 ft | 🔲 |
| `half-orc` | Half-Orc | +2 STR, +1 CON | 30 ft | 🔲 |
| `tiefling` | Tiefling | +2 CHA, +1 INT | 30 ft | 🔲 |
| `dragonborn` | Dragonborn | +2 STR, +1 CHA | 30 ft | 🔲 |

---

## Race TOML Example Skeleton

```toml
id     = "elf-high"
name   = "High Elf"
size   = "medium"
speed  = 30
languages = ["common", "elvish"]

[asi]                        # ability score increases
dexterity    = 2
intelligence = 1

[[traits]]
name   = "Darkvision"
effect = "darkvision_60ft"

[[traits]]
name   = "Keen Senses"
effect = "proficiency_perception"

[[traits]]
name   = "Fey Ancestry"
effect = "advantage_charmed_saves"

[[traits]]
name   = "Trance"
effect = "trance_rest"          # 4 hrs replaces long rest

[[traits]]
name   = "Elf Weapon Training"
effect = "proficiency_elf_weapons"

[[traits]]
name   = "Cantrip"
effect = "free_cantrip_wizard"
```
