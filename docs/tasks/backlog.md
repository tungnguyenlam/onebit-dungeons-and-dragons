# Backlog

> Ordered roughly by dependency. Pick from the top.
> Move items to [current-sprint.md](current-sprint.md) when starting, then to [done.md](done.md) on completion.
> For cross-doc update dependencies, see [../DOCS_MAP.md](../DOCS_MAP.md).

---

## Milestone 0 — Crate Bootstrap

- [x] Init Rust binary crate with feature-flagged dual renderer (`tui` / `gui`)
- [x] Scaffold renderer abstraction (`src/renderer.rs`, `src/app.rs`, `src/main.rs`, `src/ui/`)
- [x] `cargo check` passes (TUI default)
- [x] TUI renderer draws a placeholder frame; `q` exits cleanly
- [ ] Implement full `AppState` screen-switch in TUI screens (deferred to M1 TUI milestone)

## Milestone 1 — Core Systems (no content)

- [x] `src/game/dice/` — `DiceExpr` parser + `roll()` function with unit tests
  - See [gameplay/dice.md](../gameplay/dice.md)
- [x] `src/data/` — TOML asset loader with typed `serde` structs
  - See [architecture/data-pipeline.md](../architecture/data-pipeline.md)
- [x] `src/game/character/` — ability scores, modifiers, HP, conditions
  - See [gameplay/character.md](../gameplay/character.md)
- [x] `src/game/items/` — inventory, equipment slots, armor AC
  - See [gameplay/items.md](../gameplay/items.md)
- [x] `src/game/world/` — region loader, tile map, room graph, FOV
  - See [gameplay/world.md](../gameplay/world.md)
- [x] `src/game/story/world_state.rs` — flag store, save/load
  - See [gameplay/story.md](../gameplay/story.md)

## Milestone 2 — Combat

- [x] Initiative order + turn queue
- [x] Action / bonus action / reaction slot tracking
- [x] Attack roll: d20 + modifier vs AC, critical hit/miss
- [x] Damage roll with damage type
- [x] Saving throws
- [x] Condition application (poisoned, stunned, etc.)
- [x] Combat UI screen (`src/ui/screens/combat.rs`)
  - See [gameplay/combat.md](../gameplay/combat.md)

## Milestone 3 — Story & Dialog

- [x] Quest stage machine + TOML quest loader
- [x] Dialog tree evaluator
- [x] Journal entry system
- [x] Environmental lore (inspect action)
- [x] Dialog UI screen
- [x] Journal UI screen
  - See [gameplay/story.md](../gameplay/story.md), [gameplay/dialog.md](../gameplay/dialog.md)

## Milestone 4 — Items & Spells

- [x] Inventory system + equipment slots
- [x] Weapon/armor stat application
- [x] Spell slot tracking
- [x] Spell effect resolution
- [x] Spellbook UI screen
  - See [gameplay/items.md](../gameplay/items.md), [gameplay/spells.md](../gameplay/spells.md)

## Milestone 5 — NPC & Factions

- [x] Monster stat block loader
- [x] Basic NPC AI (melee, ranged, spellcaster behaviours)
- [x] Faction reputation system
- [x] Emergent world events triggered by WorldState
  - See [gameplay/npc-ai.md](../gameplay/npc-ai.md)

## Milestone 6 — First Region

- [x] Author region 1: `assets/regions/valley-of-ash/`
- [x] Author starter town NPC dialog
- [x] Author main quest Act 1 (3 stages)
- [x] Author 2 side quests
  - See [content/regions/index.md](../content/regions/index.md), [content/quests.md](../content/quests.md)

## Milestone 7 — Polish

- [x] Save / load game
- [x] Character creation screen
- [x] Main menu
- [x] Sound (optional — crossterm bell only)
- [x] README with screenshots

---

## Roadmap Policy (Post-M7)

- [x] Primary track: stability first (`M8`) before deeper systems/content
- [x] 4–6 week target: internal dev quality (not public alpha yet)
- [x] Execution split: 60% systems / 40% content
- [x] Warning policy: non-blocking until after `M9` (no broad warning-only cleanup)
- [x] AI/faction depth target: moderate (few robust behaviors over many brittle ones)

## Milestone 8 — Stability & Engineering Debt

