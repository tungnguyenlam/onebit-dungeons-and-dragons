# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    scaffolding phase — no Rust source code exists yet
Task in progress: (none — ready to begin Milestone 0)

What was completed this session:
  - All docs/ scaffolded (AGENT.md, architecture/, gameplay/,
    content/, decisions/, tasks/)
  - Cargo.toml created with all dependencies declared

What is NOT done yet:
  - src/ does not exist at all
  - No code has been written

Next action for the incoming agent:
  1. Run `cargo check` — it will fail (no src/main.rs). That is expected.
  2. Create src/main.rs with a minimal Ratatui hello-world event loop.
  3. Goal: `cargo build` passes with a working TUI window that closes on 'q'.
  4. Read these docs first:
       docs/architecture/overview.md
       docs/architecture/game-loop.md
       docs/architecture/ui-layer.md

Files modified this session:
  docs/** (all — initial creation), Cargo.toml

Blockers: none
```

---

## Active Task

### Task: Init Rust binary crate (Milestone 0, step 1)

**Files to create:**
- `src/main.rs` — Ratatui event loop skeleton
- `src/app.rs` — `AppState` enum, `App` struct
- `src/events.rs` — `Event` enum

**Done when:**
- [ ] `cargo build` passes with zero errors
- [ ] Running `cargo run` opens a TUI window
- [ ] Pressing `q` exits cleanly
- [ ] `AppState` enum exists with at least `WorldMap` and `MainMenu` variants

**Blocked by:** nothing

**Relevant docs:**
- [../architecture/overview.md](../architecture/overview.md)
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
