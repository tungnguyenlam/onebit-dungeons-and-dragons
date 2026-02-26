# Done

> Completed tasks, newest first. Keep milestone/docs status synced using [../DOCS_MAP.md](../DOCS_MAP.md).
> Older records (M0–M13): [archive/done-m00-m13.md](archive/done-m00-m13.md)

---

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
- [x] `scripts/agent_tui_smoke.sh` — soak mode (`--soak --profile standard --minutes`)
- [x] `.github/workflows/rust.yml` — CI runs asset validation + PR soak
- [x] `docs/tasks/milestone-checklist-template.md` — new handoff template

---

## 2026-02-20 — Milestone 14-15 (complete): Playtest UX + Interactive Harness

- [x] M14: `ash_gate` → `ember_square` travel fixed; smoke/soak token-efficient mode added
- [x] M15: interactive playtest harness scenarios (`ash_gate`, `ember_square`, `ember_summit`)
- [x] `cargo test` passes (118 tests, 0 failures)

---

> M0–M13 records: [archive/done-m00-m13.md](archive/done-m00-m13.md)
