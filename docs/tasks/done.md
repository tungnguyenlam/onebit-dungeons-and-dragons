
# Done

> Completed tasks, newest first. Keep milestone/docs status synced using [../DOCS_MAP.md](../DOCS_MAP.md).
> Older records (M0–M13): [archive/done-m00-m13.md](archive/done-m00-m13.md)

---

## 2026-02-26 — Milestone 66: Border Exits Graph for Room Traversal

- [x] Added room-level directional exits model (`[exits]`) and runtime loading.
- [x] Switched edge transitions to `[exits]` graph (independent from trigger placement).
- [x] Kept trigger travel for non-local/special transitions; in-region trigger stepping no longer drives normal room traversal.
- [x] Backfilled exits across room assets and updated traversal tests/docs.

Files modified (major):
  - src/data/types.rs
  - src/game/world/room.rs
  - src/app/navigation/movement.rs
  - src/app/navigation/world_map_util.rs
  - src/app/tests/flow.rs
  - assets/regions/*/rooms/*.toml
  - docs/content/SCHEMAS.md
  - docs/content/map-format.md
  - docs/gameplay/world.md
  - docs/tasks/milestones/m66.md
  - docs/tasks/backlog.md
  - docs/tasks/current-sprint.md
  - docs/tasks/done.md

Validation:
  - cargo test: pass (151 tests)
  - visual_check scenarios: pass

---

## 2026-02-26 — Milestone 65: Traversal Directionality & Interaction Cleanup

- [x] Fixed edge-transition directionality in movement handling.
- [x] Ensured room-travel transitions consume turn progression.
- [x] Removed redundant first-room dialog trigger overlap with NPCs.
- [x] Added/updated flow + visual validation expectations.

Files modified:
  - src/app/navigation/movement.rs
  - src/app/tests/flow.rs
  - assets/regions/valley-of-ash/rooms/ash_gate.toml
  - tests/visual_scenarios.json
  - docs/tasks/milestones/m65.md
  - docs/tasks/backlog.md
  - docs/tasks/current-sprint.md
  - docs/tasks/done.md

Validation:
  - cargo test: pass (151 tests)
  - visual_check scenarios: pass

---

## 2026-02-26 — Milestone 64: Edge-Based Room Transitions

- [x] Added edge-driven intra-region room transitions in movement handling.
- [x] Added auto-travel when stepping onto travel trigger tiles.
- [x] Added flow tests for trigger-step and boundary-push transitions.
- [x] Updated map/world docs to describe traversal behavior.

Files modified:
  - src/app/navigation/movement.rs
  - src/app/tests/flow.rs
  - tests/visual_scenarios.json
  - docs/content/map-format.md
  - docs/gameplay/world.md
  - docs/tasks/milestones/m64.md
  - docs/tasks/backlog.md
  - docs/tasks/current-sprint.md
  - docs/tasks/done.md

Validation:
  - cargo check: pass
  - visual_check scenarios: pass
  - Note: `cargo test` blocked in this environment due crates.io DNS/network resolution failure.

---

## 2026-02-26 — Milestone 63: Map Scale, Connectivity, and Landmarks

- [x] Added room landmark support in runtime/data models.
- [x] Expanded all region room maps for larger playable spaces (35 room files).
- [x] Added world map widget support for current landmark + connected local room paths.
- [x] Updated schema/gameplay docs for landmark metadata and widget behavior.

Files modified (major):
  - src/data/types.rs
  - src/game/world/room.rs
  - src/game/world/region.rs
  - src/app/navigation/world_map_util.rs
  - src/ui/tui/widgets/map.rs
  - src/ui/tui/screens/world_map.rs
  - src/app/samples/region.rs
  - assets/regions/*/rooms/*.toml
  - docs/content/SCHEMAS.md
  - docs/content/map-format.md
  - docs/gameplay/world.md
  - docs/tasks/milestones/m63.md
  - docs/tasks/backlog.md
  - docs/tasks/current-sprint.md
  - docs/tasks/done.md

Validation:
  - cargo check: pass
  - visual_check scenarios: pass (all scenarios in `tests/visual_scenarios.json`)

---

## 2026-02-26 — Milestone 62: Visual Playtest Bug Sweep

- [x] Removed duplicated HP label rendering in world/inventory UI.
- [x] Made world-map footer height dynamic so feedback lines no longer clip control hints.
- [x] Added scenario assertion support to `scripts/visual_check.py` (`expected_contains` / `expected_not_contains`).
- [x] Updated `tests/visual_scenarios.json` with deterministic key sequences and expectations.
- [x] Updated visual testing docs for scenario assertions.

Files modified:
  - src/ui/tui/screens/world_map.rs
  - src/ui/tui/screens/inventory.rs
  - scripts/visual_check.py
  - tests/visual_scenarios.json
  - docs/testing/step-through-testing.md
  - docs/tasks/milestones/m62.md
  - docs/tasks/backlog.md
  - docs/tasks/current-sprint.md
  - docs/tasks/done.md

Validation:
  - cargo check: pass
  - visual_check scenarios: pass (`start_game`, `enter_world`, `move_around`, `open_inventory`, `combat_init`, `combat_attack`, `combat_flee`, `tidewatch_start`, `volcanic_curse_quest`, `underdark_shelf`)

---

## 2026-02-26 — M61 follow-up bugfix (world map visibility regression)

Issue:
  - Non-fog weather was incorrectly using FOV masking, causing parts of larger rooms to disappear from map view.

Fix:
  - Apply FOV masking only when weather is `fog`.
  - Keep full-room visibility for clear/rain/ash/snow.

Files modified:
  - src/ui/tui/screens/world_map.rs
  - docs/gameplay/world.md

Validation:
  - cargo check: pass
  - Headless visual dump: `test_outputs/m61_visibility_fix.txt`

---

## 2026-02-26 — Milestone 61: Map Widgets & World Map Utility

- [x] Added world-map utility in `src/app/navigation/world_map_util.rs` to build region overview data.
- [x] Added reusable map widgets in `src/ui/tui/widgets/map.rs` (room list + exits).
- [x] Integrated widget panel into `src/ui/tui/screens/world_map.rs`.
- [x] Added utility/widget tests and verified world map rendering path.

**Validation:** `cargo check` pass, `cargo test` pass, headless visual dump snapshot saved.

---

## 2026-02-26 — Milestone 59 + 60: Weather, Hazards, Final Boss, Ending, New Game+

M59:
- [x] Weather/hazard systems
- [x] Weather-aware combat/FOV

M60:
- [x] Final boss encounter trigger/content
- [x] Ending calculation + credits screen
- [x] New Game+ option
- [x] Docs + backlog + done updates
- [x] Validation + headless visual dumps

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

---

## 2026-02-26 — Milestone 58: Bestiary & Shared Lore UI

- [x] Added persistent discovery sets in `src/game/story/world_state/types.rs` with helper APIs in `src/game/story/world_state/flags.rs`.
- [x] Wired lore discovery through `src/game/story/events.rs::inspect_lore`.
- [x] Wired monster discovery + kill counters in `src/app/combat/actions.rs` on combat victory.
- [x] Implemented `src/ui/tui/screens/bestiary.rs` and `src/ui/tui/screens/lore_library.rs`.
- [x] Added `AppState::{Bestiary,LoreLibrary}` and event routing in app/renderer/input layers.
- [x] Added world/journal shortcuts (`v` bestiary, `y` lore library) and updated world/journal screen hints.
- [x] Completed remaining M57 harvest integration by invoking `harvest_from_monster` after combat wins.

**Validation:** `cargo check` passes; headless visual dumps captured via `scripts/visual_check.py`.

---

## 2026-02-26 — Milestone 57: Crafting & Alchemy Systems

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

---

## 2026-02-26 — Milestone 56: The Underdark Shelf & Act 3 Foundation

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

---

## 2026-02-26 — Milestone 61: Map Widgets & World Map Utility

- [x] Added world-map utility in `src/app/navigation/world_map_util.rs` to build region overview data.
- [x] Added reusable map widgets in `src/ui/tui/widgets/map.rs` (room list + exits).
- [x] Integrated widget panel into `src/ui/tui/screens/world_map.rs`.
- [x] Added utility/widget tests and verified world map rendering path.

**Validation:** `cargo check` pass, `cargo test` pass, headless visual dump snapshot saved.

## 2026-02-26 — Milestone 60: Final Boss, Ending Variations & New Game+

- [x] Added final boss asset `assets/monsters/void_architect.toml`.
- [x] Wired Underdark final encounter trigger in `assets/regions/underdark-shelf/rooms/abyss_entry.toml`.
- [x] Implemented ending evaluation logic in `src/game/story/ending.rs`.
- [x] Added ending/credits TUI screen in `src/ui/tui/screens/ending.rs` and routed `AppState::Ending`.
- [x] Combat victory now sets completion flags and transitions to ending after defeating `void_architect`.
- [x] Added New Game+ menu path with retained progression and higher difficulty baseline.

## 2026-02-26 — Milestone 59: Dynamic Weather & Environmental Hazards

- [x] Added weather model in `src/game/world/weather.rs`.
- [x] Weather flags are applied each story tick in `src/app/systems.rs`.
- [x] Added hazard tile effects on movement (deep water/pit/rift) in `src/app/navigation/movement.rs`.
- [x] Added weather impact to combat rolls in `src/game/combat/attack/engine.rs`.
- [x] Added fog-based FOV masking and weather effect indicators in `src/ui/tui/screens/world_map.rs`.
- [x] Added combat attack weather tests in `src/game/combat/attack/tests.rs`.

## 2026-02-26 — Milestone 58: Bestiary & Shared Lore UI

- [x] Added persistent discovery sets in `src/game/story/world_state/types.rs` with helper APIs in `src/game/story/world_state/flags.rs`.
- [x] Wired lore discovery through `src/game/story/events.rs::inspect_lore`.
- [x] Wired monster discovery + kill counters in `src/app/combat/actions.rs` on combat victory.
- [x] Implemented `src/ui/tui/screens/bestiary.rs` and `src/ui/tui/screens/lore_library.rs`.
- [x] Added `AppState::{Bestiary,LoreLibrary}` and event routing in app/renderer/input layers.
- [x] Added world/journal shortcuts (`v` bestiary, `y` lore library) and updated world/journal screen hints.
- [x] Completed remaining M57 harvest integration by invoking `harvest_from_monster` after combat wins.

**Validation:** `cargo check` passes; headless visual dumps captured via `scripts/visual_check.py`.

## 2026-02-26 — Milestone 55: Act 2 Main Quest: The Volcanic Curse

### Quest Implementation
- [x] `assets/quests/main/volcanic-curse.toml` — Updated with full quest stages (investigation, artifact retrieval, ritual choice)
- [x] `assets/items/cursed_volcanic_artifact.toml` — Created quest item with volcanic curse effect
- [x] `assets/regions/emberpeak-summit/dialog/warden_brom.toml` — Enhanced with lore about volcano and dwarf-drow conflict
- [x] `assets/regions/emberpeak-summit/dialog/archivist_nyra.toml` — Enhanced with lore about volcanic curse and artifact location
- [x] `src/app/navigation/interaction.rs` — Implemented trigger condition check
- [x] `assets/regions/valley-of-ash/rooms/cinder_ridge.toml` — Added condition to travel trigger
- [x] `docs/content/quests.md` — Documented the completed Volcanic Curse quest
- [x] `tests/visual_scenarios.json` — Added visual test scenario for the new quest

### Quest Stages
1. **Start:** Unlocks after completing Valley Contract (Act 1)
2. **Retrieve Artifact:** Find the cursed artifact in Ironhold Mines
3. **Ritual Choice:** Decide to perform the ritual or destroy the artifact
4. **Resolution:** Volcano stabilizes with different outcomes based on choice

**Key Features:**
- Cursed artifact imposes periodic fire damage/disadvantage while equipped
- Travel to Emberpeak now requires Act 1 completion
- Enhanced dialog with dwarven lore and backstory
- Trigger conditions for quests and travel now properly implemented

## 2026-02-22 — Milestone 52: Flee Mechanics & Combat AI Targeting

### Combat Flow Refinements
- Added a new `try_flee` mechanism in `src/app/combat.rs`.
- Mapped `f` or `4` to `GameEvent::Choice(4)` in `src/main.rs` and `tui/mod.rs` to trigger fleeing.
- Added opposed dexterity (initiative_mod) check vs max enemy initiative_mod to determine Flee success.
- If successful, instantly transitions state to `WorldMap`.
- Updated `select_enemy_target` to use `ctx.seed` for pseudorandom selection of targets.
- Added visual trace scenario `combat_flee` and confirmed accurate TUI logs and state behavior.

## 2026-02-21 — Bug Fixes: Feedback System and Region Characteristics

### Fixed compilation errors in src/app/mod.rs:
- `get_npc_at_player_position()` was using NpcDef.position which doesn't exist
  (NPCs are placed via tile grid, not coordinates)
- Fixed type mismatches (i32 vs u32) in is_near_door, is_near_chest, is_blocked
- interact_current_tile now properly checks for triggers at player position

### Added feedback message system:
- Added feedback_message field to App struct with 3-second timeout
- Added set_feedback() and get_feedback() methods
- Updated interact_current_tile() to provide feedback for failed interactions
- Updated world_map.rs to display feedback in footer area

### Extended region system with unique characteristics:
- Added region_type field to RegionManifest and Region (volcanic, forest, underwater, underground, mountain)
- Added weather field to RegionManifest and Region (ash, fog, rain, none)
- Updated all 6 region manifest files with these new fields
- Updated world_map.rs to display weather in header

Files modified:
- src/app/mod.rs
- src/ui/tui/screens/world_map.rs
- src/data/types.rs
- src/game/world/region.rs
- src/app/samples.rs
- assets/regions/*/region.toml (6 files)

