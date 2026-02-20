# Agent TUI Smoke Tool

Use this tool to let an agent run a deterministic, keyboard-driven smoke test
of the TUI without manual interaction.

Script:
- `scripts/agent_tui_smoke.sh`

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
scripts/agent_tui_smoke.sh
```

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
```

Environment:
- `TUI_TIMEOUT` (default: `120`)

---

## Requirements

- `expect` must be installed.
  - macOS: `brew install expect`

---

## Agent Workflow Recommendation

1. `scripts/agent_tui_smoke.sh`
2. If smoke passes and deeper validation is needed:
  - `scripts/agent_tui_smoke.sh --with-tests`

If smoke fails, inspect:
- `src/ui/tui/mod.rs` (key mapping)
- `src/app.rs` (event handling/state transitions)
- `src/ui/tui/screens/*.rs` (render/runtime assumptions)
