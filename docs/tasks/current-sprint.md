# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Milestone 2 step 3 complete — combat attack flow wired
Task in progress: M2 combat step 4 — extend condition handling + AI turns

What was completed this session:
  Milestone 2 step 3:
    - src/app.rs                     — `GameEvent::Attack` now resolves combat:
        · target selection (next living enemy in initiative order)
        · roll attack via `game::combat::roll_attack`
        · apply damage via `game::combat::apply_damage`
        · consume action slot
        · write hit/miss/crit + HP results to combat log
    - src/app.rs                     — attack blocked for incapacitated actors
    - src/game/combat/combat.rs      — `can_take_actions`, `next_enemy_id` helpers + tests
    - src/ui/tui/screens/combat.rs   — HUD now displays condition labels
    - app tests added for attack-action flow and incapacitation checks
    - `cargo test` — 64 tests, 0 failures

What is NOT done yet:
    - src/ui/tui/screens/ — all screen render functions
    - src/ui/tui/layout.rs, widgets/
    - condition application breadth is partial (only currently-modeled effects)
    - no enemy AI behavior; turns are still manually advanced with wait
    - game/ and data/ modules mostly not wired into app.rs yet
    - src/game/story/quest.rs, dialog.rs, journal.rs, events.rs  (Milestone 3)

Next action for the incoming agent:
  1. `cargo test` — must pass (60 tests) before touching anything.
  2. Implement enemy auto-turn behavior:
       - on non-player turn, resolve one basic attack against a player target
       - auto-advance to next turn after enemy action
  3. Expand condition handling coverage in combat events:
       - incapacitating conditions force skip/auto-end turn
       - preserve poisoned disadvantage behavior in attack roll path
  4. Add combat end-state transition (back to world map or victory state).

Files modified this session:
  src/app.rs
  src/game/combat/combat.rs
  src/ui/tui/screens/combat.rs
  docs/tasks/current-sprint.md (this file)

Blockers: none
```

---

## Active Task

### Task: Combat AI Turn Flow (Milestone 2, step 4)

**Files to touch:**
- `src/app.rs`                    — enemy auto-action + turn progression
- `src/game/combat/combat.rs`     — turn helpers for side checks / target selection
- `src/ui/tui/screens/combat.rs`  — indicate active side and auto-turn events

**Done when:**
- [ ] `cargo test` passes
- [ ] Enemy turns execute basic attack automatically
- [ ] Incapacitated enemies skip action and pass turn
- [ ] Combat loop ends when one side is defeated
- [ ] Combat log clearly shows automated enemy actions

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
