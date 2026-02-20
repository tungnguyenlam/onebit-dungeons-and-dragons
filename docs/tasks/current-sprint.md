# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Milestone 14 complete
Task in progress: Post-milestone maintenance (triage-driven)

What was completed this session:
  Milestone 14:
    - assets/regions/valley-of-ash/rooms/ash_gate.toml:
      - added travel trigger from `ash_gate` to `ember_square` to remove room-flow softlock
    - src/ui/tui/screens/world_map.rs:
      - expanded header/control layout to avoid clipping
      - changed NPC spawn glyph rendering from `@` to `n` to reduce player-marker confusion
      - added world-map glyph unit test
    - src/ui/tui/screens/combat.rs:
      - added lightweight combat feedback styling for miss/crit/heal/downed log lines
    - src/app.rs tests:
      - added `ash_gate` travel trigger presence + travel transition regression tests
    - scripts/agent_tui_smoke.sh + docs/testing/tui-agent-smoke.md:
      - added interactive/manual mode and token-efficient capture-oriented options
    - cargo test:
      - 118 tests, 0 failures

What is NOT done yet:
    - warning cleanup remains intentionally deferred (non-blocking, backlog policy; out of milestone scope)
    - no defined milestones remain in backlog
    - GUI parity work remains deferred unless explicitly pulled into sprint scope

Next action for the incoming agent:
  1. Triage incoming playtest feedback and create issue-driven tasks (non-milestone).
  2. Continue monitoring startup/load performance via `scripts/profile_startup.sh`.
  3. Define the next milestone set once new product goals are agreed.
  4. Keep warning-only cleanup out of scope unless explicitly requested.

Files modified this session:
  assets/regions/valley-of-ash/rooms/ash_gate.toml
  src/ui/tui/screens/world_map.rs
  src/ui/tui/screens/combat.rs
  src/app.rs
  scripts/agent_tui_smoke.sh
  docs/testing/tui-agent-smoke.md
  docs/tasks/backlog.md
  docs/tasks/done.md
  docs/tasks/current-sprint.md (this file)

Blockers: none
```

---

## Active Task

### Task: Post-milestone maintenance (triage-driven)

**Files to touch:**
- `docs/tasks/backlog.md` + `docs/tasks/current-sprint.md` + `docs/tasks/done.md` — feedback-driven planning updates
- targeted `src/**` and `assets/**` files based on prioritized playtest findings

**Done when:**
- [ ] triage priorities are documented and accepted
- [ ] at least one top-priority regression is fixed and verified
- [ ] `cargo test` passes

**Blocked by:** none

**Relevant docs:**
- [../DOCS_MAP.md](../DOCS_MAP.md)
- [../AGENT.md](../AGENT.md)
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
