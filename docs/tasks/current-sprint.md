# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**

---

## Last Session Handoff

```
Date:          2026-02-20
Stopped at:    Milestone 1 — world + WorldState complete; combat is NEXT
Task in progress: M2 combat — initiative, attack rolls, action economy

What was completed this session:
  Milestone 0 (complete — carried over):
    - Cargo.toml with tui/gui feature flags + clap
    - src/renderer.rs, src/app.rs, src/main.rs, src/ui/

  Milestone 1 (NOW COMPLETE):
    - src/game/dice/      — DiceExpr parser, roll / advantage / disadvantage
    - src/data/types.rs   — all TOML serde structs
    - src/data/loader.rs  — load<T>, load_region, load_global_assets
    - src/game/character/ — AbilityScores, Character, Skill, Condition, progression
    - src/game/items/     — Inventory, EquipmentSlots, armor AC
    - src/game/world/     — Tile, TileGrid, Room, Region, shadowcasting FOV
        · world/map.rs    — Tile enum, TileGrid::from_str, passability/sight
        · world/room.rs   — Room::from_def, trigger_at, npc_at helpers
        · world/region.rs — Region::from_loaded, room(), entry(), exits_from()
        · world/fov.rs    — compute(origin, radius, grid) shadowcasting
        · world/mod.rs    — re-exports: compute_fov, Tile, TileGrid, Region, Room
    - src/game/story/     — WorldState flag/counter store + condition evaluator
        · story/world_state.rs — set_flag, clear_flag, flag, counter,
                                  delta_counter, evaluate(condition_str)
        · story/mod.rs         — module root
    - src/game/mod.rs     — `pub mod world;` and `pub mod story;` uncommented
    - `cargo test` — 49 tests, 0 failures

What is NOT done yet:
    - src/game/combat/    — initiative, attack rolls, action economy  ← NEXT
    - src/ui/tui/screens/ — all screen render functions
    - src/ui/tui/layout.rs, widgets/
    - game/ and data/ modules NOT wired into app.rs yet (stubs only)
    - src/game/story/quest.rs, dialog.rs, journal.rs, events.rs  (Milestone 3)

Next action for the incoming agent:
  1. `cargo test` — must pass (49 tests) before touching anything.
  2. Implement src/game/combat/ (see docs/gameplay/combat.md):
       - combat/initiative.rs — initiative order, turn queue (BTreeMap<i32, Vec<entity>>)
       - combat/attack.rs     — roll_attack, apply_damage, critical hit/miss
       - combat/action.rs     — ActionSlots (action, bonus, reaction tracking)
       - combat/combat.rs     — CombatState, active combatants, round counter
       - combat/mod.rs
  3. Uncomment `pub mod combat;` in src/game/mod.rs.
  4. Then build src/ui/tui/screens/combat.rs (TUI rendering of CombatState).

Files modified this session:
  src/game/world/map.rs (new)
  src/game/world/room.rs (new)
  src/game/world/region.rs (new)
  src/game/world/fov.rs (new)
  src/game/world/mod.rs (new)
  src/game/story/world_state.rs (new)
  src/game/story/mod.rs (new)
  src/game/mod.rs (added world + story)
  docs/tasks/current-sprint.md (this file)

Blockers: none
```

---

## Active Task

### Task: Combat module (Milestone 2, step 1)

**Files to create:**
- `src/game/combat/initiative.rs` — initiative order, turn queue
- `src/game/combat/attack.rs`     — attack rolls, damage, crits, saving throws
- `src/game/combat/action.rs`     — ActionSlots (action / bonus / reaction)
- `src/game/combat/combat.rs`     — `CombatState`, combatant list, round counter
- `src/game/combat/mod.rs`

**Done when:**
- [ ] `cargo test` passes (all existing + new combat tests)
- [ ] Initiative order is deterministic given fixed RNG seed
- [ ] `roll_attack(attacker, target, &ws)` returns hit/miss/crit
- [ ] `CombatState::next_turn()` advances through the initiative queue

**Blocked by:** `src/game/character/` (done), `src/game/dice/` (done)

**Relevant docs:**
- [../gameplay/combat.md](../gameplay/combat.md)

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
