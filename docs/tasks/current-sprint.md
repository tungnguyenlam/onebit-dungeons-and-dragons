# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Milestone 15 planning complete
Task in progress: Milestone 15 kickoff (interactive playtest harness)

What was completed this session:
  Planning and roadmap:
    - defined detailed post-M14 roadmap milestones (`M15`..`M19`) in `docs/tasks/backlog.md`
    - added per-milestone scope boundaries, done-when criteria, verification commands, and risks/non-goals
    - moved sprint focus from generic triage to explicit `M15` execution

What is NOT done yet:
    - `M15` implementation work has not started yet (only planned/scoped)
    - `M16`..`M19` remain backlog milestones
    - warning cleanup remains intentionally deferred (non-blocking, backlog policy; out of milestone scope)
    - GUI parity work remains deferred unless explicitly pulled into sprint scope

Next action for the incoming agent:
  1. Implement `M15` scenario-aware runner presets in `scripts/agent_tui_smoke.sh`.
  2. Document deterministic capture/report workflow in testing docs.
  3. Run `cargo test` and one `ash_gate` capture flow for baseline evidence.
  4. Keep warning-only cleanup out of scope unless explicitly requested.

Files modified this session:
  docs/tasks/backlog.md
  docs/tasks/current-sprint.md (this file)

Blockers: none
```

---

## Active Task

### Task: Milestone 15 — Interactive Playtest Harness (Token-Efficient)

**Files to touch:**
- `scripts/agent_tui_smoke.sh` — scenario presets, deterministic capture controls, compact output defaults
- `docs/testing/tui-agent-smoke.md` — command reference and expected artifacts
- `docs/testing/interactive-playtest-checklist.md` (new) — manual interactive checklist and report template

**Done when:**
- [ ] `--scenario` presets support at least `ash_gate`, `ember_square`, and `river_watch`
- [ ] capture mode outputs token-efficient summaries with bounded frames/events
- [ ] one complete `ash_gate` escape interactive report is documented under `docs/testing/reports/`
- [ ] `cargo test` passes

**Blocked by:** none

**Relevant docs:**
- [../DOCS_MAP.md](../DOCS_MAP.md)
- [../AGENT.md](../AGENT.md)
- [../architecture/ui-layer.md](../architecture/ui-layer.md)
- [../testing/tui-agent-smoke.md](../testing/tui-agent-smoke.md)

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
