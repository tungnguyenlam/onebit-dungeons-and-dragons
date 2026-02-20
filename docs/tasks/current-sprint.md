# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Milestone 4 complete — items/spells runtime and TUI screens implemented
Task in progress: Milestone 5 step 1 — monster stat block loader integration

What was completed this session:
  Milestone 4:
    - src/game/items/inventory.rs     — runtime helpers for equipment state and consumable usage
    - src/game/items/equipment.rs     — equip/unequip mutation helpers with explicit slot enum
    - src/game/combat/spells.rs       — spell slot check/spend + spell effect resolution
    - src/game/combat/mod.rs          — spells module exports
    - src/app.rs                      — inventory + spellbook app flow, gear-based combat setup, potion and spell actions
    - src/ui/tui/screens/inventory.rs — inventory screen
    - src/ui/tui/screens/spellbook.rs — spellbook screen
    - src/ui/tui/screens/mod.rs + src/ui/tui/mod.rs — render dispatch for inventory/spellbook screens
    - `cargo test` — 88 tests, 0 failures
  Milestone 4 status:
    - all backlog checklist items now complete

What is NOT done yet:
    - Milestone 5 (NPC & factions) not implemented
    - no distinct enemy behavior profiles beyond basic attack loop
    - faction reputation is not connected to dialog/quest outcomes
    - world events are still mostly scripted stubs

Next action for the incoming agent:
  1. `cargo test` — must pass (88 tests) before touching anything.
  2. Start Milestone 5 step 1:
       - implement monster stat block loading into runtime encounter builders
       - replace hard-coded combatants with data-driven monster templates
  3. Add basic role-driven enemy behavior (melee/ranged/spellcaster) in combat ticks.
  4. Introduce faction reputation state and hook it into dialog condition checks.

Files modified this session:
  src/app.rs
  src/game/combat/mod.rs
  src/game/combat/spells.rs (new)
  src/game/items/equipment.rs
  src/game/items/inventory.rs
  src/ui/tui/mod.rs
  src/ui/tui/screens/mod.rs
  src/ui/tui/screens/inventory.rs (new)
  src/ui/tui/screens/spellbook.rs (new)
  docs/tasks/backlog.md
  docs/tasks/done.md
  docs/tasks/current-sprint.md (this file)

Blockers: none
```

---

## Active Task

### Task: Monster Runtime (Milestone 5, step 1)

**Files to touch:**
- `src/data/loader.rs`              — monster definition load helpers
- `src/app.rs`                      — build encounter combatants from monster defs
- `src/game/combat/combat.rs`       — optional helpers for data-driven combatant construction
- `src/ui/tui/screens/combat.rs`    — show monster role/type where helpful

**Done when:**
- [ ] `cargo test` passes
- [ ] encounters can be created from loaded monster definitions
- [ ] hard-coded enemy stat values are removed from app encounter setup
- [ ] combat flow remains stable with data-driven enemies
- [ ] loader + app integration has focused tests

**Blocked by:** `Milestone 4` (done)

**Relevant docs:**
- [../gameplay/npc-ai.md](../gameplay/npc-ai.md)
- [../architecture/data-pipeline.md](../architecture/data-pipeline.md)

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
