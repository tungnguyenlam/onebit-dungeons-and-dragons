# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Last Session Handoff

```
Date:          2026-02-21
Completed:     Milestone 35 — Character Progression Level 5-10.

Tasks completed:
  M35 — Character Progression Level 5-10
  - ✅ Reaching level 5+ awards Extra Attack or 3rd level slots depending on class.
  - ✅ Extra attack functions natively in the combat loop.
  - ✅ At least 4 new high-tier spells are supported by `resolve_spell_effect()`.

Tests at close: 136 passed, 0 failed

Next for incoming agent:
  - Start M36 (The Sunken City)
```

---

## Active Task

### Task: M36 — The Sunken City

**Files to touch:**
- `assets/regions/sunken-city/`
- `src/game/world/map.rs`
- `src/data/types.rs`

**Done when:**
- [ ] Region `sunken-city` is visually distinct and fully traversable.
- [ ] Lore entries paint a coherent history of the city's fall.
- [ ] DeepWater restricts standard movement adequately.

**Blocked by:** none

**Relevant docs:**
- docs/tasks/milestones/m31.md
- docs/tasks/milestones/m36.md

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
