# World & Region System

## Design Principle: Region Isolation

The world is **not** a single large map. It is a collection of independent
**regions**. Each region is a self-contained folder:

```
assets/regions/<region-slug>/
├── region.toml         ← manifest: name, description, rooms list, connections
├── rooms/
│   ├── <room-id>.toml  ← tile grid, entities, items, triggers
│   └── ...
├── npcs/
│   └── <npc-id>.toml   ← stat block overrides, faction
└── dialog/
    └── <npc-id>.toml   ← dialog tree for this NPC in this region
```

**An agent authoring or modifying a region only reads files in that
region's folder.** The region index at [content/regions/index.md](../content/regions/index.md)
lists all regions and their connections, but the full detail of each region
stays in its own folder.

---

## region.toml Schema

```toml
slug        = "valley-of-ash"
name        = "Valley of Ash"
description = "A scorched valley at the foot of Emberpeak."
entry_room  = "gate"
music       = "ominous_wind"

[[rooms]]
id = "gate"
file = "rooms/gate.toml"

[[rooms]]
id = "ash_plain"
file = "rooms/ash_plain.toml"

[[connections]]
# travel nodes to adjacent regions
from_room = "north_pass"
to_region = "emberpeak-summit"
to_room   = "south_slope"
label     = "Head north toward Emberpeak"
condition = ""   # optional WorldState condition (empty = always accessible)
```

---

## Room Tile Grid

Each room is a 2D char grid (max 40 × 20 tiles to fit one TUI screen).
Tile legend is defined per-project:

| Char | Meaning |
|------|---------|
| `#` | Wall |
| `.` | Floor |
| `+` | Door (closed) |
| `-` | Door (open) |
| `~` | Water |
| `^` | Stairs up |
| `v` | Stairs down |
| `X` | Chest / interactable |
| `@` | NPC spawn point |
| `!` | Trigger zone (encounter, dialog, lore) |
| `O` | Pit hazard (damage + prone risk) |
| `R` | Rift hazard (rope-assisted crossing or damage/restraint) |

---

## room.toml Schema

```toml
id          = "gate"
name        = "The Ash Gate"
description = "Crumbling stone archway, ash drifting through the gap."

grid = """
##########
#........#
#..@..!..#
#........#
##########
"""

[[npcs]]
id       = "guard_kael"
position = [3, 2]    # [col, row] matching @ in grid

[[triggers]]
position  = [7, 2]    # matches ! in grid
type      = "dialog"  # dialog | encounter | lore | quest_stage
target_id = "kael_encounter_1"
condition = "not flag:met_kael"

---

## Weather & Visibility

Regions declare weather in `region.toml` (`clear`, `rain`, `fog`, `ash`, etc.).

- `fog` reduces visible map radius in the TUI.
- `rain` penalizes ranged attacks and weakens fire-based attacks in combat.
- `ash` periodically applies coughing pressure (modeled via temporary poisoned state).

Visibility masking is applied for **fog only**. Other weather types keep full
room visibility.

The TUI world screen now combines:
- **Local Map** (tile-level room view)
- **World Map Widget** (room list + available/locked regional exits)