- [x] Freeze gameplay scope temporarily
- [x] Add integration tests for core end-to-end flows:
  - world-map -> trigger -> combat
  - world-map -> trigger -> dialog
  - save/load roundtrip from active gameplay state
- [x] Tighten module boundaries and remove stale glue paths
- [x] Standardize dev automation entry points for agents (`scripts/`)
  - See [testing/tui-agent-smoke.md](../testing/tui-agent-smoke.md), [architecture/game-loop.md](../architecture/game-loop.md)

## Milestone 9 — Core RPG Depth

- [x] Level-up flow and class progression hooks in runtime
- [x] Spell scaling and slot usage depth improvements
- [x] Data-driven equipment effects in combat/runtime calculations
- [x] Expand combat action variety (targeted, high-signal improvements only)
  - See [gameplay/character.md](../gameplay/character.md), [gameplay/combat.md](../gameplay/combat.md), [gameplay/spells.md](../gameplay/spells.md)

## Milestone 10 — Content Production Pipeline

- [x] Region authoring templates and validation helpers
- [x] Author two additional regions beyond `valley-of-ash`
- [x] Author quest/dialog content with reusable content workflow
- [x] Ensure new content loads without runtime code edits
  - See [content/regions/index.md](../content/regions/index.md), [content/map-format.md](../content/map-format.md), [content/quests.md](../content/quests.md)

## Milestone 11 — NPC/Faction Simulation 2.0

- [x] Expand behavior archetypes carefully (moderate-complexity target)
- [x] Make faction reputation materially affect dialog/hostility/support
- [x] Add emergent event chains driven by faction and world-state thresholds
  - See [gameplay/npc-ai.md](../gameplay/npc-ai.md), [gameplay/story.md](../gameplay/story.md)

## Milestone 12 — UX & Presentation Polish 2.0

- [x] Improve HUD/readability across key screens
- [x] Implement terminal capability tiers + runtime fallback policy (`T0`..`T3`)
- [x] Introduce shared semantic theme tokens (color roles, not hard-coded per screen)
- [x] Add icon atlas with portable fallback glyphs (text-first controls remain)
- [x] Add animation layer for transitions/combat feedback with bounded frame budget
- [x] Add accessibility toggles (reduced motion, high contrast)
- [x] Improve input help overlays and state feedback
- [x] Expand sound behavior only if signal/value is clear
- [x] Document support matrix and configuration in README
  - See [architecture/ui-layer.md](../architecture/ui-layer.md), [architecture/tui-visual-system.md](../architecture/tui-visual-system.md), [gameplay/overview.md](../gameplay/overview.md)

## Milestone 13 — Release Readiness

- [x] Save migration/versioning hardening
- [x] Performance and load/startup profiling pass
- [x] Packaging + release notes + contributor/dev handoff quality
  - See [decisions/adr-003-save-format.md](../decisions/adr-003-save-format.md), [AGENT.md](../AGENT.md)

## Milestone 14 — Playtest UX & Ash Gate Flow Fixes

- [x] Fix `ash_gate` room-flow softlock with a deterministic outbound path (travel trigger and/or door semantics)
- [x] Fix world-map header clipping so player identity/position/status lines are always visible
- [x] Fix map rendering duplication for NPC spawn glyphs (avoid visual double-`@` confusion)
- [x] Add lightweight combat feedback cues (hit/miss/crit/heal) with reduced-motion-safe fallback
- [x] Improve information hierarchy and empty-state readability on inventory/spellbook/journal
- [x] Add focused playtest regression tests for `ash_gate` exit flow and related UI rendering expectations
  - See [gameplay/world.md](../gameplay/world.md), [architecture/ui-layer.md](../architecture/ui-layer.md), [architecture/tui-visual-system.md](../architecture/tui-visual-system.md)

## Milestone 15 — Interactive Playtest Harness (Token-Efficient)

- [x] Add scenario-aware interactive playtest runner presets for critical rooms (`ash_gate`, `ember_square`, `river_watch`)
- [x] Add deterministic capture mode (bounded ticks, compact summaries, key-event snapshots)
- [x] Add playtest report schema (`docs/testing/reports/*.md`) for UI/readability/animation findings
- [x] Expand manual testing guidance to require before/after evidence for UX changes
  - See [testing/tui-agent-smoke.md](../testing/tui-agent-smoke.md), [architecture/ui-layer.md](../architecture/ui-layer.md)
