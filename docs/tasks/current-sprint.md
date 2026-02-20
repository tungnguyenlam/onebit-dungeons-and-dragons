# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Last Session Handoff

```
Date:          2026-02-21
Completed:     Milestones 16-19 — Reliability, UX readability, consistency guards, and soak automation

Tasks completed:
  - ✅ `cargo run -- --validate-assets` and `scripts/validate_assets.sh` entrypoints added with room/dialog/quest graph checks
  - ✅ room traversal reliability checks cover blocked trigger tiles, missing travel targets, and unreachable rooms
  - ✅ combat screen now includes concise timeline strip + last-turn summary panel
  - ✅ reduced-motion-aware combat feedback styling hooks added
  - ✅ dialog runtime now emits explicit blocked-path feedback instead of silent no-op
  - ✅ long-session soak mode added to `scripts/agent_tui_smoke.sh` (`--soak --profile standard --minutes`)
  - ✅ CI workflow runs asset validation and short PR soak profile
  - ✅ milestone completion checklist template added for future handoffs

Next: Pull next roadmap milestone after M19
```

---

## Active Task

### Task: Post-M19 Follow-up Planning

**Files to touch:**
- `docs/tasks/backlog.md`
- `docs/tasks/current-sprint.md`

**Done when:**
- [ ] next milestone selected
- [ ] acceptance criteria copied from backlog template
- [ ] handoff block updated

**Blocked by:** none

**Relevant docs:**
- [../DOCS_MAP.md](../DOCS_MAP.md)
- [../AGENT.md](../AGENT.md)
- [milestone-checklist-template.md](milestone-checklist-template.md)

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
