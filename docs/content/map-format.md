# Map & Region File Format

> **The authoritative TOML schema reference for all region & room files.**
> An agent authoring a region should read this file plus the region's entry in
> [regions/index.md](regions/index.md). No other docs are needed to author maps.

---

## Folder Layout for One Region

```

Starter templates for these files live in `docs/content/regions/templates/`.
assets/regions/<slug>/
├── region.toml             ← required: manifest
├── rooms/
│   └── <room-id>.toml      ← one file per room
├── npcs/
│   └── <npc-id>.toml       ← NPC stat/metadata (references global monster or custom)
└── dialog/
    └── <npc-id>.toml       ← dialog tree (see gameplay/dialog.md for full schema)
```

---

## `region.toml` Full Schema

```toml
slug        = "valley-of-ash"           # must match folder name
name        = "Valley of Ash"
description = "A scorched valley…"      # shown in travel menu
entry_room  = "gate"                    # room id player spawns in
ambient     = "ash_drift"               # optional ambient flavour tag

[[rooms]]
id   = "gate"
file = "rooms/gate.toml"

[[rooms]]
id   = "ash_plain"
file = "rooms/ash_plain.toml"

[[connections]]
# Exit from this region to another
from_room  = "north_pass"
to_region  = "emberpeak-summit"
to_room    = "south_slope"
label      = "Head north toward Emberpeak"
condition  = "flag:act1_complete"        # empty string = always open
```

---

## `rooms/<id>.toml` Full Schema

```toml
id          = "gate"
name        = "The Ash Gate"
description = "Crumbling stone archway, ash drifting through the gap."
landmark    = "Ash Gate Landmark"

# Tile grid: exactly as rendered.
# Max 40 cols × 20 rows. Pad with spaces if needed.
# Tile legend — see gameplay/world.md
grid = """
##########
#........#
#..@.....#
#......!.#
##########
"""

[[npcs]]
id       = "guard-kael"      # references assets/regions/<slug>/npcs/guard-kael.toml
position = [3, 2]            # [col, row], 0-indexed, must match @ in grid

[[items]]
id       = "iron-longsword"  # references assets/items/iron-longsword.toml
position = [6, 3]
quantity = 1
condition = ""               # optional: only spawn if WorldState condition true

[[triggers]]
position  = [7, 3]           # matches ! in grid
type      = "dialog"         # dialog | encounter | lore | quest_stage | travel
target_id = "kael-first-meet"
condition = "not flag:met_kael"
once      = true             # fire only the first time player steps on tile

[exits]
north = "room_north"
east  = "room_east"
south = "room_south"
west  = "room_west"
```

---

## `npcs/<id>.toml` Schema

```toml
id          = "guard-kael"
name        = "Kael"
monster_ref = "city-guard"   # base stats from assets/monsters/city-guard.toml
                             # omit to define fully custom stats inline
faction     = "city-guard"
dialog_ref  = "guard-kael"   # file: dialog/guard-kael.toml

# Optional stat overrides (applied on top of monster_ref)
[overrides]
hp = 18
```

---

## Trigger Types Reference

| type | target_id points to | Effect |
|---|---|---|
| `dialog` | `dialog/<id>.toml` node id | Open dialog screen |
| `encounter` | encounter id in `assets/encounters/` | Start combat |
| `lore` | lore entry id in `assets/lore/` | Show lore popup + add journal entry |
| `quest_stage` | `<quest-id>:<stage-id>` | Force advance quest stage |
| `travel` | `<region-slug>:<room-id>` or `<room-id>` | Travel to another region or another room in the same region |

In runtime movement, normal intra-region traversal uses `[exits]` and border pushes.
`travel` triggers are reserved for special transitions (portals/stairs/conditioned jumps).

---

## Tile Legend (full)

| Char | Tile |
|------|------|
| `#` | Wall (impassable) |
| `.` | Floor |
| ` ` | Void (outside room boundary) |
| `+` | Door (closed, passable) |
| `-` | Door (open) |
| `~` | Water (difficult terrain) |
| `^` | Stairs / ladder up |
| `v` | Stairs / ladder down |
| `X` | Chest / interactable object |
| `@` | NPC spawn (replaced by NPC glyph at runtime) |
| `!` | Trigger zone (invisible at runtime unless debug mode) |
| `T` | Tree / pillar (impassable, provides cover) |
| `=` | Counter / table (impassable) |

---

## Validation Workflow

Run:

```bash
scripts/validate_content.sh
```

This executes loader-backed tests that verify all authored regions and quests
in `assets/` deserialize correctly.