---

## 2026-02-21 — Milestones 33-34 (complete): Item Pool Expansion, Multi-Region Quest Chains
### M34 — Multi-Region Quest Chains
- [x] `assets/quests/main/obsidian-scourge.toml` — Refactored for multi-boss flow (Ignis/Malphas)
- [x] `assets/regions/` — Distributed legendary artifacts (Eye, Heart, Scepter)
- [x] `src/data/validate.rs` — Verified cross-region asset dependencies

### M33 — Item Pool Expansion Part 1
- [x] `assets/items/` — 20+ new "One-Bit" items added
- [x] `src/app/tests.rs` — Added elemental resistance verification test
- [x] `src/app/combat.rs` — Fixed character mod application to damage dice and improved log messages

## 2026-02-21 — Milestones 27-32 (complete): Audio, Settings, v0.2.0 Release, World Map, Whispering Woods, Bestiary

### M32 — Bestiary Expansion Part 1
- [x] `assets/monsters/` — Created 10+ new enemies (e.g., goblin variants, forest threats)
- [x] `src/app/samples.rs` — Fixed serialization for dice properties and conditions
- [x] `src/game/combat/combat.rs` — Verified condition loading from assets

### M31 — The Whispering Woods
- [x] `assets/regions/whispering-woods/` — New 5-room region content
- [x] `assets/regions/valley-of-ash/rooms/ash_hollow.toml` — Linked to Woods
- [x] `src/data/validate.rs` — Support for room-to-room cross-region validation

