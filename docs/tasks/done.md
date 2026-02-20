# Done

> Completed tasks, newest first.
> Keep milestone/docs status synced using [../DOCS_MAP.md](../DOCS_MAP.md).

---

## 2026-02-21 — Milestones 16-19 (complete): Reliability, Readability, Consistency, Soak

- [x] `src/data/validate.rs` (new) + `src/main.rs` + `scripts/validate_assets.sh` (new) — added `--validate-assets` and validator coverage for:
  - region room outbound travel presence
  - blocked/out-of-bounds trigger positions
  - missing travel/dialog targets
  - room reachability from region entry
  - quest stage graph consistency and reachability
  - dialog node/link integrity checks
- [x] `src/app.rs` — dialog softlock guardrails now emit explicit journal feedback for blocked/broken paths
- [x] `src/ui/tui/screens/combat.rs` + `src/ui/tui/theme.rs` — combat readability upgrades:
  - concise initiative timeline strip
  - last-turn summary panel
  - reduced-motion-aware style hook for semantic feedback parity
  - added focused rendering helper tests
- [x] `scripts/agent_tui_smoke.sh` — added long-session soak mode (`--soak --profile standard --minutes`)
- [x] `.github/workflows/rust.yml` — CI now runs asset validation and short PR soak profile
- [x] `docs/tasks/milestone-checklist-template.md` (new) — milestone completion checklist template for handoffs
- [x] Verification run:
  - `cargo test` (targeted + full)
  - `cargo run -- --validate-assets`
  - `scripts/agent_tui_smoke.sh --no-build`
  - `scripts/agent_tui_smoke.sh --soak --profile standard --minutes 1 --token-efficient --no-build`

## 2026-02-20 — Milestone 14 (complete): Playtest UX & Ash Gate Flow Fixes

- [x] `assets/regions/valley-of-ash/rooms/ash_gate.toml` — added deterministic travel trigger from `ash_gate` to `ember_square`
- [x] `src/ui/tui/screens/world_map.rs` — fixed map readability issues:
  - increased header space to avoid clipping player status line
  - rendered NPC spawn tiles as `n` (not `@`) to avoid double-player glyph confusion
  - added unit test for spawn glyph behavior
- [x] `src/ui/tui/screens/combat.rs` — added lightweight visual feedback cues in combat log (hit/miss/crit/heal/downed styling)
- [x] `src/app.rs` tests — added `ash_gate` travel regression tests (`trigger exists` + `travel moves to ember_square`)
- [x] `scripts/agent_tui_smoke.sh` + `docs/testing/tui-agent-smoke.md` — added token-efficient interactive testing mode and capture flags for manual playtesting
- [x] `cargo test` passes (118 tests, 0 failures)

## 2026-02-20 — Milestone 13 (complete): Release Readiness

- [x] `src/game/save/mod.rs` + `src/app.rs` — save format versioning hardening (`format_version`) with legacy compatibility fallback
- [x] `src/game/save/mod.rs` tests — added legacy save loading coverage without version field
- [x] `src/data/loader.rs` + `scripts/profile_startup.sh` — startup/load profiling smoke pass with bounded threshold check
- [x] `scripts/release_check.sh` (new) — consolidated release verification entry point (tests + content validation + profile smoke)
- [x] `docs/releases/v0.1.0-internal.md` (new) — release notes and handoff summary
- [x] `README.md` — added release-check command, support matrix, and release-notes reference
- [x] `scripts/release_check.sh` passes

## 2026-02-20 — Milestone 12 (complete): UX & Presentation Polish 2.0

- [x] `src/ui/tui/theme.rs` (new) — terminal capability-tier detection (`T0`..`T3`) and semantic theme tokens
- [x] `src/ui/tui/mod.rs` — initializes and caches terminal capability profile at renderer startup
- [x] `src/ui/tui/screens/{main_menu,world_map,combat,inventory,spellbook}.rs` — migrated key HUD/panels to shared semantic tokens and icon fallbacks
- [x] `docs/architecture/ui-layer.md` + `docs/architecture/tui-visual-system.md` — documented theme/tier implementation and verification helpers
- [x] `README.md` — documented terminal support matrix and config behavior
- [x] `cargo test` passes

## 2026-02-20 — Milestone 11 (complete): NPC/Faction Simulation 2.0

- [x] `src/app.rs` — added spellcaster support behavior archetype (enemy AI can prioritize healing wounded allies)
- [x] `src/app.rs` — faction reputation now materially affects hostility/support:
  - goblin encounters can be averted at positive goblin reputation
  - guard ally support can be requested/consumed and auto-joined via trusted reputation
- [x] `assets/regions/valley-of-ash/dialog/captain_kael.toml` — added reputation-gated support dialog branch
- [x] `src/app.rs` — added multi-step emergent event chain (`town_guard_trusted` -> ember briefing -> `valley_warfront`)
- [x] tests: added focused coverage for event chains, encounter aversion, and ally-support spawning
- [x] `cargo test` passes (113 tests, 0 failures)

