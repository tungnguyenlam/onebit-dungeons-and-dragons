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

- [ ] Improve HUD/readability across key screens
- [ ] Implement terminal capability tiers + runtime fallback policy (`T0`..`T3`)
- [ ] Introduce shared semantic theme tokens (color roles, not hard-coded per screen)
- [ ] Add icon atlas with portable fallback glyphs (text-first controls remain)
- [ ] Add animation layer for transitions/combat feedback with bounded frame budget
- [ ] Add accessibility toggles (reduced motion, high contrast)
- [ ] Improve input help overlays and state feedback
- [ ] Expand sound behavior only if signal/value is clear
- [ ] Document support matrix and configuration in README
  - See [architecture/ui-layer.md](../architecture/ui-layer.md), [architecture/tui-visual-system.md](../architecture/tui-visual-system.md), [gameplay/overview.md](../gameplay/overview.md)

## Milestone 13 — Release Readiness

- [ ] Save migration/versioning hardening
- [ ] Performance and load/startup profiling pass
- [ ] Packaging + release notes + contributor/dev handoff quality
  - See [decisions/adr-003-save-format.md](../decisions/adr-003-save-format.md), [AGENT.md](../AGENT.md)
