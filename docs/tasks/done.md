# Done

> Completed tasks, newest first.

---

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