## 2026-02-20 — Milestone 10 (complete): Content Production Pipeline

- [x] `assets/regions/emberpeak-summit/*` (new) — authored second region with manifest, rooms, NPCs, and dialog
- [x] `assets/regions/ironhold-mines/*` (new) — authored third region with manifest, rooms, NPCs, and dialog
- [x] `assets/quests/main/volcanic-curse.toml` (new) — authored mainline Act 2 quest scaffolding
- [x] `assets/quests/side/dwarven-relic.toml` + `assets/quests/side/gnome-debt.toml` (new) — reusable side-quest content pipeline examples
- [x] `assets/lore/ember_rune.toml` + `assets/lore/mine_ledger.toml` (new) — region-linked lore assets for trigger-driven progression
- [x] `docs/content/regions/templates/*` (new) — region/room/NPC/dialog templates for repeatable authoring
- [x] `scripts/validate_content.sh` (new) + `src/data/loader.rs` tests — validation helpers for authored regions/quests loader compatibility
- [x] `src/app.rs` — lore-trigger flagging generalized to `read_<lore_id>` to support newly authored content without bespoke runtime edits
- [x] `cargo test` passes (110 tests, 0 failures); `scripts/validate_content.sh` passes

## 2026-02-20 — Milestone 9 (complete): Core RPG Depth

- [x] `src/app.rs` — runtime XP gain + level-up flow wired from combat victory, with HP/slot progression hooks and journal entries
- [x] `src/game/character/progression.rs` — added class hit-die helpers, cantrip scaling tiers, and class spell-slot progression table
- [x] `src/game/combat/spells.rs` — added cast-level/caster-level scaling for cantrips and upcast spells
- [x] `src/data/types.rs` — added data-driven `ItemBonuses` schema on `ItemDef`
- [x] `src/app.rs` — equipment bonus aggregation now feeds combat AC/attack/damage/max-HP and spell damage resolution
- [x] `src/app.rs` + `src/ui/tui/screens/combat.rs` — expanded combat action variety with numbered combat actions (attack, potion action, second wind bonus action)
- [x] tests: added focused coverage for level-up, slot-depth casting, equipment bonus application, and new combat actions
- [x] `cargo test` passes (108 tests, 0 failures)

## 2026-02-20 — Milestone 8 (complete): Stability & Engineering Debt

- [x] `src/app.rs` — added end-to-end transition tests for world-map trigger -> dialog and world-map trigger -> combat
- [x] `src/app.rs` — strengthened save/load test coverage with active runtime world-state roundtrip assertions
- [x] `src/app.rs` — removed stale demo dialog path and consolidated dialog transition glue (`start_dialog_with_npc`)
- [x] `src/ui/tui/mod.rs` — tightened renderer state dispatch to exhaustive screen match
- [x] `scripts/agent_verify.sh` (new) — standardized agent verification entry point (`cargo test`, optional smoke)
- [x] `docs/testing/tui-agent-smoke.md` + `docs/AGENT.md` + `README.md` — documented standardized script entry points
- [x] `cargo test` passes (101 tests, 0 failures)

## 2026-02-20 — Milestone 7 (complete): Polish

- [x] `src/game/save/mod.rs` — TOML save/load serialization runtime (`SaveGame`) + tests
- [x] `src/renderer.rs` + `src/ui/tui/mod.rs` + `src/ui/gui/mod.rs` — save/load/sound toggle events and key bindings
- [x] `src/ui/tui/screens/main_menu.rs` + `src/ui/tui/screens/character_creation.rs` + `src/ui/tui/screens/game_over.rs` — menu and character creation screens
- [x] `src/ui/tui/mod.rs` + `src/ui/tui/screens/mod.rs` — dispatch wired for menu/world/creation/game-over flows
- [x] `README.md` — status + screenshots + updated controls

## 2026-02-20 — Milestone 6 (complete): First Region

- [x] `assets/regions/valley-of-ash/` — region manifest, 2 rooms, triggers, NPCs, and dialog trees
- [x] `assets/quests/main/valley-contract.toml` — Act 1 main quest with 3 stages
- [x] `assets/quests/side/embers-forge.toml` + `assets/quests/side/goblin-banners.toml` — two side quests
- [x] `assets/lore/ash_tablet.toml` — environmental lore entry wired to room trigger
- [x] `src/app.rs` — runtime region loading, map movement, trigger interactions, and travel handling
- [x] `src/data/loader.rs` — authored region loader smoke test
- [x] `cargo test` passes (98 tests, 0 failures)

## 2026-02-20 — Milestone 5 (complete): NPC & Factions

