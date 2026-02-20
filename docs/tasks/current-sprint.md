# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Milestone 1 — world module not yet started
Task in progress: M1 core systems — world/region/FOV

What was completed this session:
  Milestone 0 (complete):
    - Cargo.toml with tui/gui feature flags + clap
    - src/renderer.rs     — GameRenderer trait, GameEvent enum, ControlFlow
    - src/app.rs          — App, AppState, per-screen event dispatch stubs
    - src/main.rs         — CLI --mode flag, run_loop, TUI/GUI dispatch
    - src/ui/mod.rs       — cfg-gated sub-module declarations
    - src/ui/tui/mod.rs   — TuiRenderer: impl GameRenderer (crossterm key map)
    - src/ui/gui/mod.rs   — GuiRenderer stub (deferred — TUI first)
    - `cargo check` passes clean (0 errors, 0 warnings)

  Milestone 1 (partial):
    - src/game/dice/      — DiceExpr, roll/advantage/disadvantage, serde, tests
    - src/data/types.rs   — all TOML asset serde structs
    - src/data/loader.rs  — load<T>, load_region, load_global_assets
    - src/game/character/ — AbilityScores, Character, Skill, Condition, progression
    - src/game/items/     — Inventory, EquipmentSlots, armor AC
    - README.md           — project overview + file structure

What is NOT done yet:
    - src/game/world/     — region loader, tile map, room graph, FOV  ← NEXT
    - src/game/story/     — WorldState, quest machine, dialog evaluator
    - src/game/combat/    — initiative, attack rolls, action economy
    - src/ui/tui/screens/ — all screen render functions
    - src/ui/tui/layout.rs, widgets/
    - game/ and data/ modules NOT wired into app.rs yet (stubs only)

Next action for the incoming agent:
  1. `cargo check` — must pass before touching anything.
  2. Implement src/game/world/ (see docs/gameplay/world.md):
       - world/map.rs    — Tile enum, TileGrid (40x20 char grid), passability
       - world/room.rs   — Room struct loaded from RoomDef asset
       - world/region.rs — Region struct loaded from LoadedRegion
       - world/fov.rs    — shadowcasting FOV, returns visible tile set
       - world/mod.rs
  3. Then implement src/game/story/world_state.rs (WorldState flag store).
  4. Then uncomment game/mod.rs sub-modules and wire into app.rs.

Files modified this session:
  Cargo.toml, src/** (all new), README.md,
  docs/architecture/overview.md, game-loop.md, ui-layer.md, renderer.md (new),
  docs/tasks/backlog.md, current-sprint.md, done.md

Blockers: none
```

---

## Active Task

### Task: World module (Milestone 1, step 5)

**Files to create:**
- `src/game/world/map.rs`    — `Tile` enum, `TileGrid`, passability lookup
- `src/game/world/room.rs`   — `Room` built from `RoomDef` asset
- `src/game/world/region.rs` — `Region` built from `LoadedRegion`
- `src/game/world/fov.rs`    — recursive shadowcasting FOV
- `src/game/world/mod.rs`

**Done when:**
- [ ] `cargo check` still passes
- [ ] `TileGrid::from_str` parses a room grid string
- [ ] `fov::compute` returns the set of visible tile positions
- [ ] `Room::from_def` builds a room from a `RoomDef`

**Blocked by:** `src/data/types.rs` (done)

**Relevant docs:**
- [../gameplay/world.md](../gameplay/world.md)
- [../content/map-format.md](../content/map-format.md)

---

## Acceptance Criteria Template

When pulling a new task from the backlog, replace the Active Task block above
with a copy of this template, then update the Handoff block.

```
### Task: <name>

**Files to touch:**
- src/...

**Done when:**
- [ ] criterion 1
- [ ] criterion 2

**Blocked by:** (none / task name)

**Relevant docs:**
- docs/...
```
