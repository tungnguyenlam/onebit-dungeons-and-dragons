# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Milestone 3 complete — story/dialog systems implemented
Task in progress: Milestone 4 step 1 — inventory/equipment runtime integration

What was completed this session:
  Milestone 3:
    - src/game/story/quest.rs        — quest stage machine + quest acceptance + transition eval
    - src/game/story/dialog.rs       — dialog evaluator + choice resolution + skill-check sentinel
    - src/game/story/journal.rs      — append-only journal model with category filters
    - src/game/story/events.rs       — emergent event trigger engine + lore inspection hook
    - src/game/story/mod.rs          — story module exports enabled
    - src/data/loader.rs             — quest/lore loader helpers (`load_quests`, `load_lore`)
    - src/app.rs                     — story systems wired:
        · global `WorldState`, `Journal`, `QuestLog`
        · demo dialog flow and choice handling
        · quest acceptance/progression on ticks
        · environmental lore inspect action from world map
    - src/ui/tui/screens/dialog.rs   — dialog screen (NPC text + numbered choices)
    - src/ui/tui/screens/journal.rs  — journal screen (category + entry list/detail)
    - src/ui/tui/screens/mod.rs + src/ui/tui/mod.rs — render dispatch for dialog/journal screens
    - `cargo test` — 81 tests, 0 failures
  Milestone 3 status:
    - all backlog checklist items now complete

What is NOT done yet:
    - src/ui/tui/screens/ — all screen render functions
    - src/ui/tui/layout.rs, widgets/
    - Milestone 4 (items/spells) not implemented
    - no distinct enemy behavior profiles beyond basic attack loop
    - game/ and data/ modules still partially wired into app.rs

Next action for the incoming agent:
  1. `cargo test` — must pass (81 tests) before touching anything.
  2. Start Milestone 4 step 1:
       - wire runtime inventory/equipment into app flow (loot, equip, unequip)
       - apply armor/weapon stats to combat setup
  3. Add spell slot tracking interactions in app flow and spellbook state.
  4. Implement TUI spellbook screen with current slots and known spells.

Files modified this session:
  src/app.rs
  src/data/loader.rs
  src/game/story/mod.rs
  src/game/story/quest.rs (new)
  src/game/story/dialog.rs (new)
  src/game/story/journal.rs (new)
  src/game/story/events.rs (new)
  src/ui/tui/mod.rs
  src/ui/tui/screens/mod.rs
  src/ui/tui/screens/dialog.rs (new)
  src/ui/tui/screens/journal.rs (new)
  docs/tasks/backlog.md
  docs/tasks/current-sprint.md (this file)

Blockers: none
```

---

## Active Task

### Task: Inventory Runtime (Milestone 4, step 1)

**Files to touch:**
- `src/app.rs`                    — inventory/equipment interaction handling
- `src/game/items/inventory.rs`   — runtime helpers for stack/equip usage
- `src/game/items/equipment.rs`   — equip/unequip mutation helpers
- `src/ui/tui/screens/inventory.rs` — inventory list + equip actions

**Done when:**
- [ ] `cargo test` passes
- [ ] player can equip/unequip weapon and armor in app flow
- [ ] combat setup uses equipped gear stats
- [ ] inventory mutations are reflected in TUI inventory screen
- [ ] inventory/equipment operations have focused tests

**Blocked by:** `src/game/items/` (done)

**Relevant docs:**
- [../gameplay/items.md](../gameplay/items.md)
- [../gameplay/spells.md](../gameplay/spells.md)

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
