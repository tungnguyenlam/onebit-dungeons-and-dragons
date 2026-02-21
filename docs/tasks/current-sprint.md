# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Last Session Handoff

```
Date:          2026-02-21
Completed:     Milestones 33-34 — Item pool expansion, legendary artifacts,
               multi-region quest refactor, and combat logic hardening.

Tasks completed:
  M33 — Item Pool Expansion Part 1
  - ✅ 20+ "One-Bit" items added and verified.
  - ✅ Fixed character mod application to damage dice (Strength mod now correctly adds to dmg).
  - ✅ Improved incapacitated log messages to show specific conditions.
  - ✅ Added elemental resistance verification test.

  M34 — Multi-Region Quest Chains
  - ✅ Refactored "Obsidian Scourge" for Ignis/Malphas flow.
  - ✅ Distributed Eye, Heart, Scepter across regions.
  - ✅ Asset validation passed for cross-region dependencies.

Tests at close: 154 passed, 0 failed

Next for incoming agent:
  - Start M35 (Character Progression Level 5-10)
```

---

## Active Task

### Task: M35 — Character Progression Level 5-10

**Files to touch:**
- `src/game/character/progression.rs`
- `src/game/combat/mod.rs`
- `assets/spells.toml`

**Done when:**
- [ ] Reaching level 5+ awards Extra Attack or 3rd level slots depending on class.
- [ ] Extra attack functions natively in the combat loop.
- [ ] At least 4 new high-tier spells are supported by `resolve_spell_effect()`.

**Blocked by:** none

**Relevant docs:**
- docs/tasks/milestones/m33.md
- docs/tasks/milestones/m34.md

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
