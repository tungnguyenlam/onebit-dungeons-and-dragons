# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Milestone 9 complete
Task in progress: Milestone 10 — Content Production Pipeline kickoff

What was completed this session:
  Milestone 9:
    - src/app.rs:
      - added runtime XP gain + deterministic level-up flow from combat victories
      - added class progression hooks (HP gain, slot refresh/expansion, ASI note logging)
      - switched item/spell defs to prefer loaded global assets at startup
      - added data-driven equipment bonus aggregation into combat/spell/runtime math
      - added combat action variety via numeric actions:
        - 1 attack
        - 2 healing potion (action)
        - 3 second wind (bonus action)
    - src/game/character/progression.rs:
      - added class hit die and spell-slot progression helpers
      - added cantrip scaling helper
    - src/game/combat/spells.rs:
      - added cantrip scaling by level and upcast scaling by slot level
    - src/data/types.rs:
      - added ItemBonuses schema for data-driven equipment effects
    - src/ui/tui/screens/combat.rs:
      - updated controls help for expanded combat actions
    - cargo test:
      - 108 tests, 0 failures

What is NOT done yet:
    - warning cleanup remains intentionally deferred (non-blocking, backlog policy)
    - Milestone 10 implementation has not started
    - GUI parity work remains deferred unless explicitly pulled into sprint scope

Next action for the incoming agent:
  1. Start Milestone 10 with region authoring templates + validation helpers.
  2. Add two additional authored regions beyond valley-of-ash.
  3. Ensure new regions/quests/dialog load with zero runtime code edits.
  4. Keep warning-only cleanup out of scope unless explicitly requested.

Files modified this session:
  src/app.rs
  src/game/character/progression.rs
  src/game/combat/spells.rs
  src/data/types.rs
  src/ui/tui/screens/combat.rs
  docs/tasks/backlog.md
  docs/tasks/done.md
  docs/tasks/current-sprint.md (this file)

Blockers: none
```

---

## Active Task

### Task: Milestone 10 — Content Production Pipeline

**Files to touch:**
- `assets/regions/*` — add at least two new region packs with manifests/rooms/npcs/dialog
- `assets/quests/*` + `assets/lore/*` — reusable authored content bound to new regions
- `docs/content/*` — document region templates/workflow and validation usage
- `scripts/*` (if needed) — content validation helpers for region/quest/dialog integrity

**Done when:**
- [ ] region authoring templates and validation helpers are in place
- [ ] two additional regions are authored and loadable
- [ ] quest/dialog content for new regions is authored via reusable workflow
- [ ] new content loads without runtime code edits
- [ ] `cargo test` passes

**Blocked by:** none

**Relevant docs:**
- [../DOCS_MAP.md](../DOCS_MAP.md)
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
