# ADR-003 — Save File Format

**Date:** 2026-02-20  
**Status:** Accepted

## Decision

Save files are serialised as TOML and stored in the OS config directory
(via the `dirs` crate: `~/.config/onebit-dnd/saves/<slot>.toml` on Linux/Mac,
`%APPDATA%\onebit-dnd\saves\` on Windows).

The save file captures **only mutable game state**, not static content:
- Player character (stats, HP, XP, inventory, conditions)
- `WorldState` (all flags and counters)
- Active quest stages
- Journal entries
- Current region slug + room id + player position
- Equipment slots

Static content (class features, monster stat blocks, etc.) is always re-loaded
from `assets/` on load — it is never duplicated in the save file.

## Rationale

- TOML save files are human-readable for debugging and modding.
- Separating mutable state from static content means asset updates don't
  corrupt saves.
- The `serde` + `toml` pipeline is already present for asset loading — no
  extra dependency needed.

## Rejected Alternatives

- **Binary (bincode/postcard)**: not human-readable; harder to debug corrupted
  saves; no benefit at the small save size expected.
- **JSON**: workable but TOML is already the project standard.
- **Save inside assets/**: would mix content with save data in version control.

## Consequences

- Save files are easy to inspect and manually edit (useful for testing).
- Must version the save schema; breaking changes need a migration script (or
  simply warn + offer to start a new game — acceptable for early development).
- The `src/game/save/serialization.rs` module owns all save/load logic.
