# UI Layer

## Principle

`src/ui/` contains **only** rendering code. It receives immutable references to
game state and produces output — either a Ratatui `Frame` (TUI) or an egui
`Ui` (GUI). Neither sub-module imports from `src/game/`. Both implement the
`GameRenderer` trait defined in `src/renderer.rs`.

Game mutations happen exclusively in `src/game/` in response to `GameEvent`s
processed by `src/app.rs`.

See → [renderer.md](renderer.md) for the trait definition and launch mechanics.

---

## Screen Modules

The TUI currently has dedicated screen modules for combat, dialog, journal,
inventory, and spellbook. World map and menu-like states still use the
placeholder renderer branch in `src/ui/tui/mod.rs`.

GUI remains a stub in `src/ui/gui/mod.rs` and does not yet have per-screen
modules.

| Screen | `AppState` variant | TUI file | GUI file |
|---|---|---|---|
| World Map | `WorldMap` | placeholder branch in `tui/mod.rs` | stub (`gui/mod.rs`) |
| Combat | `Combat(_)` | `tui/screens/combat.rs` | stub (`gui/mod.rs`) |
| Dialog | `Dialog(_)` | `tui/screens/dialog.rs` | stub (`gui/mod.rs`) |
| Journal | `Journal` | `tui/screens/journal.rs` | stub (`gui/mod.rs`) |
| Inventory | `Inventory` | `tui/screens/inventory.rs` | stub (`gui/mod.rs`) |
| Spellbook | `Spellbook` | `tui/screens/spellbook.rs` | stub (`gui/mod.rs`) |
| Character Sheet | overlay | not implemented | not implemented |

---

## Layout Convention

TUI screen modules define `fn render(f: &mut Frame, app: &App)`. They currently
build layouts inline per screen; shared `layout.rs` and reusable `widgets/`
modules are deferred to a later UI polish milestone.

Most current screens split the terminal into a main panel and secondary info
rows similar to:

```
┌──────────────────────────────┐
│  MAIN AREA (80% height)      │
├─────────────┬────────────────┤
│  LOG PANEL  │  HUD / STATS   │
│  (10%)      │  (10%)         │
└─────────────┴────────────────┘
```

No shared layout helper is in use yet.

---

## World Map Screen

The world map renders the current region's tile grid using block characters.
Visible tiles are determined by FOV (see [gameplay/world.md](../gameplay/world.md)).
Each region fits in one screen. There is **no scrolling mega-map** — the player
moves between regions via explicit travel actions, which load a new region file.

---

## Dialog Screen

Renders the current dialog node text in a centered popup, with numbered choice
options below. Pressing `1`–`9` selects a choice. Dialog context (current node,
NPC name) is stored in `AppState::Dialog(DialogContext)`.

See → [gameplay/dialog.md](../gameplay/dialog.md)