### M30 — World Map Structure Update
- [x] `src/app/mod.rs` — Cross-region travel logic in `interact_current_tile`
- [x] `assets/regions/valley-of-ash/rooms/cinder_ridge.toml` — Linked to Emberpeak
- [x] `assets/regions/emberpeak-summit/rooms/south_slope.toml` — Linked to Valley

### M29 — v0.2.0 Release Gate
- [x] `scripts/rc_check.sh` — Passed all T1-T3 gates
- [x] `Cargo.toml` — Version bumped to 0.2.0

### M28 — Difficulty & Accessibility Settings
- [x] `src/app/state.rs` — `SettingsUiState` and `SettingsConfig`
- [x] `src/app/combat.rs` — HP and damage scaling based on multipliers
- [x] `src/ui/tui/screens/settings.rs` — Settings screen UI

### M27 — Audio & Ambient Layer
- [x] `src/renderer.rs` — `SoundEffect` enum and `sound_queue` infrastructure
- [x] `assets/regions/` — Ambient tags added to region manifests

## 2026-02-21 — Milestones 20-26 (complete): Hardening, Depth, Robustness, AI, RC Pipeline, Faction Simulation, Content Pass

### M26 — Second Region Content Pass
- [x] `assets/regions/emberpeak-summit/rooms/summit_crater.toml` (new) — added missing region room
- [x] `assets/regions/emberpeak-summit/region.toml` — integrated Summit Crater
- [x] `src/app.rs` — hooked `accept_emberpeak_rune_task` flag to `volcanic_curse` quest
- [x] `assets/regions/emberpeak-summit/rooms/` — added `ember_wraith` encounter to Summit Crater
- [x] `assets/regions/emberpeak-summit/npcs/` — verified `emberpeak_dwarves` faction affiliation

