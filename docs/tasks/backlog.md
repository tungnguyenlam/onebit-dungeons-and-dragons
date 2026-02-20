# Backlog

> Ordered roughly by dependency. Pick from the top.
> Move items to [current-sprint.md](current-sprint.md) when starting, then to [done.md](done.md) on completion.

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
- [ ] `src/game/world/` — region loader, tile map, room graph, FOV  ← **NEXT**
  - See [gameplay/world.md](../gameplay/world.md)
- [ ] `src/game/story/world_state.rs` — flag store, save/load
  - See [gameplay/story.md](../gameplay/story.md)

## Milestone 2 — Combat

- [ ] Initiative order + turn queue
- [ ] Action / bonus action / reaction slot tracking
- [ ] Attack roll: d20 + modifier vs AC, critical hit/miss
- [ ] Damage roll with damage type
- [ ] Saving throws
- [ ] Condition application (poisoned, stunned, etc.)
- [ ] Combat UI screen (`src/ui/screens/combat.rs`)
  - See [gameplay/combat.md](../gameplay/combat.md)

## Milestone 3 — Story & Dialog

- [ ] Quest stage machine + TOML quest loader
- [ ] Dialog tree evaluator
- [ ] Journal entry system
- [ ] Environmental lore (inspect action)
- [ ] Dialog UI screen
- [ ] Journal UI screen
  - See [gameplay/story.md](../gameplay/story.md), [gameplay/dialog.md](../gameplay/dialog.md)

## Milestone 4 — Items & Spells

- [ ] Inventory system + equipment slots
- [ ] Weapon/armor stat application
- [ ] Spell slot tracking
- [ ] Spell effect resolution
- [ ] Spellbook UI screen
  - See [gameplay/items.md](../gameplay/items.md), [gameplay/spells.md](../gameplay/spells.md)

## Milestone 5 — NPC & Factions

- [ ] Monster stat block loader
- [ ] Basic NPC AI (melee, ranged, spellcaster behaviours)
- [ ] Faction reputation system
- [ ] Emergent world events triggered by WorldState
  - See [gameplay/npc-ai.md](../gameplay/npc-ai.md)

## Milestone 6 — First Region

- [ ] Author region 1: `assets/regions/valley-of-ash/`
- [ ] Author starter town NPC dialog
- [ ] Author main quest Act 1 (3 stages)
- [ ] Author 2 side quests
  - See [content/regions/index.md](../content/regions/index.md), [content/quests.md](../content/quests.md)

## Milestone 7 — Polish

- [ ] Save / load game
- [ ] Character creation screen
- [ ] Main menu
- [ ] Sound (optional — crossterm bell only)
- [ ] README with screenshots
