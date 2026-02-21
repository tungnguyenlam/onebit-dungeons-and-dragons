# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Last Session Handoff

```
Date:          2026-02-21
Completed:     Milestones 20-26 — Save hardening, region depth, quest robustness,
               combat AI, release pipeline, faction simulation, and second region content.

Tasks completed:
  M20 — Save/State Drift Hardening
  - ✅ SaveGame invariant checks + SaveDriftReport in src/game/save/mod.rs
  - ✅ --validate-save <path> CLI flag in src/main.rs
  - ✅ CI gate: cargo test save roundtrip suite

  M21 — Region Navigation Depth
  - ✅ valley-of-ash expanded: cinder_ridge, ash_hollow, soot_shrine (+3 rooms)
  - ✅ emberpeak-summit expanded: lava_shelf, peak_crater (+2 rooms)
  - ✅ ironhold-mines expanded: ore_chamber, flooded_pit (+2 rooms)
  - ✅ RoomDef.terminal field added (types.rs, room.rs, region.rs)
  - ✅ validate.rs: reachability BFS + min-2-rooms + branching checks
  - ✅ 3 new validate tests: all_regions_have_multiple_rooms,
       all_rooms_reachable_from_entry, regions_have_branching_paths

  M22 — Quest Runtime Robustness
  - ✅ QuestLog::blocked_quests() — detects stuck active stages
  - ✅ QuestLog::emit_blocked_hints() — writes Category::System journal entries
  - ✅ Category::System added to journal (journal.rs, app.rs, ui/screens/journal.rs)
  - ✅ 5 new quest tests

  M23 — Combat Depth Pass
  - ✅ src/game/combat/ai.rs (new): EnemyAiRole focus-fire targeting,
       EncounterTier CR classifier
  - ✅ 3 new monster assets: orc_warrior, orc_warchief, ember_wraith
  - ✅ 9 new AI tests

  M24 — Release Candidate Pipeline
  - ✅ scripts/rc_check.sh (new): tiered T1/T2/T3 RC gate script
  - ✅ .github/workflows/rust.yml restructured: fast / slow / release jobs

  M25 — Faction Simulation Expansion
  - ✅ WorldState::modify_faction_rep(id, delta) helper
  - ✅ App::check_room_hostilities() auto-initiates combat for hostile factions (rep <= -10)
  - ✅ Significant rep changes (>= ±5) emit journal entries
  - ✅ Inter-faction vouching event (goblin_tribe < -5 && town_guard > 5) unlocks dialog
  - ✅ 3 new faction simulation tests in src/app.rs

  M26 — Second Region Content Pass
  - ✅ Created Summit Crater room and integrated into Emberpeak Summit
  - ✅ Added Ember Wraith encounter and ember_rune lore hooks
  - ✅ Hooked up Volcanic Curse quest acceptance via Warden Brom's task flag
  - ✅ Validated all region assets

  Advanced TUI (Foundation)
  - ✅ FocusedPane enum and state added to App for independent widget focus.
  - ✅ Architecture doc updated with OpenCode-style TUI interaction patterns.

Tests at close: 149 passed, 0 failed

Next for incoming agent:
  - Start M27 (Audio & Ambient Layer)
  - Implement SoundQueue/SoundEffect systems.
  - Implement the independent scrollbar/focus logic on the Journal screen.
```

---

## Active Task

### Task: M28 — Difficulty & Accessibility Settings

**Files to touch:**
- src/app.rs
- src/ui/tui/screens/settings.rs
- src/ui/tui/mod.rs
- src/game/combat/mod.rs (or where multipliers can apply)

**Done when:**
- [ ] Settings enum handles standard toggles/multipliers.
- [ ] Setting screen is accessible from the Main Menu or pause.
- [ ] Combat equations respect the selected difficulty.

**Blocked by:** none

**Relevant docs:**
- docs/tasks/milestones/m28.md

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
