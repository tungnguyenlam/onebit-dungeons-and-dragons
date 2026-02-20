# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Last Session Handoff

```
Date:          2026-02-20
Completed:     Milestone 15 — Interactive Playtest Harness

Tasks completed:
  - ✅ --scenario presets (ash_gate, ember_square, river_watch) in scripts/agent_tui_smoke.sh
  - ✅ Token-efficient capture mode (--token-efficient, --max-frames)
  - ✅ docs/testing/reports/ash_gate_escape.md created with full escape report
  - ✅ cargo test passes (118 tests)

Next: Ready for M16 or follow-up work
```

---

## Active Task

### Task: Milestone 15 — Interactive Playtest Harness (Token-Efficient)

**Files to touch:**
- `scripts/agent_tui_smoke.sh` — scenario presets, deterministic capture controls, compact output defaults
- `docs/testing/tui-agent-smoke.md` — command reference and expected artifacts
- `docs/testing/interactive-playtest-checklist.md` (new) — manual interactive checklist and report template

**Done when:**
- [x] `--scenario` presets support at least `ash_gate`, `ember_square`, and `river_watch`
- [x] capture mode outputs token-efficient summaries with bounded frames/events
- [x] one complete `ash_gate` escape interactive report is documented under `docs/testing/reports/`
- [x] `cargo test` passes

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