- Scope boundary:
  - No new gameplay mechanics; tooling + observability only
- Target files/modules:
  - `scripts/agent_tui_smoke.sh`
  - `docs/testing/tui-agent-smoke.md`
  - `docs/testing/interactive-playtest-checklist.md` (new)
- Done when:
  - [x] `./scripts/agent_tui_smoke.sh --interactive --scenario ash_gate --token-efficient` works and is documented
  - [x] `./scripts/agent_tui_smoke.sh --capture --scenario ash_gate --max-frames 120` generates compact artifact output
  - [x] A sample report exists for one full `ash_gate` escape run
- Verification commands:
  - `cargo test`
  - `./scripts/agent_tui_smoke.sh --list-scenarios`
  - `./scripts/agent_tui_smoke.sh --capture --scenario ash_gate --max-frames 120 --token-efficient`
- Risks / non-goals:
  - Risk: capture growth becomes noisy; mitigate with strict line/frame caps
  - Non-goal: screenshot-perfect visual diffing

## Milestone 16 — Room/Traversal Reliability Sweep

- [x] Validate every region room for at least one deterministic outbound path
- [x] Add asset validation checks for missing exits/triggers and unreachable mandatory quest rooms
- [x] Add regression tests for travel transitions and map load fallbacks
  - See [gameplay/world.md](../gameplay/world.md), [content/map-format.md](../content/map-format.md)
- Scope boundary:
  - Reliability only; no region lore/content expansion
- Target files/modules:
  - `assets/regions/**/rooms/*.toml`
  - `src/game/world/`
  - `src/app.rs` integration tests
  - `scripts/validate_assets.sh` (if present) or new validator entrypoint
- Done when:
  - [x] validator fails on rooms with no exits/triggers unless explicitly marked terminal
  - [x] automated test covers transition out of each critical path room in current regions
  - [x] `ash_gate` remains covered with a dedicated regression test
- Verification commands:
  - `cargo test`
  - `cargo run -- --validate-assets`
- Risks / non-goals:
  - Risk: false positives for intentional dead-end puzzle rooms; mitigate with explicit opt-out flag in room data
  - Non-goal: redesigning map topology

## Milestone 17 — Combat UX Readability and Pacing

- [x] Improve combat log prioritization (last-turn summary + key event highlighting)
- [x] Add concise timeline strip (initiative + current actor clarity)
- [x] Tighten animation pacing with reduced-motion parity checks
- [x] Expand test coverage for combat feedback rendering paths
  - See [gameplay/combat.md](../gameplay/combat.md), [architecture/tui-visual-system.md](../architecture/tui-visual-system.md)
- Scope boundary:
  - Presentation and pacing only; balance changes deferred
- Target files/modules:
  - `src/ui/tui/screens/combat.rs`
  - `src/ui/tui/animation/`
  - `src/game/combat/` tests touching event formatting
- Done when:
  - [x] critical events (hit/miss/crit/downed/heal/status) are visible within 1 screen without scroll
  - [x] reduced-motion mode preserves full semantic feedback
  - [x] no frame-budget regressions beyond agreed threshold in docs
- Verification commands:
  - `cargo test`
  - `./scripts/agent_tui_smoke.sh --capture --scenario combat_baseline --max-frames 180 --token-efficient`
- Risks / non-goals:
  - Risk: visual emphasis increases noise; mitigate with capped highlight density
  - Non-goal: introducing new combat actions

## Milestone 18 — Quest/Dialog Consistency and Softlock Guards

- [x] Add quest graph checks for orphan stages and impossible prerequisites
- [x] Add dialog branch checks for missing targets and dead-end mandatory interactions
- [x] Add runtime guardrails and explicit player feedback on blocked progression states
  - See [gameplay/story.md](../gameplay/story.md), [gameplay/dialog.md](../gameplay/dialog.md), [content/quests.md](../content/quests.md)
- Scope boundary:
  - Consistency and recoverability only; narrative expansion deferred
- Target files/modules:
  - `assets/quests/*.toml`
  - `assets/dialog/*.toml`
  - `src/game/story/`
  - `src/game/dialog/`
- Done when:
  - [x] validator detects unreachable quest stages and broken dialog links
  - [x] runtime emits actionable feedback instead of silent progression failure
  - [x] regression tests cover at least one blocked-state recovery path
