# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Milestone 2 step 4 complete — enemy auto-turn loop added
Task in progress: M2 combat step 5 — broaden condition effects in combat

What was completed this session:
  Milestone 2 step 4:
    - src/app.rs                     — combat tick now processes enemy turns automatically
        · non-player combatants auto-attack valid player targets
        · enemy turns auto-advance until a player turn is reached
        · incapacitated enemies skip turn with combat-log entry
        · combat end now transitions to:
            - `WorldMap` on player-side victory
            - `GameOver` on player-side defeat
    - src/app.rs                     — refactored combat action flow via helper methods
      (`resolve_attack`, `run_enemy_turns`, `finish_combat_if_over`)
    - src/game/combat/combat.rs      — helper usage extended in app flow
    - src/ui/tui/screens/combat.rs   — initiative banner now shows active side
      (`PLAYER` vs `ENEMY`)
    - tests added:
        · enemy turn executes on tick and returns to player turn
        · tick transitions to world map / game over on combat end
    - `cargo test` — 67 tests, 0 failures

What is NOT done yet:
    - src/ui/tui/screens/ — all screen render functions
    - src/ui/tui/layout.rs, widgets/
    - condition application breadth is still partial (beyond incap/disadvantage hooks)
    - no distinct enemy behavior profiles (all enemies use same basic attack loop)
    - game/ and data/ modules mostly not wired into app.rs yet
    - src/game/story/quest.rs, dialog.rs, journal.rs, events.rs  (Milestone 3)

Next action for the incoming agent:
  1. `cargo test` — must pass (67 tests) before touching anything.
  2. Expand condition effects in combat execution:
       - prevent attack while `Prone`/`Poisoned`/`Restrained` where applicable
       - model on-hit condition infliction hooks in combat resolution
  3. Add richer combat messaging:
       - explicit skip-turn messages for each incap condition
       - attack summaries include condition-driven advantage/disadvantage reason
  4. Add targeted tests for new condition branches.

Files modified this session:
  src/app.rs
  src/game/combat/combat.rs
  src/ui/tui/screens/combat.rs
  docs/tasks/current-sprint.md (this file)

Blockers: none
```

---

## Active Task

### Task: Combat Conditions Expansion (Milestone 2, step 5)

**Files to touch:**
- `src/game/combat/attack.rs`     — condition-aware roll metadata and hooks
- `src/app.rs`                    — consume condition metadata in combat log
- `src/game/combat/combat.rs`     — optional helper(s) for turn-skip condition messaging
- `src/ui/tui/screens/combat.rs`  — expose condition effects in HUD/log context

**Done when:**
- [ ] `cargo test` passes
- [ ] Condition-driven advantage/disadvantage reason is visible in combat log
- [ ] Turn skip reason is specific to condition name
- [ ] At least one condition-application hook exists in attack resolution path
- [ ] New condition branches have dedicated tests

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
