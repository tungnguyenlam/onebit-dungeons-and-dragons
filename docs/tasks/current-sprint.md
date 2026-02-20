# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Milestone 11 complete
Task in progress: Milestone 12 — UX & Presentation Polish 2.0 kickoff

What was completed this session:
  Milestone 11:
    - src/app.rs:
      - added spellcaster support AI behavior archetype (ally-heal decision path)
      - faction reputation now affects hostility/support:
        - positive goblin rep can avert goblin encounters
        - trusted/requested guard support can join combat as ally
      - added emergent event chain triggers (`town_guard_trusted`, ember briefing, `valley_warfront`)
    - assets/regions/valley-of-ash/dialog/captain_kael.toml:
      - added reputation-gated support request branch
    - tests:
      - added event-chain and faction-driven behavior coverage in app tests
    - cargo test:
      - 113 tests, 0 failures

What is NOT done yet:
    - warning cleanup remains intentionally deferred (non-blocking, backlog policy)
    - Milestone 12 implementation has not started
    - GUI parity work remains deferred unless explicitly pulled into sprint scope

Next action for the incoming agent:
  1. Start Milestone 12 with HUD/readability improvements on core screens.
  2. Add terminal capability tiers and fallback policy (`T0`..`T3`) in the TUI layer.
  3. Introduce shared semantic theme tokens and portable icon fallback glyphs.
  4. Keep warning-only cleanup out of scope unless explicitly requested.

Files modified this session:
  assets/regions/valley-of-ash/dialog/captain_kael.toml
  src/app.rs
  docs/tasks/backlog.md
  docs/tasks/done.md
  docs/tasks/current-sprint.md (this file)

Blockers: none
```

---

## Active Task

### Task: Milestone 12 — UX & Presentation Polish 2.0

**Files to touch:**
- `src/ui/tui/*` + `src/renderer.rs` — terminal capability tiers and fallback policy
- `src/ui/tui/screens/*` — HUD/readability improvements and help overlays
- shared style token location (`src/ui/tui/...`) — semantic color roles/icons with fallback glyphs
- docs in `docs/architecture/` + `README.md` — support matrix and presentation config

**Done when:**
- [ ] HUD/readability improves across key screens
- [ ] terminal capability tiers + fallback policy are implemented
- [ ] semantic theme tokens + icon fallback glyphs are in use
- [ ] support matrix/config docs are updated
- [ ] `cargo test` passes

**Blocked by:** none

**Relevant docs:**
- [../DOCS_MAP.md](../DOCS_MAP.md)
- [../architecture/ui-layer.md](../architecture/ui-layer.md)
- [../architecture/tui-visual-system.md](../architecture/tui-visual-system.md)

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
