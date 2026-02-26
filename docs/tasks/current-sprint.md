# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Active Task

**Milestone:** Backlog complete (M60 done)

**Goal:** All listed milestones through M60 are completed; next work should come from new milestone definition.

**Spec:** Define next milestone in `docs/tasks/backlog.md` and add `docs/tasks/milestones/mXX.md`

---

```
Date:          2026-02-26
Completed:     M59 + M60 in one pass

Execution TODO plan:
  [x] M59 weather/hazard systems
  [x] M59 weather-aware combat/FOV
  [x] M60 final boss encounter trigger/content
  [x] M60 ending calculation + credits screen
  [x] M60 New Game+ option
  [x] Docs + backlog + done updates
  [x] Validation + headless visual dumps

Files modified/created (major):
  - src/game/world/weather.rs (new)
  - src/app/navigation/movement.rs
  - src/app/systems.rs
  - src/ui/tui/screens/world_map.rs
  - src/game/combat/attack/types.rs
  - src/game/combat/attack/engine.rs
  - src/app/combat/attack.rs
  - src/app/combat/ai.rs
  - src/game/story/ending.rs (new)
  - src/game/story/mod.rs
  - src/ui/tui/screens/ending.rs (new)
  - src/app/state/app_state.rs
  - src/app/combat/actions.rs
  - src/app/handlers/ui.rs
  - src/app/handlers/menus.rs
  - src/ui/tui/screens/main_menu.rs
  - assets/monsters/void_architect.toml (new)
  - assets/regions/underdark-shelf/rooms/abyss_entry.toml
  - docs/tasks/milestones/m59.md
  - docs/tasks/milestones/m60.md
  - docs/tasks/backlog.md
  - docs/tasks/done.md

Validation:
  - cargo check: pass
  - cargo test: pass (144 tests)
  - Headless visual dump snapshots:
    - test_outputs/m59_weather_world.txt
    - test_outputs/m60_ending_screen.txt

Next for incoming agent:
  - Define M61 and add its spec before coding.
```

```
Date:          2026-02-26
Completed:     M58 - Bestiary & Shared Lore UI

M58 TODO plan:
  [x] Add explicit discovery sets and helper APIs in WorldState for monsters/lore.
  [x] Wire discovery events: monster defeat updates bestiary; lore inspection updates lore library.
  [x] Add new UI states/screens for Bestiary and Lore Library.
  [x] Add key bindings + renderer/handler wiring for accessing new screens.
  [x] Validate via cargo check and headless visual dump snapshots.

M57 completion check:
  - Status before this session: partially complete (harvesting helper existed but was not integrated into combat resolution).
  - Completed in this session: combat victory now triggers `harvest_from_monster` for defeated enemies and awards harvested ingredients.

Files modified:
  - src/game/story/world_state/types.rs
  - src/game/story/world_state/flags.rs
  - src/game/story/events.rs
  - src/app/combat/actions.rs
  - src/app/actions.rs
  - src/app/state/app_state.rs
  - src/app/mod.rs
  - src/renderer.rs
  - src/main.rs
  - src/ui/tui/input.rs
  - src/ui/gui/mod.rs
  - src/app/handlers/world.rs
  - src/app/handlers/ui.rs
  - src/ui/tui/renderer.rs
  - src/ui/tui/screens/mod.rs
  - src/ui/tui/screens/journal.rs
  - src/ui/tui/screens/world_map.rs
  - src/ui/tui/screens/bestiary.rs (new)
  - src/ui/tui/screens/lore_library.rs (new)
  - docs/tasks/milestones/m58.md
  - README.md

Build status: cargo check passes.
Headless visual dump: snapshots captured for `enter_world + v` and `enter_world + y`.

Next for incoming agent:
  - Start M59 implementation from `docs/tasks/milestones/m59.md`.
```