### M25 — Faction Simulation Expansion
- [x] `src/game/story/world_state.rs` — `modify_faction_rep(id, delta)`
- [x] `src/app.rs` — `check_room_hostilities()` (rep <= -10 auto-combat), ±5 journaling
- [x] `src/game/story/events.rs` — `ModifyFactionRep` world event support
- [x] `assets/regions/valley-of-ash/` — `town_guard_vouched` dialog unlock path for `captain_kael`
- [x] `src/app.rs` — inter-faction logic (goblins vs guards relationship)

### M20 — Save/State Drift Hardening
- [x] `src/game/save/mod.rs` — `SaveDriftReport`, invariant checks, `validate_save_file()`
- [x] `src/main.rs` — `--validate-save <path>` CLI flag
- [x] `.github/workflows/rust.yml` — save/load roundtrip CI gate

### M21 — Region Navigation Depth
- [x] `assets/regions/*/rooms/` — expanded all 3 regions: `cinder_ridge`, `ash_hollow`, `soot_shrine`, `lava_shelf`, `peak_crater`, `ore_chamber`, `flooded_pit`
- [x] `src/data/types.rs` + `room.rs` + `region.rs` — `terminal` field added
- [x] `src/data/validate.rs` — BFS reachability, min-2-rooms, branching checks + 3 tests

