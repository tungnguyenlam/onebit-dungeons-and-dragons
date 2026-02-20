# Agent TUI Smoke Tool

Use this tool to let an agent run a deterministic, keyboard-driven smoke test
of the TUI without manual interaction.

Script:
- `scripts/agent_tui_smoke.sh`
- `scripts/agent_verify.sh` (standard agent verification entry point)

---

## What It Tests

The scripted flow launches `cargo run -- --mode tui` and sends keys for:
- Main menu -> character creation -> world map
- Save (`p`) and load (`o`)
- Open/close inventory, spellbook, journal
- Enter/advance/exit combat
- Quit cleanly (`q`)

It then verifies `saves/slot1.toml` was created.

---

## Usage

From repo root:

```bash
# Standard regression entry point (tests only)
scripts/agent_verify.sh

# Tests + scripted TUI keyboard smoke
scripts/agent_verify.sh --with-smoke

# Smoke-only flow
scripts/agent_tui_smoke.sh

# Interactive manual inspection mode (same binary, no scripted keys)
scripts/agent_tui_smoke.sh --interactive
```

### Interactive Mode and TTY Requirements

`--interactive` launches the real TUI process and requires a terminal TTY.

- Works: local terminal sessions (normal shell), PTY-backed agent sessions.
- Fails by design: non-TTY command runners (piped/backgrounded headless exec).

If `--interactive` is run without a TTY, the script now exits with:

```text
Error: --interactive requires a TTY (stdin/stdout must be terminals).
Run this directly in a terminal, or omit --interactive for scripted smoke mode.
```

Use these modes intentionally:

- Manual interactive inspection: `scripts/agent_tui_smoke.sh --interactive`
- Deterministic automated smoke: `scripts/agent_tui_smoke.sh`

Options:

```bash
# Also run full test suite after smoke flow
scripts/agent_tui_smoke.sh --with-tests

# Keep generated save file for inspection
scripts/agent_tui_smoke.sh --keep-save

# Skip pre-build if already built
scripts/agent_tui_smoke.sh --no-build

# Override expect timeout (seconds)
scripts/agent_tui_smoke.sh --timeout 180

# Capture raw terminal output while running scripted flow
scripts/agent_tui_smoke.sh --capture-log /tmp/dnd-tui.raw.log
```

Environment:
- `TUI_TIMEOUT` (default: `120`)
- `TUI_RUSTFLAGS` (default: `-Awarnings`) to keep build output concise during smoke runs

---

## Requirements

- `expect` must be installed for scripted smoke mode.
  - macOS: `brew install expect`
- `expect` is not required for `--interactive`.

---

## Agent Workflow Recommendation

1. `scripts/agent_tui_smoke.sh`
2. If smoke passes and deeper validation is needed:
  - `scripts/agent_tui_smoke.sh --with-tests`

If smoke fails, inspect:
- `src/ui/tui/mod.rs` (key mapping)
- `src/app.rs` (event handling/state transitions)
- `src/ui/tui/screens/*.rs` (render/runtime assumptions)

## Scenario-Aware and Deterministic Capture (Milestone 15)

New options for interactive playtest and token-efficient capture:

```bash
# List available scenario presets
scripts/agent_tui_smoke.sh --list-scenarios

# Run a specific scenario (e.g., ash_gate)
scripts/agent_tui_smoke.sh --scenario ash_gate

# Run in interactive mode for a scenario
scripts/agent_tui_smoke.sh --interactive --scenario ember_square

# Capture a deterministic, token-efficient log (bounded frames)
scripts/agent_tui_smoke.sh --capture-log /tmp/ash_gate.log --scenario ash_gate --token-efficient --max-frames 120
```

- `--scenario <name>`: Use a scenario preset (ash_gate, ember_square, river_watch)
- `--token-efficient`: Output compact, summary-oriented logs for review
- `--max-frames <N>`: Limit the number of frames/events captured
- `--list-scenarios`: List all available scenario presets

See also: [interactive-playtest-checklist.md](interactive-playtest-checklist.md)
