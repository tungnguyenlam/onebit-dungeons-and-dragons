# UI Layer

## Principle

`src/ui/` contains **only** rendering code. It receives immutable references to
game state and produces pixel-perfect Ratatui `Frame` output. It imports
nothing from `src/game/`. Game mutations happen exclusively in `src/game/` in
response to `Event`s processed by `src/app.rs`.

---

## Screen Modules

| Module | Shown when |
|---|---|
| `screens/world_map.rs` | `AppState::WorldMap` |
| `screens/combat.rs` | `AppState::Combat(_)` |
| `screens/dialog.rs` | `AppState::Dialog(_)` |
| `screens/journal.rs` | `AppState::Journal` |
| `screens/inventory.rs` | `AppState::Inventory` |
| `screens/spellbook.rs` | `AppState::Spellbook` |
| `screens/character_sheet.rs` | overlay, accessible from WorldMap |

---

## Layout Convention

Every screen defines a `fn render(f: &mut Frame, app: &App)` function.

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
