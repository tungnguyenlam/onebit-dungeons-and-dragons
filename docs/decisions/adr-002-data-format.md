# ADR-002 — TOML as the Content Data Format

**Date:** 2026-02-20  
**Status:** Accepted

## Decision

All hand-crafted game content (maps, classes, races, monsters, spells, items,
quests, dialog, lore) is stored as TOML files in `assets/`.

## Rationale

- TOML is human-readable and human-writable without a special editor — a
  content agent or designer can create a dungeon room with a text editor.
- The `toml` crate integrates cleanly with `serde` Rust structs, giving free
  type-safe deserialization.
- TOML multiline strings make dialog and lore `body` fields natural to write.
- TOML tables map directly to the nested data shapes needed (e.g. `[weapon]`
  sub-table, `[[triggers]]` arrays of tables).

## Rejected Alternatives

- **JSON**: verbose, no comments, multiline strings are awkward. Ruled out for
  authoring comfort.
- **YAML**: ambiguous parsing edge cases (the "Norway problem"), complex spec.
  Ruled out for reliability.
- **RON** (Rusty Object Notation): unfamiliar to non-Rust content authors.
- **Custom binary format**: no human editability. Only appropriate if
  performance becomes a bottleneck (it won't at this content scale).
- **SQLite**: overkill for read-only content; poor diff/merge story in version
  control.

## Consequences

- All content changes are plain-text and version-control friendly (good diffs).
- Schema changes require updating both the TOML files and the `serde` struct in
  `src/data/types.rs` simultaneously.
- Performance: TOML parsing at startup is acceptable. No streaming/lazy loading
  is needed at this content scale (< 2 MB total).
