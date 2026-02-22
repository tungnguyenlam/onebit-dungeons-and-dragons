# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Active Task

**Milestone:** M52 - Flee Mechanics & Combat AI Targeting

**Goal:** Provide players with an escape mechanism from unwinnable encounters and ensure enemy AI logic uses pseudo-random target selection rather than always choosing the first player entity.

**Spec:** See [milestones/m52.md](milestones/m52.md)

---

## Last Session Handoff

```
Date:          2026-02-22
Completed:     M52 completed (Flee Mechanics & AI Targeting)

Tasks completed this session:
  1. Defined and executed Milestone 52 based on previous combat system refinements.
  2. Implemented `try_flee` logic (contested Dexterity/initiative_mod) inside `src/app/combat.rs`.
  3. Bound `f` and `4` to Flee Action across TUI event handlers.
  4. Formatted UI combat footer to display `4/f flee` option.
  5. Refactored `select_enemy_target` to use `ctx.seed` for pseudo-random targeting rather than locking onto the 0th index.
  6. Expanded `scripts/visual_check.py` capabilities by adding `combat_flee` test scenario to `tests/visual_scenarios.json`.

Files modified:
  - docs/tasks/milestones/m52.md (created)
  - docs/tasks/backlog.md
  - docs/tasks/done.md
  - docs/tasks/current-sprint.md
  - src/main.rs
  - src/ui/tui/mod.rs
  - src/ui/tui/screens/combat.rs
  - src/app/handlers.rs
  - src/app/combat.rs
  - tests/visual_scenarios.json

Build status: cargo test passes cleanly (136 tests). 

Next for incoming agent:
  - Proceed with any remaining bug fixes or start defining and executing M53 (possibly related to Advanced Storytelling or visual scenarios).
```

```
Date:          2026-02-21
Completed:     M51 completed

Tasks completed this session:
  1. Updated `run_text_mode` in `src/main.rs`:
     - Replaced custom print loop with Ratatui's `TestBackend`
     - Added auto-load and auto-save of state to `save.toml`
  2. Fixed a deadlock bug in `src/ui/tui/theme.rs`:
     - `TIER.get_or_init(init_terminal_tier)` called `init_terminal_tier` which then called `TIER.get_or_init` again, causing an infinite hang. 
     - Fixed `init_terminal_tier` so it directly returns `TerminalTier` without going through the OnceLock.
  3. Updated `docs/testing/step-through-testing.md` to document new test output format and precise state persistence file (`save.toml`).
  4. Updated `scripts/runtest.sh` string documenting where state is persisted.

Files modified:
  - src/main.rs
  - src/ui/tui/theme.rs
  - scripts/runtest.sh
  - docs/testing/step-through-testing.md

Build status: cargo check passes (14 minor warnings)
Runtest script functions correctly without hanging and outputs full TUI buffers.

All requirements complete:
  ✅ State persists via save/load in run_text_mode
  ✅ `runtest.sh j` and similar commands move the player with saved context between invovcations
  ✅ Documentation updated with accurate test runner info

Next for incoming agent:
  - Proceed with the next milestone from the backlog
```
```
Date:          2026-02-21
Completed:     App Module Refactoring

Tasks completed this session:
  1. Refactored `src/app/mod.rs` into multiple submodules (navigation, equipment, progression, systems, actions, debug) to improve maintainability and adhere to file size limits.
  2. Moved `find_spawn_pos_for_room` to `src/app/samples.rs`.
  3. Cleaned up `src/app/mod.rs` to only contain core logic and struct definitions.
  4. Verified all changes with `cargo check` and `scripts/agent_verify.sh` (135 tests passed).

Files modified/created:
  - src/app/mod.rs (modified)
  - src/app/navigation.rs (new)
  - src/app/equipment.rs (new)
  - src/app/progression.rs (new)
  - src/app/systems.rs (new)
  - src/app/actions.rs (new)
  - src/app/debug.rs (new)
  - src/app/samples.rs (modified)

Build status: Finished dev profile [unoptimized + debuginfo] target(s) in 0.77s.
Warnings: 13 minor styling/unused variable warnings (unrelated to refactor).

Next for incoming agent:
  - Proceed with any new feature requests or further polish.
  - No pending milestones in backlog.md.
```

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
