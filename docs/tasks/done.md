# Done

> Completed tasks, newest first. Keep milestone/docs status synced using [../DOCS_MAP.md](../DOCS_MAP.md).
> Older records (M0–M13): [archive/done-m00-m13.md](archive/done-m00-m13.md)

---

## 2026-02-21 — Milestones 20-24 (complete): Hardening, Depth, Robustness, AI, RC Pipeline

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

**Test result:** 146 passed, 0 failed

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
