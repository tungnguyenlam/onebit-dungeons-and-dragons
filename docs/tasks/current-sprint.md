# Current Sprint

> **This is the first file an agent reads after AGENT.md.**  
> The `Last Session Handoff` block is the authoritative record of where work
> stopped. **You must update it before ending every session.**
> Also review [../DOCS_MAP.md](../DOCS_MAP.md) when a task changes docs, architecture, or milestone state.

---

## Last Session Handoff

```
Date:          2026-02-21
Completed:     Milestones 38, 39, and 40.

Tasks completed:
  M38 — Epic Threat Escalation
  - ✅ Assassination squads (ghostly_knight, orc_warchief, forest_goblin) ambush based on threat level
  - ✅ Ruined hubs (ruined-ironhold-mines) load when macguffin acquired
  - ✅ World events trigger journal entries when macguffins obtained
  
  M39 — v0.3.0 Release Candidate & Polish
  - ✅ Added ambient cue to sunken-city region
  - ✅ Enhanced validate_assets.sh script
  - ✅ XP curve follows standard 5e SRD
  
  M40 — Retained-Mode VFX Engine
  - ✅ Created VfxEngine in src/ui/tui/vfx.rs
  - ✅ Added GameEvent::Frame for 30 FPS updates
  - ✅ Added damage floaters, tile pulses, screen wipes primitives
  - ✅ Added DND_REDUCED_MOTION and DND_VFX_TIER environment variables

Files modified:
  - src/app/mod.rs: Enhanced epic threat escalation logic
  - src/renderer.rs: Added Frame event
  - src/ui/tui/mod.rs: Added VfxEngine integration
  - src/ui/tui/vfx.rs: New VFX engine module
  - assets/regions/sunken-city/region.toml: Added ambient
  - scripts/validate_assets.sh: Enhanced validation

Tests at close: 136 passed, 0 failed

Next for incoming agent:
  - All 40 milestones complete!
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