- Verification commands:
  - `cargo test`
  - `cargo run -- --validate-assets`
- Risks / non-goals:
  - Risk: strict validation blocks in-progress authoring drafts; mitigate with warning mode for local draft files
  - Non-goal: writing new quest arcs

## Milestone 19 — Long-Session Automation and CI Soak

- [x] Add long-session automation profile (multi-scenario chain, deterministic seeds, compact logs)
- [x] Add nightly/CI soak command for stability regressions (memory growth, panic detection, save/load drift)
- [x] Add milestone-level completion checklist template for automated agent handoffs
  - See [AGENT.md](../AGENT.md), [testing/tui-agent-smoke.md](../testing/tui-agent-smoke.md)
- Scope boundary:
  - Reliability and maintainability only; feature work deferred
- Target files/modules:
  - `scripts/agent_tui_smoke.sh`
  - `scripts/` CI helper scripts
  - `.github/workflows/` (if present)
  - `docs/tasks/current-sprint.md` handoff conventions
- Done when:
  - [x] one command runs a >=30 minute deterministic soak profile locally
  - [x] CI runs a shorter equivalent profile on each PR
  - [x] failure output includes direct reproduction command and seed
- Verification commands:
  - `cargo test`
  - `./scripts/agent_tui_smoke.sh --soak --profile standard --minutes 30 --token-efficient`
- Risks / non-goals:
  - Risk: CI runtime cost grows too high; mitigate with tiered profiles (PR short, nightly long)
  - Non-goal: external cloud load testing

## Milestone 20 — Save/State Drift Hardening

- [ ] Add snapshot invariants for `App`, `WorldState`, quest state, room/position, and combat context
- [ ] Add save/load drift tests across varied runtime states and repeated roundtrips
- [ ] Harden schema compatibility for optional/missing fields and forward-safe defaults
- [ ] Add `--validate-save <path>` command for structural save checks
- [ ] Add CI gate for save/load roundtrip suite
  - See [decisions/adr-003-save-format.md](../decisions/adr-003-save-format.md), [architecture/game-loop.md](../architecture/game-loop.md)
- Scope boundary:
  - Consistency and compatibility only; no new gameplay systems
- Target files/modules:
  - `src/game/save/`
  - `src/app.rs` save/load glue + tests
  - `src/main.rs` CLI
  - `.github/workflows/`
- Done when:
  - [ ] no invariant drift in repeated save/load loops
  - [ ] legacy and current saves load cleanly with explicit compatibility coverage
  - [ ] save validator reports actionable errors with non-zero exit on invalid files
- Verification commands:
  - `cargo test`
  - `cargo test save_and_load_roundtrip -- --nocapture`
  - `cargo run -- --validate-save saves/slot1.toml`
- Risks / non-goals:
  - Risk: over-strict validation blocks legitimate old saves; mitigate with compatibility warning mode
  - Non-goal: redesigning save schema

## Milestone 21 — Region Navigation Depth (Multi-room Expansion)

- [ ] Expand each region to a meaningful multi-room graph (branches + loops)
- [ ] Add critical-path traversal checks and optional-path checks
- [ ] Add validator rules for required progression routes and dead-end flags
- [ ] Add region traversal smoke scenarios for full path coverage
- [ ] Keep region lore/flow coherent while expanding topology
  - See [content/regions/index.md](../content/regions/index.md), [content/map-format.md](../content/map-format.md), [gameplay/world.md](../gameplay/world.md)
- Scope boundary:
  - Topology and traversal reliability only; no combat rebalance
- Target files/modules:
  - `assets/regions/**/rooms/*.toml`
  - `assets/regions/**/region.toml`
  - `src/data/validate.rs`
  - `src/app.rs` traversal tests
- Done when:
  - [ ] every active region has >= 5 rooms with at least one branching route
  - [ ] critical progression path is validator-covered and test-covered
  - [ ] no accidental dead-ends without explicit terminal-room annotation
- Verification commands:
  - `cargo run -- --validate-assets`
  - `cargo test world_map_travel -- --nocapture`
  - `scripts/agent_tui_smoke.sh --scenario ash_gate --token-efficient --max-frames 220`
- Risks / non-goals:
  - Risk: map growth introduces softlocks; mitigate with graph reachability checks
  - Non-goal: adding new regions

