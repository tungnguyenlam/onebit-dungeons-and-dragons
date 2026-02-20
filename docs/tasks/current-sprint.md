# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Post-M7 roadmap reset complete; Milestone 8 selected as next track
Task in progress: Milestone 8 — Stability & Engineering Debt kickoff

What was completed this session:
  Post-M7 planning and docs alignment:
    - docs/tasks/backlog.md:
      - added Roadmap Policy (Post-M7) with recommended defaults
      - added Milestones 8-13 with linked reference docs
    - docs/AGENT.md:
      - clarified that cargo warnings are currently non-blocking
      - documented the TUI smoke script as the default agent check
    - docs/tasks/current-sprint.md:
      - moved active focus to Milestone 8 stability track

What is NOT done yet:
    - Milestone 8 implementation work has not started yet
    - warning cleanup remains intentionally deferred (non-blocking until after M9)
    - GUI parity work remains deferred unless explicitly pulled into sprint scope

Next action for the incoming agent:
  1. Start Milestone 8 with integration tests for:
     - world-map -> trigger -> combat
     - world-map -> trigger -> dialog
     - save/load roundtrip from active runtime state
  2. Keep warning-only cleanup out of scope unless explicitly requested.
  3. Standardize developer/agent automation entry points under scripts/.
  4. Update done/backlog/current-sprint together when M8 checklist items move.

Files modified this session:
  docs/AGENT.md
  docs/tasks/backlog.md
  docs/tasks/current-sprint.md (this file)

Blockers: none
```

---

## Active Task

### Task: Milestone 8 — Stability & Engineering Debt

**Files to touch:**
- `src/app.rs` — stabilize runtime transitions and reduce glue paths
- `src/game/save/mod.rs` — strengthen roundtrip stability checks
- `src/ui/tui/mod.rs` — ensure deterministic event-driven transitions in tests
- `tests/` (or `src/**/tests`) — add end-to-end integration coverage
- `scripts/` — standardize agent/developer entry points for smoke/integration runs

**Done when:**
- [ ] integration tests cover world-map -> trigger -> combat
- [ ] integration tests cover world-map -> trigger -> dialog
- [ ] save/load roundtrip is covered from active gameplay state
- [ ] `cargo test` passes
- [ ] scripts provide clear automation entry points for agents

**Blocked by:** none

**Relevant docs:**
- [../DOCS_MAP.md](../DOCS_MAP.md)
- [../AGENT.md](../AGENT.md)
- [../testing/tui-agent-smoke.md](../testing/tui-agent-smoke.md)
- [../architecture/game-loop.md](../architecture/game-loop.md)
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