- [x] `src/data/loader.rs` — monster loader helper (`load_monsters`) with focused tests
- [x] `src/app.rs` — data-driven encounter builder from `MonsterDef` templates
- [x] `src/app.rs` — role-based enemy behavior loop (`melee`, `ranged`, `spellcaster`)
- [x] `src/game/combat/combat.rs` — combatant AI role/loadout fields for runtime decisioning
- [x] `src/game/story/world_state.rs` — faction reputation helpers (`faction_rep`, `delta_faction_rep`, etc.)
- [x] `src/app.rs` — emergent world event engine wired to `WorldState` thresholds
- [x] `src/ui/tui/screens/combat.rs` — combat HUD now shows enemy role
- [x] `cargo test` passes (94 tests, 0 failures)

## 2026-02-20 — Milestone 4 (complete): Items & Spells

- [x] `src/game/items/inventory.rs` — runtime inventory helpers (`set_equipped`, `is_equipped`, `use_one`) + focused tests
- [x] `src/game/items/equipment.rs` — explicit `EquipmentSlot` plus equip/unequip mutation helpers + tests
- [x] `src/game/combat/spells.rs` — spell slot checks/spending and spell effect resolution (`Damage`, `Heal`, `Condition`) + tests
- [x] `src/game/combat/mod.rs` — spell runtime exports wired
- [x] `src/app.rs` — inventory/spellbook app flow, gear-to-combat stat application, potion usage, spell casting, combat sync-back
- [x] `src/ui/tui/screens/inventory.rs` — inventory UI screen implemented
- [x] `src/ui/tui/screens/spellbook.rs` — spellbook UI screen implemented
- [x] `src/ui/tui/mod.rs` + `src/ui/tui/screens/mod.rs` — render dispatch wired for inventory/spellbook
- [x] `cargo test` passes (88 tests, 0 failures)

## 2026-02-20 — Milestone 3 (complete): Story & Dialog

- [x] `src/game/story/quest.rs` — quest stage machine with acceptance, stage transitions, and completion state
- [x] `src/data/loader.rs` — quest/lore loading helpers (`load_quests`, `load_lore`)
- [x] `src/game/story/dialog.rs` — dialog evaluator, condition-filtered choices, and skill-check branch resolution
- [x] `src/game/story/journal.rs` — append-only journal entries with category filtering
- [x] `src/game/story/events.rs` — emergent event trigger evaluation + lore inspection hook
- [x] `src/game/story/mod.rs` — story modules exported and enabled
- [x] `src/app.rs` — quest ticking, dialog advancement, journal integration, and world-map lore inspect action wired
- [x] `src/ui/tui/screens/dialog.rs` — dialog UI screen implemented
- [x] `src/ui/tui/screens/journal.rs` — journal UI screen implemented
- [x] `src/ui/tui/mod.rs` + `src/ui/tui/screens/mod.rs` — render dispatch wired for dialog/journal screens
- [x] `cargo test` passes (81 tests, 0 failures)

## 2026-02-20 — Milestone 2 (complete): Combat Turn Lifecycle Step 6

- [x] `src/game/combat/combat.rs` — timed condition storage (`condition_durations`) and expiry tick support
- [x] `src/game/combat/combat.rs` — `advance_turn_with_condition_tick()` integrates condition lifecycle with turn transitions
- [x] `src/app.rs` — turn advancement now logs condition expirations
- [x] `src/app.rs` — on-hit condition application now uses timed duration helper
- [x] `src/ui/tui/screens/combat.rs` — condition duration rendered in combat HUD
- [x] tests: condition tick/expiry behavior in combat state + app flow
- [x] `cargo test` passes (71 tests, 0 failures)

## 2026-02-20 — Milestone 2 (partial): Combat Conditions Step 5

- [x] `src/game/combat/attack.rs` — `AttackOutcome` now reports roll mode and on-hit condition hook
- [x] `src/game/combat/combat.rs` — `CombatantState` now supports `on_hit_condition`
- [x] `src/app.rs` — combat log now includes advantage/disadvantage and specific incapacitation condition names
- [x] `src/app.rs` — on-hit condition application now updates target conditions and logs it
- [x] tests: roll-mode and on-hit condition behavior coverage added
- [x] `cargo test` passes (69 tests, 0 failures)

## 2026-02-20 — Milestone 2 (partial): Combat AI Step 4

- [x] `src/app.rs` — tick-driven enemy auto-turn loop added
- [x] `src/app.rs` — enemy turns now auto-attack and auto-advance to next turn
- [x] `src/app.rs` — incapacitated enemies skip with combat-log output
- [x] `src/app.rs` — combat end now transitions to `WorldMap` (victory) or `GameOver` (defeat)
- [x] `src/ui/tui/screens/combat.rs` — active combatant side shown in initiative banner
- [x] tests: enemy tick action + combat-end transition coverage added
- [x] `cargo test` passes (67 tests, 0 failures)

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