## Milestone 22 — Quest Runtime Robustness + Recovery UX

- [ ] Add runtime diagnostics for blocked quest transitions (condition + stage context)
- [ ] Emit explicit player-facing blocked-reason feedback in journal/UI
- [ ] Add guided recovery hooks for known stuck states
- [ ] Add regression tests for blocked-state recovery paths
- [ ] Add debug command/output for current quest graph/status
  - See [gameplay/story.md](../gameplay/story.md), [content/quests.md](../content/quests.md), [gameplay/journal.md](../gameplay/journal.md)
- Scope boundary:
  - Robustness and recovery only; no narrative expansion
- Target files/modules:
  - `src/game/story/quest.rs`
  - `src/app.rs` quest/journal feedback paths
  - `src/ui/tui/screens/journal.rs`
  - `src/data/validate.rs`
- Done when:
  - [ ] blocked progression always yields explicit reason text
  - [ ] at least one automated recovery path per known stuck-state class is covered
  - [ ] validator and runtime diagnostics agree on broken quest links
- Verification commands:
  - `cargo test quest -- --nocapture`
  - `cargo run -- --validate-assets`
  - `scripts/agent_tui_smoke.sh --scenario ember_square --token-efficient --max-frames 220`
- Risks / non-goals:
  - Risk: noisy feedback overwhelms journal; mitigate with compact reason formatting
  - Non-goal: writing new quest arcs

## Milestone 23 — Combat Depth Pass (AI + Targeting + Encounter Variety)

- [ ] Improve enemy target selection heuristics by role/threat/health context
- [ ] Expand encounter composition templates for better role variety
- [ ] Improve combat event summarization and condition resolution clarity
- [ ] Add deterministic combat simulations for stability checks
- [ ] Add dedicated combat scenario smoke profile
  - See [gameplay/combat.md](../gameplay/combat.md), [gameplay/npc-ai.md](../gameplay/npc-ai.md), [architecture/tui-visual-system.md](../architecture/tui-visual-system.md)
- Scope boundary:
  - Combat decision quality and readability only; no major ruleset expansion
- Target files/modules:
  - `src/game/combat/`
  - `src/app.rs` combat loop + logs
  - `src/ui/tui/screens/combat.rs`
  - `scripts/agent_tui_smoke.sh`
- Done when:
  - [ ] role-driven AI behaves deterministically under fixed seeds
  - [ ] encounter templates produce diverse but bounded outcomes
  - [ ] combat critical events remain visible without scrolling
- Verification commands:
  - `cargo test combat -- --nocapture`
  - `scripts/agent_tui_smoke.sh --capture-log /tmp/combat.log --token-efficient --max-frames 240 --scenario ash_gate`
- Risks / non-goals:
  - Risk: AI complexity reduces predictability; mitigate with seed-based test fixtures
  - Non-goal: introducing entirely new combat subsystems

## Milestone 24 — Release Candidate Pipeline

- [ ] Add `scripts/rc_check.sh` that composes tests, validators, smoke, soak, and save checks
- [ ] Add tiered CI lanes (fast PR lane + extended nightly lane)
- [ ] Add startup/perf budget assertions with failure messaging
- [ ] Ensure all failures emit direct reproduction command and seed
- [ ] Publish RC checklist and release runbook updates
  - See [AGENT.md](../AGENT.md), [testing/tui-agent-smoke.md](../testing/tui-agent-smoke.md), [releases/v0.1.0-internal.md](../releases/v0.1.0-internal.md)
- Scope boundary:
  - Release quality gating only; no net-new gameplay features
- Target files/modules:
  - `scripts/release_check.sh`
  - `scripts/rc_check.sh` (new)
  - `.github/workflows/`
  - `docs/releases/`
- Done when:
  - [ ] one command certifies RC gate readiness
  - [ ] CI enforces gates with bounded runtime and low flake rate
  - [ ] every failure artifact includes reproducible command context
- Verification commands:
  - `scripts/rc_check.sh`
  - `cargo test`
  - `cargo run -- --validate-assets`
  - `scripts/agent_tui_smoke.sh --soak --profile standard --minutes 5 --token-efficient --no-build`
- Risks / non-goals:
  - Risk: pipeline runtime becomes too slow; mitigate with strict tiering and nightly-only heavy checks
  - Non-goal: distribution packaging changes