### M22 — Quest Runtime Robustness
- [x] `src/game/story/quest.rs` — `blocked_quests()`, `emit_blocked_hints()`, `BlockedReason`
- [x] `src/game/story/journal.rs` — `Category::System` + `Journal::entries()`
- [x] All match sites updated in `app.rs` + `ui/screens/journal.rs`

### M23 — Combat Depth Pass
- [x] `src/game/combat/ai.rs` (new) — focus-fire targeting by role, `EncounterTier`
- [x] `assets/monsters/` — `orc_warrior`, `orc_warchief`, `ember_wraith` (new)

### M24 — Release Candidate Pipeline
- [x] `scripts/rc_check.sh` (new) — T1 fast / T2 slow / T3 soak gate
- [x] `.github/workflows/rust.yml` — `fast` → `slow` → `release` tiered jobs

**Test result:** 149 passed, 0 failed

---

## 2026-02-21 — Milestones 16-19 (complete): Reliability, Readability, Consistency, Soak

- [x] `src/data/validate.rs` (new) — `--validate-assets` with room/dialog/quest graph checks
- [x] `src/app.rs` — dialog softlock guardrails emit explicit journal feedback
- [x] `src/ui/tui/screens/combat.rs` — timeline strip, last-turn summary, reduced-motion styling
- [x] `scripts/visual_check.py` — scenario-based smoke/soak runs
- [x] `.github/workflows/rust.yml` — CI runs asset validation + PR soak
- [x] `docs/tasks/milestone-checklist-template.md` — new handoff template

---

## 2026-02-20 — Milestone 14-15 (complete): Playtest UX + Interactive Harness

- [x] M14: `ash_gate` → `ember_square` travel fixed; smoke/soak token-efficient mode added
- [x] M15: interactive playtest harness scenarios (`ash_gate`, `ember_square`, `ember_summit`)
- [x] `cargo test` passes (118 tests, 0 failures)

---

> M0–M13 records: [archive/done-m00-m13.md](archive/done-m00-m13.md)
