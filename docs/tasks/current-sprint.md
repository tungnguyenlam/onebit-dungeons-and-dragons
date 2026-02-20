# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Milestone 2 complete — combat systems implemented end-to-end
Task in progress: Milestone 3 step 1 — quest stage machine + loader

What was completed this session:
  Milestone 2 step 6:
    - src/game/combat/combat.rs      — timed condition system:
        · `condition_durations` store
        · `apply_condition(condition, duration)`
        · `tick_condition_durations()` expiry processing
        · turn transition hook `advance_turn_with_condition_tick()`
    - src/app.rs                     — turn lifecycle wiring:
        · condition expiry processed when turns advance
        · expiry messages written to combat log
        · on-hit condition application now uses timed duration
    - src/ui/tui/screens/combat.rs   — HUD displays condition durations (e.g. `Poisoned(1)`)
    - tests added for:
        · condition tick expiry in combat state
        · app-level expiry on turn advance
    - `cargo test` — 71 tests, 0 failures
  Milestone 2 status:
    - all backlog checklist items now complete

What is NOT done yet:
    - src/ui/tui/screens/ — all screen render functions
    - src/ui/tui/layout.rs, widgets/
    - no distinct enemy behavior profiles beyond basic attack loop
    - game/ and data/ modules mostly not wired into app.rs yet
    - src/game/story/quest.rs, dialog.rs, journal.rs, events.rs  (Milestone 3)

Next action for the incoming agent:
  1. `cargo test` — must pass (71 tests) before touching anything.
  2. Start Milestone 3 step 1:
       - add `src/game/story/quest.rs` quest stage machine
       - support condition-based stage transitions using `WorldState::evaluate`
       - add loader glue for quest assets in `src/data/loader.rs`
  3. Add unit tests for quest progression and transition predicates.

Files modified this session:
  src/app.rs
  src/game/combat/attack.rs
  src/game/combat/combat.rs
  src/game/combat/mod.rs
  src/ui/tui/screens/combat.rs
  docs/tasks/backlog.md
  docs/tasks/current-sprint.md (this file)

Blockers: none
```

---

## Active Task

### Task: Quest Stage Machine (Milestone 3, step 1)

**Files to touch:**
- `src/game/story/quest.rs`       — quest runtime state machine
- `src/game/story/mod.rs`         — module export wiring
- `src/data/loader.rs`            — quest asset loading helpers
- `src/data/types.rs`             — reuse/extend quest structs if needed

**Done when:**
- [ ] `cargo test` passes
- [ ] quest can move between stages based on `WorldState::evaluate`
- [ ] quest completion/failure states are represented
- [ ] quest loader reads quest definitions from assets
- [ ] quest transitions have focused tests

**Blocked by:** `src/game/combat/` (done)

**Relevant docs:**
- [../gameplay/combat.md](../gameplay/combat.md)
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
