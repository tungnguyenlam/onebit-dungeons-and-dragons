# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Milestone 2 step 5 complete — condition hooks expanded
Task in progress: M2 combat step 6 — polish combat flow + final condition pass

What was completed this session:
  Milestone 2 step 5:
    - src/game/combat/attack.rs      — attack outcome now includes:
        · roll mode (`Normal`/`Advantage`/`Disadvantage`)
        · on-hit condition hook (`inflicted_condition`)
    - src/game/combat/combat.rs      — CombatantState now supports `on_hit_condition`
    - src/app.rs                     — combat log now surfaces:
        · advantage/disadvantage context on attack lines
        · specific incap condition names in skip messages
        · on-hit condition application messages
    - src/app.rs                     — seeded demo data: Goblin A inflicts `Poisoned` on hit
    - tests added:
        · roll-mode disadvantage from poisoned attacker
        · on-hit condition hook propagation
        · incap message includes specific condition
    - `cargo test` — 69 tests, 0 failures

What is NOT done yet:
    - src/ui/tui/screens/ — all screen render functions
    - src/ui/tui/layout.rs, widgets/
    - condition application breadth is still partial (no timed durations/resolution pipeline)
    - no distinct enemy behavior profiles (all enemies use same basic attack loop)
    - game/ and data/ modules mostly not wired into app.rs yet
    - src/game/story/quest.rs, dialog.rs, journal.rs, events.rs  (Milestone 3)

Next action for the incoming agent:
  1. `cargo test` — must pass (69 tests) before touching anything.
  2. Add turn lifecycle condition processing:
       - begin-turn and end-turn condition effect hook points
       - per-condition duration decrement support
  3. Implement minimal enemy behavior profiles:
       - basic melee profile and cautious/wait profile
  4. Continue UI wiring for non-combat screens (main menu/world map/dialog stubs).

Files modified this session:
  src/app.rs
  src/game/combat/combat.rs
  src/ui/tui/screens/combat.rs
  docs/tasks/current-sprint.md (this file)

Blockers: none
```

---

## Active Task

### Task: Combat Turn Lifecycle (Milestone 2, step 6)

**Files to touch:**
- `src/game/combat/combat.rs`     — turn lifecycle hooks + condition duration support
- `src/app.rs`                    — invoke lifecycle hooks at turn start/end
- `src/game/character/conditions.rs` — helper APIs for per-turn processing
- `src/ui/tui/screens/combat.rs`  — display condition duration where present

**Done when:**
- [ ] `cargo test` passes
- [ ] turn start/end hooks run for both player and enemies
- [ ] at least one condition duration decrements each round
- [ ] expired conditions are removed and logged
- [ ] lifecycle logic has focused unit tests

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
