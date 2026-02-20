# Architecture Overview

> **Index file.** Read this for orientation before diving into a specific layer.
> Links below point to detail docs.

---

## Module Boundary Rule

```
src/ui/tui/  ← Ratatui/Crossterm rendering [feature = "tui"]. Reads game state only.
src/ui/gui/  ← egui/eframe rendering      [feature = "gui"]. Reads game state only.
src/game/    ← Pure game logic. No renderer imports whatsoever.
src/data/    ← TOML deserialization + typed asset structs. No game logic.
src/renderer.rs ← GameRenderer trait + GameEvent enum (renderer-agnostic).
src/app.rs   ← Glue: owns AppState, wires GameEvent → game → renderer.
src/main.rs  ← CLI flag parsing, selects TuiRenderer or GuiRenderer at launch.
```

Violating the `ui` / `game` split is the single most important constraint
to maintain. It keeps both layers independently testable.

See → [renderer.md](renderer.md) for the full dual-renderer design.

---

## Full Source Tree

```
src/
├── main.rs          ← CLI --mode flag; dispatches to TuiRenderer or GuiRenderer
├── app.rs           ← renderer-agnostic AppState + event handling
├── renderer.rs      ← GameRenderer trait, GameEvent enum
├── ui/
│   ├── mod.rs
│   ├── tui/                     [feature = "tui"]
│   │   ├── mod.rs               ← TuiRenderer: impl GameRenderer
│   │   ├── layout.rs
│   │   ├── screens/
│   │   │   ├── world_map.rs
│   │   │   ├── combat.rs
│   │   │   ├── character_sheet.rs
│   │   │   ├── inventory.rs
│   │   │   ├── spellbook.rs
│   │   │   ├── dialog.rs
│   │   │   └── journal.rs
│   │   └── widgets/
│   │       ├── dice_roll.rs
│   │       ├── log.rs
│   │       └── hud.rs
│   └── gui/                     [feature = "gui"]
│       ├── mod.rs               ← GuiRenderer: impl GameRenderer + eframe::App
│       ├── screens/
│       │   ├── world_map.rs
│       │   ├── combat.rs
│       │   ├── character_sheet.rs
│       │   ├── inventory.rs
│       │   ├── spellbook.rs
│       │   ├── dialog.rs
│       │   └── journal.rs
│       └── widgets/
│           ├── dice_roll.rs
│           ├── log.rs
│           └── hud.rs
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
- Renderer abstraction (TUI vs GUI) → [renderer.md](renderer.md)
- UI screen state machine → [ui-layer.md](ui-layer.md)
- Asset loading pipeline → [data-pipeline.md](data-pipeline.md)