```
Date:          2026-02-26
Completed:     M57 - Crafting & Alchemy Systems

Tasks completed this session:
  1. Added is_ingredient and crafting_tags to ItemDef in data/types.rs
  2. Created RecipeDef, RecipeIngredient, and RecipeSkillCheck types
  3. Added RecipeDef to GlobalAssets and implemented recipe loading
  4. Created CraftingSystem in game/items/crafting.rs
  5. Added crafting methods to app/actions.rs (craft_item, get_available_recipes, harvest_from_monster)
  6. Created Crafting UI screen (src/ui/tui/screens/crafting.rs)
  7. Added AppState::Crafting and key bindings (c key)
  8. Created ingredient items (spider_silk, dragon_scale, poison_sac, crystal_shard, leather)
  9. Created result items (healing_potion_v2, reinforced_boots, dragon_scale_shield)
  10. Created 5 recipes in assets/recipes/
  11. Fixed all compilation errors and test failures

Files modified/created:
  - src/data/types.rs (added RecipeDef, is_ingredient, crafting_tags)
  - src/data/loader/dir.rs (added RecipeDef HasId implementation)
  - src/data/loader/global.rs (added recipes loading)
  - src/game/items/crafting.rs (new - CraftingSystem)
  - src/game/items/mod.rs (added crafting module)
  - src/app/mod.rs (added recipe_defs field, handle_crafting dispatch)
  - src/app/actions.rs (added craft_item, get_available_recipes, harvest_from_monster)
  - src/app/state/app_state.rs (added Crafting state)
  - src/app/handlers/ui.rs (added handle_crafting)
  - src/app/handlers/world.rs (added OpenCrafting key binding)
  - src/app/samples/items.rs (added new fields to samples)
  - src/app/tests/equipment.rs (added new fields)
  - src/ui/tui/screens/crafting.rs (new - Crafting UI)
  - src/ui/tui/screens/mod.rs (added crafting module)
  - src/ui/tui/renderer.rs (added Crafting rendering)
  - src/ui/tui/input.rs (added 'c' key for crafting)
  - src/main.rs (added Crafting render, key binding)
  - src/renderer.rs (added OpenCrafting event)
  - src/ui/gui/mod.rs (added OpenCrafting key)
  - assets/items/spider_silk.toml (new)
  - assets/items/dragon_scale.toml (new)
  - assets/items/poison_sac.toml (new)
  - assets/items/crystal_shard.toml (new)
  - assets/items/leather.toml (new)
  - assets/items/healing_potion_v2.toml (new)
  - assets/items/reinforced_boots.toml (new)
  - assets/items/dragon_scale_shield.toml (new)
  - assets/recipes/*.toml (5 recipe files)

Build status: cargo test passes (141 tests), asset validation passes.

Next for incoming agent:
  - Implement the Survival skill check for crafting (optional enhancement)
  - Add more recipes for different item types
  - Add harvest mechanic integration with combat (call harvest_from_monster after combat)
```

```
Date:          2026-02-26
Completed:     M56 - The Underdark Shelf & Act 3 Foundation

Tasks completed this session:
  1. Added Pit and Rift tile types to src/game/world/map.rs
  2. Created Underdark Shelf region (region.toml) with 5 rooms
  3. Created 5 rooms: cavern_entrance, fungal_groves, crystal_lake, drow_outpost, abyss_entry
  4. Created drow-merchant-coven faction with 4 NPCs (merchant_zae, spore_herder, crystal_seer, myconid_sapient)
  5. Created dialog files for all NPCs with faction reputation logic
  6. Created Act 3 quest (silence-below.toml) with exploration, alliance-building, and final confrontation
  7. Added connection from Ironhold Mines to Underdark Shelf (requires ritual_completed flag)
  8. Added obsidian_heart item to ore_chamber (reward for completing volcanic curse)
  9. Created rope_of_climbing and glowing_spores items
  10. Created abyss_runes lore entry
  11. Updated src/app/debug.rs to handle new tile types
  12. Added visual test scenario for underdark_shelf
  13. Fixed all asset validation errors

Files modified/created:
  - src/game/world/map.rs (modified - added Pit/Rift tiles)
  - src/app/debug.rs (modified - added Pit/Rift to debug dump)
  - assets/regions/underdark-shelf/region.toml (created)
  - assets/regions/underdark-shelf/rooms/*.toml (5 files created)
  - assets/regions/underdark-shelf/npcs/*.toml (4 files created)
  - assets/regions/underdark-shelf/dialog/*.toml (4 files created)
  - assets/quests/main/silence-below.toml (created)
  - assets/items/rope_of_climbing.toml (created)
  - assets/items/glowing_spores.toml (created)
  - assets/lore/abyss_runes.toml (created)
  - assets/regions/ironhold-mines/region.toml (modified - added connection)
  - assets/regions/ironhold-mines/rooms/ore_chamber.toml (modified - added item)
  - tests/visual_scenarios.json (modified - added scenario)

Build status: cargo test passes (138 tests), asset validation passes.

Next for incoming agent:
  - Implement the final boss encounter for Act 3 (M60)
  - Add more visual test scenarios for Underdark Shelf exploration
  - Add verticality mechanics (Dexterity saves for Pit/Rift tiles)
```

```
Date:          2026-02-26
Completed:     M55 - Act 2 Main Quest: The Volcanic Curse

Tasks completed this session:
  1. Updated `volcanic-curse.toml` to include full quest stages with investigation, artifact retrieval, and ritual choice
  2. Created `cursed_volcanic_artifact.toml` quest item with volcanic curse effect
  3. Enhanced Warden Brom's dialog to include lore about the volcano and dwarf-drow conflict
  4. Enhanced Archivist Nyra's dialog to include lore about the volcanic curse and artifact location
  5. Implemented trigger condition check in `execute_trigger` method
  6. Added condition to travel trigger in Valley of Ash to only allow travel to Emberpeak after completing Valley Contract quest
  7. Updated `quests.md` to document the completed Volcanic Curse quest
  8. Added visual test scenario for the new quest

Files modified/created:
  - assets/quests/main/volcanic-curse.toml (modified)
  - assets/items/cursed_volcanic_artifact.toml (created)
  - assets/regions/emberpeak-summit/dialog/warden_brom.toml (modified)
  - assets/regions/emberpeak-summit/dialog/archivist_nyra.toml (modified)
  - assets/regions/valley-of-ash/rooms/cinder_ridge.toml (modified)
  - src/app/navigation/interaction.rs (modified)
  - docs/content/quests.md (modified)
  - tests/visual_scenarios.json (modified)

Build status: cargo test passes (138 tests), asset validation passes.

Next for incoming agent:
  - Implement the ritual choice mechanic in Ironhold Mines
  - Add the cursed artifact to the Ironhold Mines loot table
  - Implement volcanic curse effects (periodic fire damage/disadvantage)
  - Test the full quest flow from Valley of Ash to Emberpeak and Ironhold Mines
  - Add more visual test scenarios for different quest outcomes
```

