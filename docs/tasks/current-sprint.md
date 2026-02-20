# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Milestone 13 complete
Task in progress: Post-release stabilization and roadmap refresh

What was completed this session:
  Milestone 12:
    - src/ui/tui/theme.rs + src/ui/tui/mod.rs:
      - added terminal capability-tier detection (`T0`..`T3`) and shared semantic theme tokens
    - src/ui/tui/screens/{main_menu,world_map,combat,inventory,spellbook}.rs:
      - improved readability and help overlays using semantic styles + icon fallback policy
    - docs/architecture/ui-layer.md + docs/architecture/tui-visual-system.md + README.md:
      - documented support matrix and visual fallback policy
  Milestone 13:
    - src/game/save/mod.rs + src/app.rs:
      - added save format versioning (`format_version`) and legacy compatibility fallback
    - src/data/loader.rs + scripts/profile_startup.sh:
      - added startup/data-load profiling smoke pass
    - scripts/release_check.sh + docs/releases/v0.1.0-internal.md:
      - added consolidated release verification + release notes handoff
  Milestone 11 carryover:
    - src/app.rs + assets/regions/valley-of-ash/dialog/captain_kael.toml:
      - completed faction-driven hostility/support and emergent event chain behavior
    - tests:
      - added event-chain and faction-driven behavior coverage in app tests
    - cargo test:
      - 115 tests, 0 failures
    - scripts/release_check.sh:
      - passes (tests + content validation + profile smoke)

What is NOT done yet:
    - warning cleanup remains intentionally deferred (non-blocking, backlog policy; out of milestone scope)
    - post-release bug triage/perf tracking pass has not started
    - GUI parity work remains deferred unless explicitly pulled into sprint scope

Next action for the incoming agent:
  1. Run post-release triage against internal feedback and convert top issues into backlog items.
  2. Track performance regressions over time using `scripts/profile_startup.sh`.
  3. Plan next content/system roadmap beyond M13 in `docs/tasks/backlog.md`.
  4. Keep warning-only cleanup out of scope unless explicitly requested.

Files modified this session:
  src/ui/tui/theme.rs (new)
  src/ui/tui/mod.rs
  src/ui/tui/screens/main_menu.rs
  src/ui/tui/screens/world_map.rs
  src/ui/tui/screens/combat.rs
  src/ui/tui/screens/inventory.rs
  src/ui/tui/screens/spellbook.rs
  src/game/save/mod.rs
  src/data/loader.rs
  scripts/profile_startup.sh (new)
  scripts/release_check.sh (new)
  docs/releases/v0.1.0-internal.md (new)
  docs/architecture/ui-layer.md
  docs/architecture/tui-visual-system.md
  README.md
  assets/regions/valley-of-ash/dialog/captain_kael.toml
  src/app.rs
  docs/tasks/backlog.md
  docs/tasks/done.md
  docs/tasks/current-sprint.md (this file)

Blockers: none
```

---

## Active Task

### Task: Post-release stabilization and roadmap refresh

**Files to touch:**
- `docs/tasks/backlog.md` — new post-M13 roadmap items
- `docs/tasks/current-sprint.md` + `docs/tasks/done.md` — triage handoff updates
- targeted `src/**` files for prioritized fixes found during internal triage

**Done when:**
- [ ] top post-release regressions are triaged and prioritized
- [ ] performance tracking cadence is documented and repeatable
- [ ] next roadmap slice beyond M13 is defined
- [ ] `cargo test` passes

**Blocked by:** none

**Relevant docs:**
- [../DOCS_MAP.md](../DOCS_MAP.md)
- [../AGENT.md](../AGENT.md)
- [../decisions/adr-003-save-format.md](../decisions/adr-003-save-format.md)

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
