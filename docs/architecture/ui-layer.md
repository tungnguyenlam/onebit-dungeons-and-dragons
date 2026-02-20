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

Both renderers expose the same screen set, one per `AppState` variant.
Each screen in `src/ui/tui/screens/` uses Ratatui; each in
`src/ui/gui/screens/` uses egui.

| Screen | `AppState` variant | TUI file | GUI file |
|---|---|---|---|
| World Map | `WorldMap` | `tui/screens/world_map.rs` | `gui/screens/world_map.rs` |
| Combat | `Combat(_)` | `tui/screens/combat.rs` | `gui/screens/combat.rs` |
| Dialog | `Dialog(_)` | `tui/screens/dialog.rs` | `gui/screens/dialog.rs` |
| Journal | `Journal` | `tui/screens/journal.rs` | `gui/screens/journal.rs` |
| Inventory | `Inventory` | `tui/screens/inventory.rs` | `gui/screens/inventory.rs` |
| Spellbook | `Spellbook` | `tui/screens/spellbook.rs` | `gui/screens/spellbook.rs` |
| Character Sheet | overlay | `tui/screens/character_sheet.rs` | `gui/screens/character_sheet.rs` |

---

## Layout Convention

**TUI screens** define `fn render(f: &mut Frame, app: &App)`.
**GUI screens** define `fn draw(ui: &mut egui::Ui, app: &App)`.

The terminal is divided into three zones:

```
┌──────────────────────────────┐
│  MAIN AREA (80% height)      │
├─────────────┬────────────────┤
│  LOG PANEL  │  HUD / STATS   │
│  (10%)      │  (10%)         │
└─────────────┴────────────────┘
```

`src/ui/layout.rs` exports `Layout::split(frame_size) -> (main, log, hud)`
so all screens share the same zones.

---

## Reusable Widgets

- `widgets/dice_roll.rs` — animated dice result pop-up (shown for 1.5 s)
- `widgets/log.rs` — scrollable combat / story log fed from `game/story/journal.rs`
- `widgets/hud.rs` — HP bar, AC, spell slots, conditions row

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
