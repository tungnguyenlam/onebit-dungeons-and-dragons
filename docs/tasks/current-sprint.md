# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Milestone 7 complete — all planned milestones implemented
Task in progress: Post-milestone hardening — reduce warnings and integration polish

What was completed this session:
  Milestone 6:
    - assets/regions/valley-of-ash/region.toml + rooms + npcs + dialog authored
    - assets/quests/main + assets/quests/side authored for first-region progression
    - assets/lore + assets/items + assets/spells + assets/monsters seeded for runtime loading
    - src/app.rs — region/room runtime state, movement, trigger interaction, travel wiring
    - src/data/loader.rs — authored region smoke test
  Milestone 7:
    - src/game/save/mod.rs — save/load TOML serialization runtime + tests
    - src/renderer.rs + src/ui/tui/mod.rs + src/ui/gui/mod.rs — save/load/sound events and key mappings
    - src/ui/tui/screens/ — main menu, world map, character creation, game over screens
    - README.md — screenshots and updated controls/status
    - cargo test — 98 tests, 0 failures
  Milestone 6/7 status:
    - all backlog checklist items now complete

What is NOT done yet:
    - warning cleanup (unused re-exports and dead-code across modules)
    - GUI remains a functional stub compared with TUI
    - deeper content/balance pass for Valley of Ash

Next action for the incoming agent:
  1. cargo test — must pass (98 tests) before touching anything.
  2. Tackle warning reduction by removing stale re-exports and dead demo code.
  3. Add end-to-end integration tests for world-map trigger -> combat/dialog transitions.
  4. If GUI milestone is desired, mirror the new TUI screens in src/ui/gui/.

Files modified this session:
  Cargo.toml
  src/app.rs
  src/renderer.rs
  src/game/mod.rs
  src/game/save/mod.rs (new)
  src/data/loader.rs
  src/ui/tui/mod.rs
  src/ui/tui/screens/mod.rs
  src/ui/tui/screens/main_menu.rs (new)
  src/ui/tui/screens/character_creation.rs (new)
  src/ui/tui/screens/world_map.rs (new)
  src/ui/tui/screens/game_over.rs (new)
  src/ui/tui/screens/combat.rs
  src/ui/gui/mod.rs
  assets/regions/valley-of-ash/* (new)
  assets/quests/main/* (new)
  assets/quests/side/* (new)
  assets/lore/* (new)
  assets/items/* (new)
  assets/spells/* (new)
  assets/monsters/* (new)
  docs/content/regions/index.md
  README.md
  docs/tasks/backlog.md
  docs/tasks/done.md
  docs/tasks/current-sprint.md (this file)

Blockers: none
```

---

## Active Task

### Task: Post-Milestone Hardening

**Files to touch:**
- `src/game/*/mod.rs` — remove/adjust stale re-exports
- `src/app.rs` — remove deprecated demo-only paths
- `src/ui/gui/mod.rs` — improve parity with TUI flows

**Done when:**
- [ ] `cargo test` passes
- [ ] warnings materially reduced from current baseline
- [ ] no milestone docs are stale
- [ ] basic GUI path supports menu/world/combat transitions

**Blocked by:** none

**Relevant docs:**
- [../DOCS_MAP.md](../DOCS_MAP.md)
- [../architecture/ui-layer.md](../architecture/ui-layer.md)

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
