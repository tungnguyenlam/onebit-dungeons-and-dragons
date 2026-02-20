# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Milestone 5 complete — NPC/faction runtime integrated
Task in progress: Milestone 6 step 1 — first playable region content authoring

What was completed this session:
  Milestone 5:
    - src/data/loader.rs               — `load_monsters` helper + loader tests
    - src/game/combat/combat.rs        — combatant AI-role/loadout metadata (`EnemyAiRole`, ranged/spell profiles)
    - src/game/combat/mod.rs           — exports updated for AI role
    - src/game/story/world_state.rs    — faction reputation helper APIs
    - src/app.rs                       — monster-template encounter building, role-driven enemy turns, emergent events from `WorldState`
    - src/ui/tui/screens/combat.rs     — enemy role shown in combat HUD
    - `cargo test` — 94 tests, 0 failures
  Milestone 5 status:
    - all backlog checklist items now complete

What is NOT done yet:
    - Milestone 6 (first region content) not started
    - real asset-driven content set under `assets/` is still minimal/missing
    - combat/map integration still uses demo encounter flow rather than region triggers

Next action for the incoming agent:
  1. `cargo test` — must pass (94 tests) before touching anything.
  2. Start Milestone 6 step 1:
       - author `assets/regions/valley-of-ash/region.toml` + rooms + starter npcs/dialog
       - create first encounter/quest content with existing loaders
  3. Wire region entry/loading in app flow from `assets/regions/`.
  4. Add smoke tests for loading authored region files.

Files modified this session:
  src/app.rs
  src/data/loader.rs
  src/game/combat/mod.rs
  src/game/combat/combat.rs
  src/game/story/world_state.rs
  src/ui/tui/screens/combat.rs
  docs/tasks/backlog.md
  docs/tasks/done.md
  docs/tasks/current-sprint.md (this file)

Blockers: none
```

---

## Active Task

### Task: Valley Of Ash Authoring (Milestone 6, step 1)

**Files to touch:**
- `assets/regions/valley-of-ash/region.toml`   — region manifest
- `assets/regions/valley-of-ash/rooms/*.toml`  — room layouts, triggers, items
- `assets/regions/valley-of-ash/npcs/*.toml`   — starter NPC definitions
- `assets/regions/valley-of-ash/dialog/*.toml` — starter dialog trees

**Done when:**
- [ ] `cargo test` passes
- [ ] region manifest + rooms load through `load_region`
- [ ] at least one dialog trigger and one encounter trigger are authored
- [ ] starter quest hooks are represented in content files
- [ ] authored content has loader-focused tests

**Blocked by:** `Milestone 5` (done)

**Relevant docs:**
- [../content/regions/index.md](../content/regions/index.md)
- [../content/map-format.md](../content/map-format.md)
- [../content/quests.md](../content/quests.md)

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
