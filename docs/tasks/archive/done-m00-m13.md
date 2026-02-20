# Done — Archive (M0–M13)

> Older milestone completion records. Not needed for day-to-day agent work.
> Current records are in [done.md](done.md).

---

## 2026-02-20 — Milestone 13 (complete): Release Readiness

- [x] `src/game/save/mod.rs` + `src/app.rs` — save format versioning hardening (`format_version`) with legacy compatibility fallback
- [x] `src/game/save/mod.rs` tests — added legacy save loading coverage without version field
- [x] `src/data/loader.rs` + `scripts/profile_startup.sh` — startup/load profiling smoke pass with bounded threshold check
- [x] `scripts/release_check.sh` (new) — consolidated release verification entry point (tests + content validation + profile smoke)
- [x] `docs/releases/v0.1.0-internal.md` (new) — release notes and handoff summary
- [x] `README.md` — added release-check command, support matrix, and release-notes reference
- [x] `scripts/release_check.sh` passes

## 2026-02-20 — Milestone 12 (complete): UX & Presentation Polish 2.0

- [x] `src/ui/tui/theme.rs` (new) — terminal capability-tier detection (`T0`..`T3`) and semantic theme tokens
- [x] `src/ui/tui/mod.rs` — initializes and caches terminal capability profile at renderer startup
- [x] `src/ui/tui/screens/{main_menu,world_map,combat,inventory,spellbook}.rs` — migrated key HUD/panels to shared semantic tokens and icon fallbacks
- [x] `docs/architecture/ui-layer.md` + `docs/architecture/tui-visual-system.md` — documented theme/tier implementation and verification helpers
- [x] `README.md` — documented terminal support matrix and config behavior

## 2026-02-20 — Milestone 11 (complete): NPC/Faction Simulation 2.0

- [x] Spellcaster heal archetype, reputation-driven hostility/support, multi-step emergent events
- [x] `cargo test` passes (113 tests, 0 failures)

## 2026-02-20 — Milestone 10 (complete): Content Production Pipeline

- [x] `emberpeak-summit` + `ironhold-mines` regions authored
- [x] Act 2 quest scaffolding + side quests + lore assets
- [x] Region/room/NPC/dialog templates added

## 2026-02-20 — M0–M9: Foundation Milestones

Complete. See git history for individual change records.
