# Done

> Completed tasks, newest first. Keep milestone/docs status synced using [../DOCS_MAP.md](../DOCS_MAP.md).
> Older records (M0–M13): [archive/done-m00-m13.md](archive/done-m00-m13.md)

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
