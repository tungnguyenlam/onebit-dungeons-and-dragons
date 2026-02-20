# Done

> Completed tasks, newest first.

---

## 2026-02-20 — Milestone 2 (partial): Combat Actions Step 3

- [x] `src/app.rs` — `GameEvent::Attack` now performs target selection, attack roll, damage application, and action consumption
- [x] `src/app.rs` — combat log records hit/miss/crit results and HP updates
- [x] `src/game/combat/combat.rs` — added `can_take_actions` and `next_enemy_id` helpers
- [x] `src/ui/tui/screens/combat.rs` — HUD includes active conditions per combatant
- [x] tests: app-level combat attack + incapacitation coverage added
- [x] `cargo test` passes (64 tests, 0 failures)

## 2026-02-20 — Milestone 2 (partial): Combat UI Step 2

- [x] `src/ui/tui/screens/combat.rs` — combat screen with initiative banner, HUD, and log panel
- [x] `src/ui/tui/screens/mod.rs` — screen module export
- [x] `src/ui/tui/mod.rs` — renderer dispatch to combat screen for `AppState::Combat(_)`
- [x] `src/app.rs` — `CombatContext` now owns seeded `CombatState` + log for local testing
- [x] `src/app.rs` — world-map `Attack` enters combat; combat `Wait` advances turn and logs event
- [x] `cargo test` passes (60 tests, 0 failures)

## 2026-02-20 — Milestone 2 (partial): Combat Core

- [x] `src/game/combat/action.rs` — `ActionSlots` (action / bonus action / reaction / movement) + tests
- [x] `src/game/combat/initiative.rs` — initiative roll, deterministic seeded queue + tests
- [x] `src/game/combat/attack.rs` — attack resolution (hit/miss/crit), damage roll, saving throw, `apply_damage` + tests
- [x] `src/game/combat/combat.rs` — `CombatantState`, `CombatState`, `next_turn()`, round advancement + tests
- [x] `src/game/combat/mod.rs` — module root and re-exports
- [x] `src/game/mod.rs` — combat module enabled (`pub mod combat;`)
- [x] `cargo test` passes (60 tests, 0 failures)

## 2026-02-20 — Milestone 1 (partial): Core Game Systems

- [x] `src/game/dice/` — `DiceExpr` parser + roll / advantage / disadvantage + serde + unit tests
- [x] `src/data/types.rs` — typed serde structs for every TOML asset
  (RegionManifest, RoomDef, NpcDef, DialogTree, MonsterDef, ClassDef, RaceDef,
   ItemDef, SpellDef, QuestDef, LoreEntry)
- [x] `src/data/loader.rs` — `load<T>`, `load_region`, `load_global_assets`, `HasId`
- [x] `src/game/character/` — AbilityScores, Character, Skill/SkillSet, Condition, progression + tests
- [x] `src/game/items/` — Inventory, EquipmentSlots, armor AC calculation
- [x] `README.md` — project overview, full file tree, agent onboarding notice

## 2026-02-20 — Milestone 0: Crate Bootstrap & Renderer Abstraction

- [x] `Cargo.toml` — tui/gui feature flags, optional ratatui/crossterm/eframe/egui, clap
- [x] `src/renderer.rs` — `GameRenderer` trait, `GameEvent` enum, `ControlFlow`
- [x] `src/app.rs` — `App`, `AppState` (all variants), `handle_event`, per-screen dispatch stubs
- [x] `src/main.rs` — `--mode tui|gui` CLI, `run_loop`, cfg-gated renderer dispatch
- [x] `src/ui/tui/mod.rs` — `TuiRenderer: impl GameRenderer` with full crossterm key mapping
- [x] `src/ui/gui/mod.rs` — `GuiRenderer`/`GuiApp` stub (GUI deferred; TUI is current focus)
- [x] `cargo check` passes clean (0 errors, 0 warnings)
- [x] Docs: overview.md, game-loop.md, ui-layer.md updated; renderer.md added
