# Copilot Instructions

Terminal-based open-world D&D game in Rust (TUI via Ratatui, experimental GUI via egui), following the D&D 5e SRD ruleset.

> **Before touching any code, read `docs/AGENT.md`** — it describes the session handoff workflow, which task is active, and which docs are relevant to the current sprint. 
>
> **If you update any documentation, gameplay/architecture/content/task doc, or conventions, you MUST review `docs/DOCS_MAP.md` and update all linked files in the same change.**
>
> When you finish a task or must stop mid-task, update the `Last Session Handoff` block in `tasks/current-sprint.md` before ending: list exactly where you stopped, every file you modified, and the next concrete action for the incoming agent. Move completed tasks to `done.md` and pull the next from `backlog.md`.

---

## Build & Test Commands

```bash
cargo check                          # compile check (fast)
cargo run                            # TUI mode (default)
cargo run --features gui -- --mode gui  # GUI mode (experimental)
cargo test                           # full test suite
cargo test <test_name> -- --nocapture   # single test with output

# Verification scripts
scripts/agent_verify.sh              # cargo test (standard pre-commit check)
scripts/agent_verify.sh --with-smoke # also runs non-interactive TUI smoke flow
scripts/agent_tui_smoke.sh           # keyboard smoke test for TUI only
scripts/validate_content.sh          # validates authored TOML region/quest files
scripts/release_check.sh             # full release gate: tests + content + startup profile
```

`cargo` warnings are **non-blocking** — do not do broad warning-only cleanup unless the active task explicitly requests it.

---

## Architecture

### Module Boundary Rule (most important constraint)

```
src/ui/        ← rendering only; reads App state, never imports from src/game/
src/game/      ← pure game logic; no renderer imports whatsoever
src/data/      ← TOML deserialization + typed asset structs; no game logic
src/renderer.rs ← GameRenderer trait + GameEvent enum (renderer-agnostic)
src/app.rs     ← glue: owns AppState, wires GameEvent → game → renderer
src/main.rs    ← CLI --mode flag, selects TuiRenderer or GuiRenderer at launch
```

Violating the `ui` / `game` split is the single most important constraint to avoid — it keeps both layers independently testable.

### Game Loop

```rust
loop {
    renderer.render(&app)?;          // shared ref only — no mutations during render
    match renderer.poll_event()? {   // blocks ≤250 ms; timeout → GameEvent::Tick
        GameEvent::Quit => break,
        event => app.handle_event(event)?,
    }
}
renderer.teardown()
```

`GameEvent` is renderer-agnostic. Both TuiRenderer and GuiRenderer map their platform-specific inputs down to it. `App::handle_event` never sees raw key codes.

On `Tick`: advance combat animations → check WorldState emergent event triggers → age temporary conditions.

### AppState (ADR-001)

`AppState` is an enum in `src/app.rs`. The active variant determines which screen renders and which subsystem receives input. Screen-specific context is stored in the variant (e.g. `Combat(CombatContext)`, `Dialog(DialogContext)`). Transitions are explicit: `App::transition(next: AppState)`.

Adding a new screen requires: a new `AppState` variant + handling it in `App::render()` and `App::handle_event()`.

### Data Pipeline (ADR-002)

All hand-crafted content lives in `assets/` as TOML. No content is hardcoded in Rust.

```
assets/<subsystem>/<file>.toml
  → src/data/types.rs   (serde structs mirroring TOML schema)
  → src/data/loader.rs  (only place that calls fs::read_to_string + toml::from_str)
  → src/game/**         (consumes typed structs)
```

- Global assets (classes, races, monsters, spells, items, quests, lore) are loaded once at startup into `App`.
- Region assets are loaded per-region via `loader::load_region(slug)` which reads only `assets/regions/<slug>/`; the previous region's data is dropped.
- Schema changes require updating both the TOML files **and** the corresponding serde struct in `src/data/types.rs`.

---

## Key Conventions

### Region isolation
Each world region is self-contained under `assets/regions/<region-slug>/`:
```
region.toml     ← manifest (slug, rooms list, inter-region connections)
rooms/          ← one .toml per room
npcs/           ← NPC stat/metadata
dialog/         ← dialog trees
```
An agent working on one region reads only that region's files. Region slug must match the folder name.

### WorldState (story flags)
Story conditions are boolean flags and integer counters in `WorldState` (`src/game/story/world_state.rs`). The condition mini-language used in TOML quest/dialog files:
- `flag:key` / `not flag:key`
- `counter:key >= N` (also `>`, `<=`, `<`, `==`)
- `A && B` / `A || B`

Story code **never** mutates character stats directly — it fires events that the game loop routes to the appropriate subsystem.

### DiceExpr
`DiceExpr` (e.g. `"2d6+3"`) deserializes directly from TOML strings via a custom serde implementation. Use it for any damage/roll fields in asset files.

### TUI visual system
- Use semantic tokens from `src/ui/tui/theme.rs` (`theme()`, `panel_style()`, `emph_style()`, `muted_style()`, `accent_style()`). Never hardcode colors in screen modules.
- Use `theme::icon(key)` for icons — it returns tier-appropriate fallbacks (T0 = ASCII, T1–T3 = Unicode/glyphs).
- Terminal capability is detected once at startup into `TerminalTier` (T0–T3) via env vars (`NO_COLOR`, `COLORTERM`, `TERM`, `LANG`).

### Feature flags
- `tui` (default): Ratatui + Crossterm
- `gui`: egui + eframe (stub — not yet production-ready)

### ADRs
Settled architecture decisions are in `docs/decisions/`. Do not re-litigate them.

### Library docs lookup order
1. `rusty-man <crate>::Type` (reads local rustdoc JSON, best for terminal agents)
2. `cargo doc` → `target/doc/`
3. `https://docs.rs/<crate>/latest/<crate>/`