```
Date:          2026-02-22
Completed:     M52 completed (Flee Mechanics & AI Targeting)

Tasks completed this session:
  1. Defined and executed Milestone 52 based on previous combat system refinements.
  2. Implemented `try_flee` logic (contested Dexterity/initiative_mod) inside `src/app/combat.rs`.
  3. Bound `f` and `4` to Flee Action across TUI event handlers.
  4. Formatted UI combat footer to display `4/f flee` option.
  5. Refactored `select_enemy_target` to use `ctx.seed` for pseudo-random targeting rather than locking onto the 0th index.
  6. Expanded `scripts/visual_check.py` capabilities by adding `combat_flee` test scenario to `tests/visual_scenarios.json`.

Files modified:
  - docs/tasks/milestones/m52.md (created)
  - docs/tasks/backlog.md
  - docs/tasks/done.md
  - docs/tasks/current-sprint.md
  - src/main.rs
  - src/ui/tui/mod.rs
  - src/ui/tui/screens/combat.rs
  - src/app/handlers.rs
  - src/app/combat.rs
  - tests/visual_scenarios.json

Build status: cargo test passes cleanly (136 tests). 

Next for incoming agent:
  - Proceed with any remaining bug fixes or start defining and executing M53 (possibly related to Advanced Storytelling or visual scenarios).
```

```
Date:          2026-02-21
Completed:     M51 completed

Tasks completed this session:
  1. Updated `run_text_mode` in `src/main.rs`:
     - Replaced custom print loop with Ratatui's `TestBackend`
     - Added auto-load and auto-save of state to `save.toml`
  2. Fixed a deadlock bug in `src/ui/tui/theme.rs`:
     - `TIER.get_or_init(init_terminal_tier)` called `init_terminal_tier` which then called `TIER.get_or_init` again, causing an infinite hang. 
     - Fixed `init_terminal_tier` so it directly returns `TerminalTier` without going through the OnceLock.
  3. Updated `docs/testing/step-through-testing.md` to document new test output format and precise state persistence file (`save.toml`).
  4. Updated `scripts/runtest.sh` string documenting where state is persisted.

Files modified:
  - src/main.rs
  - src/ui/tui/theme.rs
  - scripts/runtest.sh
  - docs/testing/step-through-testing.md

Build status: cargo check passes (14 minor warnings)
Runtest script functions correctly without hanging and outputs full TUI buffers.

All requirements complete:
  ✅ State persists via save/load in run_text_mode
  ✅ `runtest.sh j` and similar commands move the player with saved context between invovcations
  ✅ Documentation updated with accurate test runner info

Next for incoming agent:
  - Proceed with the next milestone from the backlog
```
```
Date:          2026-02-21
Completed:     App Module Refactoring

Tasks completed this session:
  1. Refactored `src/app/mod.rs` into multiple submodules (navigation, equipment, progression, systems, actions, debug) to improve maintainability and adhere to file size limits.
  2. Moved `find_spawn_pos_for_room` to `src/app/samples.rs`.
  3. Cleaned up `src/app/mod.rs` to only contain core logic and struct definitions.
  4. Verified all changes with `cargo check` and `scripts/agent_verify.sh` (135 tests passed).

Files modified/created:
  - src/app/mod.rs (modified)
  - src/app/navigation.rs (new)
  - src/app/equipment.rs (new)
  - src/app/progression.rs (new)
  - src/app/systems.rs (new)
  - src/app/actions.rs (new)
  - src/app/debug.rs (new)
  - src/app/samples.rs (modified)

Build status: Finished dev profile [unoptimized + debuginfo] target(s) in 0.77s.
Warnings: 13 minor styling/unused variable warnings (unrelated to refactor).

Next for incoming agent:
  - Proceed with any new feature requests or further polish.
  - No pending milestones in backlog.md.
```

---

## Acceptance Criteria Template

When pulling a new task from the backlog, replace the Active Task block above
with a copy of this template, then update the Handoff block.

```
### Task: <name>

**Files to touch:**
- src/...

**Done when:**
- [ ] criterion 1
- [ ] criterion 2

**Blocked by:** (none / task name)

**Relevant docs:**
- docs/...
```
