# Gameplay Overview

> **Index.** Read this file to understand what gameplay systems exist, then
> follow a link to the specific system you need.

---

## Core Loop

1. Player explores a **region** tile-by-tile (turn-based movement).
2. Entering a room may trigger **dialog**, a **combat encounter**, or a
   **story event**.
3. After combat, the player collects items, gains XP, and potentially triggers
   **quest stage** advances.
4. The player can leave a region via a **travel node**, loading the next region.

---

## System Index

| System | Summary | Detail doc |
|---|---|---|
| Dice | DiceExpr parser, roll(), advantage/disadvantage | [dice.md](dice.md) |
| Combat | 5e action economy, initiative, attack, spells | [combat.md](combat.md) |
| Character | Ability scores, class, race, conditions, leveling | [character.md](character.md) |
| World | Region files, tile map, rooms, FOV, travel | [world.md](world.md) |
| Items | Inventory, equipment slots, weapons, armor | [items.md](items.md) |
| Spells | Spell slots, concentration, effect resolution | [spells.md](spells.md) |
| NPC AI | Monster turns, behaviour trees, factions | [npc-ai.md](npc-ai.md) |
| Story | WorldState flags, quest machine, emergent events | [story.md](story.md) |
| Dialog | Dialog tree format, condition syntax | [dialog.md](dialog.md) |
| Journal | Entry lifecycle, trigger types | [journal.md](journal.md) |

---

## Related Indexes

- Architecture index → [../architecture/overview.md](../architecture/overview.md)
- Content index → [../content/overview.md](../content/overview.md)
- Active tasks → [../tasks/current-sprint.md](../tasks/current-sprint.md)
- Documentation link map → [../DOCS_MAP.md](../DOCS_MAP.md)
