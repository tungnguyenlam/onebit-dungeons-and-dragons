# Architecture Overview

> **Index file.** Read this for orientation before diving into a specific layer.
> Links below point to detail docs.

---

## Module Boundary Rule

```
src/ui/      ← Ratatui rendering ONLY. Reads game state, never mutates it.
src/game/    ← Pure game logic. No Ratatui imports whatsoever.
src/data/    ← TOML deserialization + typed asset structs. No game logic.
src/app.rs   ← Glue: owns AppState, wires events → game → ui.
src/events.rs← Input event enum (key presses, ticks).
```

Violating the `ui` / `game` split is the single most important constraint
to maintain. It keeps both layers independently testable.

---

## Full Source Tree

```
src/
├── main.rs
├── app.rs
├── events.rs
├── ui/
│   ├── mod.rs
│   ├── layout.rs
│   ├── screens/
│   │   ├── world_map.rs
│   │   ├── combat.rs
│   │   ├── character_sheet.rs
│   │   ├── inventory.rs
│   │   ├── spellbook.rs
│   │   ├── dialog.rs
│   │   └── journal.rs
│   └── widgets/
│       ├── dice_roll.rs
│       ├── log.rs
│       └── hud.rs
├── game/
│   ├── mod.rs
│   ├── world/
│   │   ├── map.rs
│   │   ├── room.rs
│   │   ├── region.rs
│   │   └── fov.rs
│   ├── character/
│   │   ├── stats.rs
│   │   ├── class.rs
│   │   ├── race.rs
│   │   ├── skills.rs
│   │   ├── conditions.rs
│   │   └── progression.rs
│   ├── combat/
│   │   ├── initiative.rs
│   │   ├── actions.rs
│   │   ├── attack.rs
│   │   ├── damage.rs
│   │   ├── spells.rs
│   │   └── economy.rs
│   ├── items/
│   │   ├── inventory.rs
│   │   ├── equipment.rs
│   │   ├── weapons.rs
│   │   └── armor.rs
│   ├── npc/
│   │   ├── monster.rs
│   │   └── ai.rs
│   ├── dice/
│   │   ├── mod.rs
│   │   └── parser.rs
│   ├── story/
│   │   ├── mod.rs
│   │   ├── world_state.rs
│   │   ├── quest.rs
│   │   ├── dialog.rs
│   │   ├── journal.rs
│   │   └── events.rs
│   └── save/
│       └── serialization.rs
└── data/
    ├── loader.rs
    └── types.rs

assets/
├── regions/
│   └── <region-slug>/     ← one folder per region (see content/regions/index.md)
│       ├── region.toml
│       ├── rooms/
│       │   └── <room-id>.toml
│       ├── npcs/
│       │   └── <npc-id>.toml
│       └── dialog/
│           └── <npc-id>.toml
├── classes/
├── races/
├── monsters/
├── spells/
├── items/
├── quests/
│   ├── main/
│   └── side/
└── lore/
```

---

## Detail Docs

- App loop & tick logic → [game-loop.md](game-loop.md)
- UI screen state machine → [ui-layer.md](ui-layer.md)
- Asset loading pipeline → [data-pipeline.md](data-pipeline.md)
