# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Milestone 8 complete
Task in progress: Milestone 9 — Core RPG Depth kickoff

What was completed this session:
  Milestone 8:
    - src/app.rs:
      - added integration tests:
        - world-map trigger -> dialog transition
        - world-map trigger -> combat transition
        - save/load roundtrip from active runtime world state
      - removed stale demo-dialog glue path
      - consolidated dialog transition path via shared helper
    - src/ui/tui/mod.rs:
      - switched to exhaustive AppState->screen match dispatch
    - scripts/agent_verify.sh:
      - added standardized agent verification entry point (tests, optional smoke)
    - docs/testing/tui-agent-smoke.md + docs/AGENT.md + README.md:
      - documented standardized verification commands
    - cargo test:
      - 101 tests, 0 failures

What is NOT done yet:
    - warning cleanup remains intentionally deferred (non-blocking until after M9)
    - Milestone 9 implementation has not started
    - GUI parity work remains deferred unless explicitly pulled into sprint scope

Next action for the incoming agent:
  1. Start Milestone 9 with level-up flow and progression hooks in runtime.
  2. Improve spell scaling/slot-depth behavior with focused tests.
  3. Wire data-driven equipment effects into combat/runtime calculations.
  4. Keep warning-only cleanup out of scope unless explicitly requested.

Files modified this session:
  src/app.rs
  src/ui/tui/mod.rs
  scripts/agent_verify.sh (new)
  docs/testing/tui-agent-smoke.md
  docs/AGENT.md
  README.md
  docs/tasks/backlog.md
  docs/tasks/done.md
  docs/tasks/current-sprint.md (this file)

Blockers: none
```

---

## Active Task

### Task: Milestone 9 — Core RPG Depth

**Files to touch:**
- `src/app.rs` — integrate level/progression flow into runtime
- `src/game/character/progression.rs` — progression hooks used by gameplay state
- `src/game/combat/*` — targeted combat-depth updates tied to RPG progression
- `src/game/items/*` + data wiring — data-driven equipment effects in runtime
- tests in `src/**/tests` — focused coverage for each new M9 behavior

**Done when:**
- [ ] level-up flow and class progression hooks are active in runtime
- [ ] spell scaling and slot-usage depth improvements are implemented
- [ ] data-driven equipment effects are applied in combat/runtime calculations
- [ ] combat action variety is expanded with high-signal improvements
- [ ] `cargo test` passes

**Blocked by:** none

**Relevant docs:**
- [../DOCS_MAP.md](../DOCS_MAP.md)
- [../gameplay/character.md](../gameplay/character.md)
- [../gameplay/combat.md](../gameplay/combat.md)
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
